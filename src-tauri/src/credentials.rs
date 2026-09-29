use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TwitchCredentials {
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct YoutubeCredentials {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ObsWebsocket {
    pub address: String,
    pub port: u16,
    pub password: String,
    #[serde(default)]
    pub auto_reconnect: bool,
    #[serde(default)]
    pub auto_reconnect_time: u16,
}

impl Default for ObsWebsocket {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".to_string(),
            port: 4455,
            password: String::new(),
            auto_reconnect: true,
            auto_reconnect_time: 30,
        }
    }
}
