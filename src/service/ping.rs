use std::io;

use crate::{socket, Client};

impl Client {
    pub fn ping(&self) -> Result<String, io::Error> {
        socket::get(self, "/ping")
    }
}
