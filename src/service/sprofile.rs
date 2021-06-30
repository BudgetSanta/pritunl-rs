use crate::{socket, Client, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Sprofile {
    pub id: String,
    pub name: String,
    pub state: bool,
    pub wg: bool,
    pub last_mode: String,
    pub organization_id: String,
    pub organization: String,
    pub server_id: String,
    pub server: String,
    pub user_id: String,
    pub user: String,
    pub pre_connect_msg: String,
    pub password_mode: String,
    pub token: bool,
    pub token_ttl: i32,
    pub disable_reconnect: bool,
    pub sync_hosts: Vec<String>,
    pub sync_hash: String,
    pub sync_secret: String,
    pub sync_token: String,
    pub server_public_key: Vec<String>,
    pub server_box_public_key: String,
    pub ovpn_data: String,
}

impl Client {
    pub fn get_system_profiles(&self) -> Result<Vec<Sprofile>> {
        let res = socket::get(self, "/sprofile")?;
        Ok(serde_json::from_str::<Vec<Sprofile>>(&res.body)?)
    }

    //pub fn put_system_profile(&self) -> Result<String, io::Error>  {
    //    socket::put(self, "/sprofile")
    //}

    //pub fn delete_system_profile(&self) -> Result<String, io::Error>  {
    //    socket::delete(self, "/sprofile")
    //}

    //pub fn get_system_profile_log(&self, id: &str) -> Result<String, io::Error> {
    //    socket::get(self, &format!("/sprofile/{}/log", id))
    //}

    //pub fn delete_system_profile_log(&self, id: &str) -> Result<String, io::Error>  {
    //    socket::delete(self, &format!("/sprofile/{}/log", id))
    //}
}
