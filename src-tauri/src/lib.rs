mod db;
mod credentials;
use credentials::*;
use db::AppDatabase;
use tauri::Manager;
use tauri::State;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .setup(|app|{
          let app_dir = app.path().app_data_dir()?;
          std::fs::create_dir_all(&app_dir)?;

          let db_path = app_dir.join("broadkinesis.redb");
          let database = AppDatabase::init(db_path)?;

          app.manage(database);
          Ok(())
        })
                .invoke_handler(tauri::generate_handler![set_obs_websocket])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
fn save_twitch_creds(creds: TwitchCredentials, db: State<AppDatabase>) -> Result<(), String> {
    db.save_item("twitch", &creds).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_twitch_creds(db: State<AppDatabase>) -> Result<Option<TwitchCredentials>, String> {
    db.get_item("twitch").map_err(|e| e.to_string())
}

#[tauri::command]
fn set_obs_websocket(creds:ObsWebsocket, db: State<AppDatabase>) -> Result<(), String> {
  db.save_item("obsWebsocket", &creds).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_obs_websocket(db: State<AppDatabase>) -> Result<Option<ObsWebsocket>, String> {
    db.get_item("obsWebsocket").map_err(|e| e.to_string())
}