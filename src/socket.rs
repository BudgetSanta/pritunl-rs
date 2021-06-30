use crate::Client;
use std::io::{self, Read, Write};

pub enum RequestVerb {
    Get,
    Post,
}

pub struct Response {
    pub success: bool,
    pub headers: String,
    pub body: String,
}

pub fn get(r: &Client, endpoint: &str) -> Result<Response, io::Error> {
    let req = format_request(r, endpoint, RequestVerb::Get, "");

    let req = send_request(r, req)?;
    // TODO: Read in request buff and split it yourself
    let mut parts = req.split("\r\n\r\n");
    let headers = parts.next().expect("some headers").to_string();
    let body = parts.next().expect("some body").to_string();
    let success = headers[..].contains("200 OK");
    Ok(Response {
        headers,
        body,
        success,
    })
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
