use crate::{socket, Client};
use std::io;

impl Client {
    pub fn ping(&self) -> Result<bool, io::Error> {
        Ok(socket::get(self, "/ping")?.success)
    }
}
