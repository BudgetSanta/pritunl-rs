pub mod app;
pub mod service;

pub use app::Client;
pub use error::Result;

pub mod error;
mod socket;
