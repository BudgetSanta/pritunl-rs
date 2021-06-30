use std::error::Error;

use crate::{socket, Client};

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
    pub fn get_system_profile(&self) -> Result<String, Box<dyn Error>> {
        socket::get(self, "/sprofile")
    }

    //pub fn put_system_profile(&self) -> Result<String, Box<dyn Error>>  {
    //    socket::put(self, "/sprofile")
    //}

    //pub fn delete_system_profile(&self) -> Result<String, Box<dyn Error>>  {
    //    socket::delete(self, "/sprofile")
    //}

    pub fn get_system_profile_log(&self, id: &str) -> Result<String, Box<dyn Error>> {
        socket::get(self, &format!("/sprofile/{}/log", id))
    }

    //pub fn delete_system_profile_log(&self, id: &str) -> Result<String, Box<dyn Error>>  {
    //    socket::delete(self, &format!("/sprofile/{}/log", id))
    //}
}
