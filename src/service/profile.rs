use std::collections::HashMap;

use crate::{socket, Client, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub mode: String,
    pub iface: String,
    pub tun_iface: String,
    //routes: Vec<Routes>,
    //routes6: Vec<Routes>,
    pub reconnect: bool,
    pub status: String,
    pub timestamp: i64,
    pub gateway_addr: String,
    pub gateway_addr6: String,
    pub server_addr: String,
    pub client_addr: String,
    pub mac_addr: String,
    //mac_addrs: Vec<String>, // TODO: Possible null value, need to transform into empty vector before deserialisation
}

#[derive(Debug, Deserialize, Serialize)]
struct ProfilePost<'p> {
    id: &'p str,
    mode: &'p str,
    password: &'p str,
}

impl Client {
    // Get currently active profiles
    pub fn query_active_profiles(&self) -> Result<HashMap<String, Profile>> {
        let res = socket::get(self, "/profile")?;
        Ok(serde_json::from_str::<HashMap<String, Profile>>(&res.body)?)
    }

    pub fn connect_profile(&self, profile_id: &str, password: &str) -> Result<bool> {
        let body = serde_json::to_string(&ProfilePost {
            id: profile_id,
            mode: "ovpn", // TODO: Support other modes
            password,
        })?;

        Ok(socket::post(self, "/profile", &body)?.success)
    }

    pub fn disconnect_profile(&self, profile_id: &str) -> Result<bool> {
        let body = format!("{{\"id\": \"{}\"}}", profile_id);
        Ok(socket::delete(self, "/profile", &body)?.success)
    }
}
