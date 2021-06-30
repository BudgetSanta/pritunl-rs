use std::error::Error;

use crate::{socket, Client};

impl Client {
    pub fn wakeup(&self, body: &str) -> Result<String, Box<dyn Error>> {
        socket::post(self, "/wakeup", body)
    }
}
