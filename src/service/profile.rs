use crate::{socket, Client, Result};
use serde::{Deserialize, Serialize};
use std::io;

// type Route struct {
// NextHop    string `json:"next_hop"`
// Network    string `json:"network"`
// Metric     int    `json:"metric"`
// NetGateway bool   `json:"net_gateway"`
// }
//
// type Profile struct {
// Id           string   `json:"id"`
// Mode         string   `json:"mode"`
// Iface        string   `json:"iface"`
// Tuniface     string   `json:"tun_iface"`
// Routes       []*Route `json:"routes'"`
// Routes6      []*Route `json:"routes6'"`
// Reconnect    bool     `json:"reconnect"`
// Status       string   `json:"status"`
// Timestamp    int64    `json:"timestamp"`
// GatewayAddr  string   `json:"gateway_addr"`
// GatewayAddr6 string   `json:"gateway_addr6"`
// ServerAddr   string   `json:"server_addr"`
// ClientAddr   string   `json:"client_addr"`
// MacAddr      string   `json:"mac_addr"`
// MacAddrs     []string `json:"mac_addrs"`
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    id: String,
    mode: String,
    iface: String,
    tun_iface: String,
    //routes: Vec<Routes>,
    //routes6: Vec<Routes>,
    reconnect: bool,
    status: String,
    timestamp: i64,
    gateway_addr: String,
    gateway_addr6: String,
    server_addr: String,
    client_addr: String,
    mac_addr: String,
    mac_addrs: Vec<String>,
}

impl Client {
    pub fn get_profile(&self) -> Result<Profile> {
        let res = socket::get(self, "/profile")?;
        Ok(serde_json::from_str::<Profile>(&res.body)?)
    }

    pub fn post_profile(&self, body: &str) -> std::result::Result<String, io::Error> {
        socket::post(self, "/profile", body)
    }

    //pub fn delete_profile(&self) -> Result<String, Box<dyn Error>>  {
    //    proxy::delete(self, "/profile")
    //}
}
