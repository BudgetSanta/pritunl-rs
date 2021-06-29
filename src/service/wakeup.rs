use crate::{
    socket::{self, Response},
    Client,
};

impl Client {
    pub fn wakeup(&self, body: &str) -> Response {
        socket::post(self, "/wakeup", body)
    }
}
