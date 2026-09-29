use crate::credentials::ObsWebsocket;
use std::net::Ipv4Addr;

pub async fn connect_obs(creds: ObsWebsocket) -> Result<obws::Client, String> {
    // Check whether the address is a valid IPv4 address.
    let address = if is_valid_ipv4(creds.address.as_str()) {
        creds.address.as_str()
    } else {
        "127.0.0.1"
    };
    // Check whether the port is valid.
    let port = if creds.port > 0 { creds.port } else { 4455 };
    // Use no password when authentication is disabled.
    let password = if creds.password.is_empty() {
        None
    } else {
        Some(creds.password.as_str())
    };

    let client = obws::Client::connect(&address, port, password)
        .await
        .map_err(|error| error.to_string())?;

    Ok(client)
}

// Validate an IPv4 address.
fn is_valid_ipv4(s: &str) -> bool {
    s.parse::<Ipv4Addr>().is_ok()
}

#[derive(Default)]
pub struct ObsConnectionState {
    pub client: tokio::sync::Mutex<Option<obws::Client>>,
}

async fn check_connection(client: &obws::Client) -> Result<String, String> {
    client
        .general()
        .version()
        .await
        .map_err(|error| error.to_string())?;

    Ok("OK".to_string())
}

pub async fn handle_reconnection(creds: ObsWebsocket, client: &obws::Client) {
    if creds.auto_reconnect {
        if check_connection(client).await.is_err() {
            // TODO: reconnect the client.
        }
    }
}
