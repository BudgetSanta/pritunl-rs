use pritunl_rs::Client;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let r = Client::new();

    println!("PROFILE: {:#?}", r.query_active_profiles()?);
    println!("PROFILE: {:#?}", r.query_active_profiles()?);
    println!("PROFILE: {:#?}", r.query_active_profiles()?);

    Ok(())
}
