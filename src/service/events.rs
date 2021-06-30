use std::io;

use crate::{socket, Client};

impl Client {
    pub fn get_events(&self) -> Result<String, io::Error> {
        socket::get(self, "/events")
    }
}
