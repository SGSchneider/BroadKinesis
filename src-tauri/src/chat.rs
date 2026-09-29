use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::var;
use tauri::{AppHandle, Emitter};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};

use crate::credentials::TwitchCredentials;
use crate::db::AppDatabase;

const CHAT_HISTORY_KEY: &str = "twitch_chat_history";
const CHAT_HISTORY_LIMIT: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: String,
    pub sender: String,
    pub text: String,
    pub color: Option<String>,
    pub badges: Vec<ChatBadge>,
    pub emotes: Vec<ChatEmote>,
    pub bits: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatBadge {
    pub name: String,
    pub version: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEmote {
    pub id: String,
    pub code: String,
}

pub fn persist_chat_message(db: &AppDatabase, chat_msg: &ChatMessage) {
    if let Ok(history) = db.get_item::<Vec<ChatMessage>>(CHAT_HISTORY_KEY) {
        let mut history = history.unwrap_or_default();
        history.push(chat_msg.clone());
        if history.len() > CHAT_HISTORY_LIMIT {
            let excess = history.len() - CHAT_HISTORY_LIMIT;
            history.drain(..excess);
        }
        let _ = db.save_item(CHAT_HISTORY_KEY, &history);
    }
}

#[tauri::command]
pub fn get_twitch_chat_history(db: tauri::State<AppDatabase>) -> Result<Vec<ChatMessage>, String> {
    db.get_item(CHAT_HISTORY_KEY)
        .map_err(|error| error.to_string())
        .map(|history: Option<Vec<ChatMessage>>| history.unwrap_or_default())
}

#[tauri::command]
pub async fn start_twitch_chat(
    channel: String,
    app: AppHandle,
    db: tauri::State<'_, AppDatabase>,
) -> Result<(), String> {
    // Load credentials from the database.
    let creds: Option<TwitchCredentials> = db.get_item("twitch").map_err(|e| e.to_string())?;

    let access_token = match creds {
        Some(c) => c.access_token.ok_or("Token not found")?,
        None => return Err("Twitch is not connected".into()),
    };

    let _ = dotenvy::dotenv();
    let client_id =
        var("TWITCH_CLIENT_ID").map_err(|_| "TWITCH_CLIENT_ID is not defined".to_string())?;
    let http_client = Client::new();

    let validation = http_client
        .get("https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("OAuth {access_token}"))
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if !validation.status().is_success() {
        return Err("The Twitch token is invalid or expired".to_string());
    }

    let validation_json: serde_json::Value =
        validation.json().await.map_err(|error| error.to_string())?;
    let login = validation_json["login"]
        .as_str()
        .ok_or("Could not identify the Twitch user")?;
    let broadcaster_id = validation_json["user_id"]
        .as_str()
        .ok_or("Could not identify the Twitch channel")?;

    let channel = channel.trim_start_matches('#').to_lowercase();
    if channel.is_empty() {
        return Err("Twitch channel was not provided".to_string());
    }

    let badge_images =
        load_badge_images(&http_client, &client_id, &access_token, broadcaster_id).await;

    // Configure the chat client.
    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        login.to_owned(),
        Some(access_token),
    ));

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
    let history_db = db.inner().clone();

    // Join the requested channel.
    client.join(channel).map_err(|e| e.to_string())?;

    // Listen for messages in the background.
    tokio::spawn(async move {
        let _client = client;

        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    let chat_msg = ChatMessage {
                        id: msg.message_id,
                        sender: msg.sender.name,
                        text: msg.message_text,
                        color: msg
                            .name_color
                            .map(|color| format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b)),
                        badges: msg
                            .badges
                            .iter()
                            .map(|badge| ChatBadge {
                                name: badge.name.clone(),
                                version: badge.version.clone(),
                                image_url: badge_images
                                    .get(&format!("{}:{}", badge.name, badge.version))
                                    .cloned(),
                            })
                            .collect(),
                        emotes: msg
                            .emotes
                            .iter()
                            .map(|emote| ChatEmote {
                                id: emote.id.clone(),
                                code: emote.code.clone(),
                            })
                            .collect(),
                        bits: msg.bits,
                    };

                    persist_chat_message(&history_db, &chat_msg);

                    // Emit the event for the frontend.
                    let _ = app.emit("chat-message", chat_msg);
                }
                ServerMessage::Notice(notice) => {
                    let _ = app.emit("chat-error", notice.message_text);
                }
                ServerMessage::Reconnect(_) => {
                    let _ = app.emit("chat-error", "The Twitch connection was restarted");
                }
                _ => {} // Ignore other server events for now.
            }
        }

        let _ = app.emit("chat-error", "The Twitch chat connection was closed");
    });

    Ok(())
}

async fn load_badge_images(
    client: &Client,
    client_id: &str,
    access_token: &str,
    broadcaster_id: &str,
) -> HashMap<String, String> {
    let endpoints = [
        "https://api.twitch.tv/helix/chat/badges/global".to_string(),
        format!("https://api.twitch.tv/helix/chat/badges?broadcaster_id={broadcaster_id}"),
    ];
    let mut images = HashMap::new();

    for endpoint in endpoints {
        let response = match client
            .get(endpoint)
            .header("Client-Id", client_id)
            .bearer_auth(access_token)
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => response,
            _ => continue,
        };

        let payload: serde_json::Value = match response.json().await {
            Ok(payload) => payload,
            Err(_) => continue,
        };

        let Some(sets) = payload["data"].as_array() else {
            continue;
        };

        for badge_set in sets {
            let Some(set_id) = badge_set["set_id"].as_str() else {
                continue;
            };
            let Some(versions) = badge_set["versions"].as_array() else {
                continue;
            };

            for version in versions {
                if let (Some(version_id), Some(image_url)) =
                    (version["id"].as_str(), version["image_url_1x"].as_str())
                {
                    images.insert(format!("{set_id}:{version_id}"), image_url.to_string());
                }
            }
        }
    }

    images
}
