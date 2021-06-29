use crate::{
    socket::{self, Response},
    Client,
};

impl Client {
    pub fn restart(&self, body: &str) -> Response {
        socket::post(self, "/restart", body)
    }
}
