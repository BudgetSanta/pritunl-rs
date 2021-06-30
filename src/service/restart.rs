use std::io;

use crate::{socket, Client};

impl Client {
    pub fn restart(&self, body: &str) -> Result<String, io::Error> {
        socket::post(self, "/restart", body)
    }
}
