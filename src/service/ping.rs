use crate::{
    socket::{self, Response},
    Client,
};

impl Client {
    pub fn ping(&self) -> Response {
        socket::get(self, "/ping")
    }
}
