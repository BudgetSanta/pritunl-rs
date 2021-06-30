use std::error::Error;

use crate::{socket, Client};

impl Client {
    pub fn ping(&self) -> Result<String, Box<dyn Error>> {
        socket::get(self, "/ping")
    }
}
