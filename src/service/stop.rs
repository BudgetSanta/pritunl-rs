use crate::{
    socket::{self, Response},
    Client,
};

impl Client {
    pub fn stop(&self, body: &str) -> Response {
        socket::post(self, "/stop", body)
    }
}
