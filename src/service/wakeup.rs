use std::io;

use crate::{socket, Client};

impl Client {
    pub fn wakeup(&self, body: &str) -> Result<String, io::Error> {
        socket::post(self, "/wakeup", body)
    }
}
