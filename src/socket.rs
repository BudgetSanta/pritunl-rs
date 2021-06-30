use std::io::{self, Read, Write};

use crate::Client;

pub enum RequestVerb {
    Get,
    Post,
}

pub fn get(r: &Client, endpoint: &str) -> Result<String, io::Error> {
    let req = format_request(r, endpoint, RequestVerb::Get, "");
    send_request(r, req)
}

pub fn post(r: &Client, endpoint: &str, json_body: &str) -> Result<String, io::Error> {
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

fn send_request(r: &Client, req: String) -> Result<String, io::Error> {
    let mut socket = &r.client;
    socket.write_all(req.as_bytes())?;

    let mut res = String::new();
    socket.read_to_string(&mut res)?;

    Ok(res)
}
