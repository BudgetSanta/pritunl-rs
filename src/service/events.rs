use crate::{socket, Client, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {}

impl Client {
    pub fn get_events(&self) -> Result<Vec<Event>> {
        let res = socket::get(self, "/events")?;
        println!("{}\n{}", &res.headers, &res.body);
        Ok(serde_json::from_str(&res.body)?)
    }
}
