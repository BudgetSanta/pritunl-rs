use crate::{socket, Client, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub status: bool,
}

impl Client {
    pub fn get_status(&self) -> Result<Status> {
        let res = socket::get(self, "/status")?;
        Ok(serde_json::from_str::<Status>(&res.body)?)
    }
}
