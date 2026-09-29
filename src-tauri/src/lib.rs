mod chat;
mod credentials;
mod db;
mod obswebsocket;
mod youtube;

use chat::{get_twitch_chat_history, start_twitch_chat};
use credentials::*;
use db::AppDatabase;
use reqwest::Client;
use std::collections::HashMap;
use std::env::var;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_opener::OpenerExt;
use tiny_http::{Response, Server};
use url::form_urlencoded;

const TWITCH_REDIRECT_URI: &str = "http://localhost:17563/callback";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir)?;

            let db_path = app_dir.join("broadkinesis.redb");
            let database = AppDatabase::init(db_path)?;

            app.manage(database);
            app.manage(obswebsocket::ObsConnectionState::default());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_obs_websocket,
            get_obs_websocket,
            connect_obs_websocket,
            login_twitch,
            get_twitch_creds,
            validate_twitch,
            logout_twitch,
            start_twitch_chat,
            get_twitch_chat_history,
            youtube::login_youtube,
            youtube::get_youtube_creds,
            youtube::validate_youtube,
            youtube::logout_youtube,
            youtube::start_youtube_chat
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

fn save_twitch_creds(creds: TwitchCredentials, db: State<AppDatabase>) -> Result<(), String> {
    db.save_item("twitch", &creds).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_twitch_creds(db: State<AppDatabase>) -> Result<Option<TwitchCredentials>, String> {
    db.get_item("twitch").map_err(|e| e.to_string())
}

#[tauri::command]
async fn validate_twitch(db: State<'_, AppDatabase>) -> Result<bool, String> {
    let credentials: Option<TwitchCredentials> =
        db.get_item("twitch").map_err(|e| e.to_string())?;
    let Some(access_token) = credentials.and_then(|value| value.access_token) else {
        return Ok(false);
    };

    let response = Client::new()
        .get("https://id.twitch.tv/oauth2/validate")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    Ok(response.status().is_success())
}

#[tauri::command]
fn logout_twitch(db: State<AppDatabase>) -> Result<(), String> {
    db.delete_item("twitch").map_err(|e| e.to_string())
}

#[tauri::command]
fn set_obs_websocket(creds: ObsWebsocket, db: State<AppDatabase>) -> Result<(), String> {
    db.save_item("obsWebsocket", &creds)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_obs_websocket(db: State<AppDatabase>) -> Result<Option<ObsWebsocket>, String> {
    db.get_item("obsWebsocket").map_err(|e| e.to_string())
}

#[tauri::command]
async fn connect_obs_websocket(
    db: State<'_, AppDatabase>,
    connection: State<'_, obswebsocket::ObsConnectionState>,
) -> Result<(), String> {
    let result_from_db: Option<ObsWebsocket> =
        db.get_item("obsWebsocket").map_err(|e| e.to_string())?;

    let creds = result_from_db.unwrap_or_default();

    let client = obswebsocket::connect_obs(creds)
        .await
        .map_err(|error| error.to_string())?;

    let mut stored_client = connection.client.lock().await;
    *stored_client = Some(client);

    Ok(())
}

#[tauri::command]
async fn login_twitch(app: AppHandle, db: State<'_, AppDatabase>) -> Result<(), String> {
    let _ = dotenvy::dotenv();

    let client_id =
        var("TWITCH_CLIENT_ID").map_err(|_| "TWITCH_CLIENT_ID was not defined".to_string())?;
    let client_secret = var("TWITCH_CLIENT_SECRET")
        .map_err(|_| "TWITCH_CLIENT_SECRET was not defined".to_string())?;

    let auth_url = format!(
        "https://id.twitch.tv/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=chat:read+chat:edit",
        client_id, TWITCH_REDIRECT_URI
    );

    let code = tokio::task::spawn_blocking(move || {
        let server = Server::http("127.0.0.1:17563").expect("Could not start the local server");

        app.opener()
            .open_url(&auth_url, None::<&str>)
            .map_err(|error| error.to_string())?;

        for request in server.incoming_requests() {
            let url = request.url().to_string();

            if let Some(query) = url.strip_prefix("/callback?") {
                let params: HashMap<String, String> = form_urlencoded::parse(query.as_bytes())
                    .into_owned()
                    .collect();

                if let Some(error) = params.get("error") {
                    let description = params
                        .get("error_description")
                        .map(String::as_str)
                        .unwrap_or("Twitch rejected the authorization request");
                    let _ = request.respond(Response::from_string(format!(
                        "Login failed: {error} - {description}. You can close this window."
                    )));
                    return Err(format!(
                        "Twitch authorization failed: {error} - {description}"
                    ));
                }

                if let Some(code) = params.get("code") {
                    let _ = request.respond(Response::from_string(
                        "Login successful! You can close this window.",
                    ));
                    return Ok(code.to_string());
                }
            }
        }
        Err("Could not obtain the OAuth code".to_string())
    })
    .await
    .map_err(|e| e.to_string())??;

    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("code", code);
    params.insert("grant_type", "authorization_code".to_string());
    params.insert("redirect_uri", TWITCH_REDIRECT_URI.to_string());

    let res = client
        .post("https://id.twitch.tv/oauth2/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

    if let Some(access_token) = json["access_token"].as_str() {
        let creds = TwitchCredentials {
            access_token: Some(access_token.to_string()),
            refresh_token: json["refresh_token"].as_str().map(|s| s.to_string()),
        };

        save_twitch_creds(creds, db)?;

        return Ok(());
    }

    Err(json["message"]
        .as_str()
        .unwrap_or("Could not obtain the response token")
        .to_string())
}
