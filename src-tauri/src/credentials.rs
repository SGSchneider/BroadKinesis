use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TwitchCredentials {
    pub client_id: String,
    pub client_secret: String,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct YoutubeCredentials {
    pub client_id: String,
    pub client_secret: String,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObsWebsocket{
  pub address: String,
  pub port: u16,
  pub password: String,
  #[serde(default)]
  pub auto_reconnect: bool,
  #[serde(default)]
  pub auto_reconnect_time: u16,
}