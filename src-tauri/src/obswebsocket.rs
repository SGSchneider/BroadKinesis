use crate::credentials::ObsWebsocket;
use std::net::Ipv4Addr;


pub async fn connect_obs(creds : ObsWebsocket) -> obws::Result<obws::Client>{
  //Checks if Address is a valid ipv4
  let address = if is_valid_ipv4(creds.address.as_str()){
    creds.address.as_str()
  } else {
    "127.0.0.1"
  };
  //Checks if port is valid
  let port = if creds.port > 0 {
    creds.port
  } else {
    4455
  };
  //Checks if there is a password, if not, sets to None
  let password = if creds.password.is_empty() {
    None
  } else {
    Some(creds.password.as_str())
  };
  

  let client = obws::Client::connect(&address, port, password,).await;

  return client;
}


//Validate ipv4
fn is_valid_ipv4(s: &str) -> bool {
    s.parse::<Ipv4Addr>().is_ok()
}

#[derive(Default)]
pub struct ObsConnectionState {
    pub client: tokio::sync::Mutex<Option<obws::Client>>
}