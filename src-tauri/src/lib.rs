use std::env;
use std::fs;
use std::path::{PathBuf};
use serde_json::{json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub twitch: TwitchConfig,
    pub obs: ObsConfig,
    pub youtube: YoutubeConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwitchConfig {
    pub username: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObsConfig {
    pub address: String,
    pub port: u16,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YoutubeConfig {
    pub api_key: String,
    pub channel_id: String,
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>>{
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

  Ok(())
}

pub fn write_config_file(config: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {

  let path = get_config_path()?;
  
  let config_string = serde_json::to_string_pretty(&config)?;

  fs::write(&path, config_string)?;

  Ok(())
}

pub fn create_config_file() -> Result<(), Box<dyn std::error::Error>> {
  
  let config = json!({
    "twitch":{
      "username": "",
      "client_id": "",
      "client_secret": ""
    },
    "obs":{
      "address": "localhost",
      "port": 4455,
      "password": ""
    },
    "youtube":{
      "api_key": "",
      "channel_id": ""
    }
  });
  
  write_config_file(config)?;

  Ok(())
}

pub fn read_config_file() -> Result<AppConfig, Box<dyn std::error::Error>>{
    let path = get_config_path()?;

    if !path.exists() {
        create_config_file()?;
    }

    let content = fs::read_to_string(&path)?;
    let config: AppConfig = serde_json::from_str(&content)?;

    Ok(config)
}

pub fn get_config_path() -> Result<PathBuf, String> {
  let mut exe_path = std::env::current_exe().map_err(|e| e.to_string())?;// Get the path of the current executable
  exe_path.pop(); // Remove the executable name
  exe_path.push("config.json"); // Add the config file name
  Ok(exe_path)// Return the full path to the config file
}