use core::time;
use pritunl_rs::Client;
use std::{
    error::Error,
    thread,
    time::{Duration, SystemTime},
    vec,
};

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
    for sys_prof in sprofiles {
        let profile = active_profiles.get(&sys_prof.id);
        let status = match profile {
            None => "Disconnected".into(),
            Some(p) => match p.timestamp {
                0 => "Connecting".into(),
                _ => format!(
                    "Up for ({})",
                    format_time(p.timestamp)
                        .to_string()
                        .split_ascii_whitespace()
                        .next()
                        .unwrap()
                ),
            },
        };
        println!("{} [{}] - {}", &sys_prof.id[..8], status, sys_prof.server,);
    }
    Ok(())
}

fn format_time(timestamp: i64) -> humantime::FormattedDuration {
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        - Duration::new(timestamp as u64, 0);
    humantime::format_duration(duration)
}
