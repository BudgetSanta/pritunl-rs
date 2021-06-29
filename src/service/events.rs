use crate::{
    socket::{self, Response},
    Client,
};

impl Client {
    pub fn get_events(&self) -> Response {
        socket::get(self, "/events")
    }
}
