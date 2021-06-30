use pritunl_rs::Client;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let r = Client::new();

    println!("{}", r.ping().unwrap());

    Ok(())
}
