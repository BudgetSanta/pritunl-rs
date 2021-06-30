use std::io;

use crate::{socket, Client};

impl Client {
    pub fn get_status(&self) -> Result<String, io::Error> {
        socket::get(self, "/status")
    }
}
