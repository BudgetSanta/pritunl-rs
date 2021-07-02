use std::{fs, io::Error, os::unix::net::UnixStream};

pub struct Client {
    pub auth_key: String,
    pub socket_path: String,
}

impl Client {
    pub fn new() -> Self {
        let socket_path = String::from("/var/run/pritunl.sock");
        Self {
            auth_key: get_auth_key().unwrap(),
            socket_path,
        }
    }

    pub fn new_socket_connection(&self) -> UnixStream {
        UnixStream::connect(&self.socket_path).expect("Couldn't create unix socket")
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn get_auth_key() -> Result<String, Error> {
    fs::read_to_string("/var/run/pritunl.auth")
}
