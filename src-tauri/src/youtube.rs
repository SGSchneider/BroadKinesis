use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::env::var;
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_opener::OpenerExt;
use tiny_http::{Response, Server};
use url::form_urlencoded;

use crate::chat::{persist_chat_message, ChatBadge, ChatEmote, ChatMessage};
use crate::credentials::YoutubeCredentials;
use crate::db::AppDatabase;

const YOUTUBE_REDIRECT_URI: &str = "http://localhost:17564/callback";
const YOUTUBE_SCOPE: &str = "https://www.googleapis.com/auth/youtube.readonly";

#[tauri::command]
pub fn get_youtube_creds(db: State<AppDatabase>) -> Result<Option<YoutubeCredentials>, String> {
    db.get_item("youtube").map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn login_youtube(app: AppHandle, db: State<'_, AppDatabase>) -> Result<(), String> {
    let _ = dotenvy::dotenv();
    let client_id =
        var("YOUTUBE_CLIENT_ID").map_err(|_| "YOUTUBE_CLIENT_ID is not defined".to_string())?;
    let client_secret = var("YOUTUBE_CLIENT_SECRET")
        .map_err(|_| "YOUTUBE_CLIENT_SECRET is not defined".to_string())?;

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?{}",
        form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", &client_id)
            .append_pair("redirect_uri", YOUTUBE_REDIRECT_URI)
            .append_pair("response_type", "code")
            .append_pair("scope", YOUTUBE_SCOPE)
            .append_pair("access_type", "offline")
            .append_pair("prompt", "consent")
            .finish()
    );

    let code = tokio::task::spawn_blocking(move || {
        let server = Server::http("127.0.0.1:17564")
            .map_err(|error| format!("Could not start the OAuth callback: {error}"))?;

        app.opener()
            .open_url(&auth_url, None::<&str>)
            .map_err(|error| error.to_string())?;

        for request in server.incoming_requests() {
            let url = request.url().to_string();
            let Some(query) = url.strip_prefix("/callback?") else {
                continue;
            };
            let params: HashMap<String, String> = form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect();

            if let Some(error) = params.get("error") {
                let description = params
                    .get("error_description")
                    .map(String::as_str)
                    .unwrap_or("Google rejected the authorization request");
                let _ = request.respond(Response::from_string(format!(
                    "Login failed: {error} - {description}. You can close this window."
                )));
                return Err(format!(
                    "YouTube authorization failed: {error} - {description}"
                ));
            }

            if let Some(code) = params.get("code") {
                let _ = request.respond(Response::from_string(
                    "Login successful! You can close this window.",
                ));
                return Ok(code.to_string());
            }
        }

        Err("Could not obtain the YouTube OAuth code".to_string())
    })
    .await
    .map_err(|error| error.to_string())??;

    let response = Client::new()
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", YOUTUBE_REDIRECT_URI),
        ])
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let payload: Value = response.json().await.map_err(|error| error.to_string())?;
    let Some(access_token) = payload["access_token"].as_str() else {
        return Err(payload["error_description"]
            .as_str()
            .or_else(|| payload["error"].as_str())
            .unwrap_or("Could not obtain the YouTube token")
            .to_string());
    };

    db.save_item(
        "youtube",
        &YoutubeCredentials {
            api_key: None,
            access_token: Some(access_token.to_string()),
            refresh_token: payload["refresh_token"].as_str().map(str::to_string),
        },
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn validate_youtube(db: State<'_, AppDatabase>) -> Result<bool, String> {
    let credentials: Option<YoutubeCredentials> =
        db.get_item("youtube").map_err(|error| error.to_string())?;
    let Some(token) = credentials.and_then(|value| value.access_token) else {
        return Ok(false);
    };

    let response = Client::new()
        .get("https://www.googleapis.com/youtube/v3/channels")
        .bearer_auth(token)
        .query(&[("part", "id"), ("mine", "true")])
        .send()
        .await
        .map_err(|error| error.to_string())?;

    Ok(response.status().is_success())
}

#[tauri::command]
pub fn logout_youtube(db: State<AppDatabase>) -> Result<(), String> {
    db.delete_item("youtube").map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn start_youtube_chat(app: AppHandle, db: State<'_, AppDatabase>) -> Result<(), String> {
    let credentials: Option<YoutubeCredentials> =
        db.get_item("youtube").map_err(|error| error.to_string())?;
    let Some(token) = credentials.and_then(|value| value.access_token) else {
        return Ok(());
    };

    let client = Client::new();
    let history_db = db.inner().clone();

    tokio::spawn(async move {
        let mut live_chat_id: Option<String> = None;
        let mut page_token: Option<String> = None;

        loop {
            if live_chat_id.is_none() {
                match find_live_chat(&client, &token).await {
                    Ok(Some((lifecycle, chat_id))) if lifecycle == "live" => {
                        live_chat_id = Some(chat_id);
                        page_token = None;
                    }
                    Ok(Some(_)) | Ok(None) => {
                        tokio::time::sleep(Duration::from_secs(15)).await;
                        continue;
                    }
                    Err(error) => {
                        let _ = app.emit("chat-error", error);
                        tokio::time::sleep(Duration::from_secs(30)).await;
                        continue;
                    }
                }
            }

            let Some(chat_id) = live_chat_id.as_deref() else {
                continue;
            };
            let mut request = client
                .get("https://www.googleapis.com/youtube/v3/liveChat/messages")
                .bearer_auth(&token)
                .query(&[
                    ("part", "snippet,authorDetails"),
                    ("liveChatId", chat_id),
                    ("maxResults", "200"),
                ]);
            if let Some(page_token) = &page_token {
                request = request.query(&[("pageToken", page_token.as_str())]);
            }

            let response = match request.send().await {
                Ok(response) if response.status().is_success() => response,
                Ok(response) => {
                    let _ = app.emit(
                        "chat-error",
                        format!("YouTube chat returned HTTP {}", response.status()),
                    );
                    break;
                }
                Err(error) => {
                    let _ = app.emit("chat-error", format!("YouTube chat error: {error}"));
                    break;
                }
            };

            let payload: Value = match response.json().await {
                Ok(payload) => payload,
                Err(error) => {
                    let _ = app.emit("chat-error", format!("Invalid YouTube response: {error}"));
                    break;
                }
            };

            if let Some(items) = payload["items"].as_array() {
                for item in items {
                    let Some(message) = youtube_message(item) else {
                        continue;
                    };
                    persist_chat_message(&history_db, &message);
                    let _ = app.emit("chat-message", message);
                }
            }

            page_token = payload["nextPageToken"].as_str().map(str::to_string);
            let delay = payload["pollingIntervalMillis"].as_u64().unwrap_or(5_000);
            tokio::time::sleep(Duration::from_millis(delay)).await;
        }
    });

    Ok(())
}

async fn find_live_chat(client: &Client, token: &str) -> Result<Option<(String, String)>, String> {
    let response = client
        .get("https://www.googleapis.com/youtube/v3/liveBroadcasts")
        .bearer_auth(token)
        .query(&[("part", "snippet,status"), ("mine", "true")])
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "response contained no details".to_string());
        return Err(format!(
            "YouTube liveBroadcasts returned HTTP {status}: {body}"
        ));
    }

    let payload: Value = response.json().await.map_err(|error| error.to_string())?;
    Ok(payload["items"].as_array().and_then(|items| {
        items.iter().find_map(|item| {
            let lifecycle = item["status"]["lifeCycleStatus"].as_str()?;
            if lifecycle != "live" && lifecycle != "upcoming" {
                return None;
            }

            Some((
                lifecycle.to_string(),
                item["snippet"]["liveChatId"].as_str()?.to_string(),
            ))
        })
    }))
}

fn youtube_message(item: &Value) -> Option<ChatMessage> {
    let id = item["id"].as_str()?.to_string();
    let snippet = &item["snippet"];
    let author = &item["authorDetails"];
    let mut badges = Vec::new();

    if author["isChatOwner"].as_bool() == Some(true) {
        badges.push(ChatBadge {
            name: "Dono".to_string(),
            version: "1".to_string(),
            image_url: None,
        });
    } else if author["isChatModerator"].as_bool() == Some(true) {
        badges.push(ChatBadge {
            name: "Moderador".to_string(),
            version: "1".to_string(),
            image_url: None,
        });
    } else if author["isChatSponsor"].as_bool() == Some(true) {
        badges.push(ChatBadge {
            name: "Membro".to_string(),
            version: "1".to_string(),
            image_url: None,
        });
    }

    Some(ChatMessage {
        id,
        sender: author["displayName"]
            .as_str()
            .unwrap_or("YouTube")
            .to_string(),
        text: snippet["displayMessage"].as_str()?.to_string(),
        color: None,
        badges,
        emotes: Vec::<ChatEmote>::new(),
        bits: None,
    })
}
