use crate::{
    socket::{self, Response},
    Client,
};

//struct Sprofile {
//    id: String,
//    name: String,
//    server: String,
//    user_id: String,
//    user: String,
//    password_mode: String,
//    password: String,
//    profile: profile,
//}

impl Client {
    pub fn get_system_profile(&self) -> Response {
        socket::get(self, "/sprofile")
    }

    //pub fn put_system_profile(&self) -> Response {
    //    socket::put(self, "/sprofile")
    //}

    //pub fn delete_system_profile(&self) -> Response {
    //    socket::delete(self, "/sprofile")
    //}

    pub fn get_system_profile_log(&self, id: &str) -> Response {
        socket::get(self, &format!("/sprofile/{}/log", id))
    }

    pub fn delete_system_profile_log(&self, id: &str) -> Response {
        socket::get(self, &format!("/sprofile/{}/log", id))
    }
}