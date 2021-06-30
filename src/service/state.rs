use std::io;

use crate::{socket, Client};

impl Client {
    pub fn get_state(&self) -> Result<String, io::Error> {
        socket::get(self, "/state")
    }
}
