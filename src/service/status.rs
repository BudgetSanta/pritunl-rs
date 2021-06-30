use std::error::Error;

use crate::{socket, Client};

impl Client {
    pub fn get_status(&self) -> Result<String, Box<dyn Error>> {
        socket::get(self, "/status")
    }
}
