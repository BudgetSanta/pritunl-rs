use std::error::Error;

use crate::{socket, Client};

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

struct Profile {
    id: String,
    mode: String,
    iface: String,
    tun_iface: String,
    //routes: Vec<Routes>,
    //routes6: Vec<Routes>,
    reconect: bool,
    status: String,
    timestamp: i64,
    gateway_addr: String,
    server_addr: String,
    client_addr: String,
    mac_addr: String,
    mac_addrs: Vec<String>,
}

impl Client {
    pub fn get_profile(&self) -> Result<String, Box<dyn Error>> {
        socket::get(self, "/profile")
    }

    pub fn post_profile(&self, body: &str) -> Result<String, Box<dyn Error>> {
        socket::post(self, "/profile", body)
    }

    //pub fn delete_profile(&self) -> Result<String, Box<dyn Error>>  {
    //    proxy::delete(self, "/profile")
    //}
}
