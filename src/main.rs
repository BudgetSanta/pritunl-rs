use core::time;
use pritunl_rs::Client;
use std::{error::Error, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let r = Client::new();

    let delay = time::Duration::from_secs(2);

    loop {
        print_server_status(&r)?;
        println!("sleeping...");
        thread::sleep(delay);
    }

    Ok(())
}

fn print_server_status(r: &Client) -> Result<(), Box<dyn Error>> {
    let mut sprofiles = r.get_system_profiles()?;
    let active_profiles = r.query_active_profiles()?;
    sprofiles.sort_by(|a, b| a.server.cmp(&b.server));
    for p in sprofiles {
        let status = match active_profiles.contains_key(&p.id) {
            true => match active_profiles.get(&p.id).unwrap().timestamp {
                0 => "Connecting  ",
                _ => "Connected   ",
            },
            false => "Disconnected",
        };
        println!("{} [{}] - {}", &p.id[..8], status, p.server);
    }
    Ok(())
}
