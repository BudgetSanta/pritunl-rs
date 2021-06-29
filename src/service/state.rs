use crate::{
    socket::{self, Response},
    Client,
};

impl Client {
    pub fn get_state(&self) -> Response {
        socket::get(self, "/state")
    }
}
