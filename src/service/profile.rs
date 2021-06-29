use crate::{
    socket::{self, Response},
    Client,
};

//struct Profile {
//    id: String,
//    mode: String,
//    status: String,
//    timestamp: i64,
//    serveraddr: String,
//    clientaddr: String,
//}

impl Client {
    pub fn get_profile(&self) -> Response {
        socket::get(self, "/profile")
    }

    pub fn post_profile(&self, body: &str) -> Response {
        socket::post(self, "/profile", body)
    }

    //pub fn delete_profile(&self) -> Response {
    //    proxy::delete(self, "/profile")
    //}
}
