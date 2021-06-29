use crate::{
    socket::{self, Response},
    Client,
};

impl Client {
    pub fn get_status(&self) -> Response {
        socket::get(self, "/status")
    }
}
