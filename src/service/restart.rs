use std::error::Error;

use crate::{socket, Client};

impl Client {
    pub fn restart(&self, body: &str) -> Result<String, Box<dyn Error>> {
        socket::post(self, "/restart", body)
    }
}
