use std::io;

use crate::{socket, Client};

impl Client {
    pub fn stop(&self, body: &str) -> Result<String, io::Error> {
        socket::post(self, "/stop", body)
    }
}
