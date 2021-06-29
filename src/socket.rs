use std::{
    fmt,
    io::{Read, Write},
};

use crate::Client;

pub enum RequestVerb {
    Get,
    Post,
}

pub struct Response {
    pub string_buffer: String,
}

impl Response {
    fn new() -> Self {
        Response {
            string_buffer: String::new(),
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Fix this mess
        write!(f, "{}", self.string_buffer.as_str().trim())
    }
}

pub fn get(r: &Client, endpoint: &str) -> Response {
    let req = format_request(r, endpoint, RequestVerb::Get, "");
    send_request(r, req)
}

pub fn post(r: &Client, endpoint: &str, json_body: &str) -> Response {
    let req = format_request(r, endpoint, RequestVerb::Post, json_body);
    send_request(r, req)
}

fn format_request(r: &Client, endpoint: &str, method: RequestVerb, body: &str) -> String {
    match method {
        RequestVerb::Get => format!(
            "GET {} HTTP/1.0\r\nUser-Agent: pritunl\r\nAuth-Key: {}\r\n\r\n",
            endpoint, r.auth_key
        ),
        RequestVerb::Post => {
            format!("POST {} HTTP/1.0\r\nUser-Agent: pritunl\r\nAuth-Key: {}\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}\r\n\r\n",
                endpoint,
                r.auth_key,
                body.len(),
                body
            )
        }
    }
}

fn send_request(r: &Client, req: String) -> Response {
    let mut socket = &r.client;
    socket.write_all(req.as_bytes()).unwrap();

    let mut res = Response::new();
    socket.read_to_string(&mut res.string_buffer).unwrap();

    res
}
