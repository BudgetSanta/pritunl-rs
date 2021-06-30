use pritunl_rs::Client;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let r = Client::new();

    let profiles = r.get_system_profiles()?;
    for p in profiles {
        println!("{}", p.server)
    }

    Ok(())
}
