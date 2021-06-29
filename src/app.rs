use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Error,
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub server: String,
    pub server_id: String,
    // {
    // "disable_reconnect": true,
    // "lastMode": null,
    // "name": null,
    // "organization": "String",
    // "organization_id": String,
    // "password_mode": "otp",
    // "pre_connect_msg": null,
    // "server": "String",
    // "server_box_public_key": String,
    // "server_id": String,
    // "server_public_key": String,
    // "sync_hash": String,
    // "sync_hosts": Vec<String>
    // "sync_secret": String,
    // "sync_token": String,
    // "token": false,
    // "token_ttl": 32 or 64?, check if signed or unsigned
    // "user": String,
    // "user_id": String,
    // "wg": false
    // }
}
pub struct Client {
    pub auth_key: String,
    pub prof_path: PathBuf,
    pub client: UnixStream,
    pub profiles: Vec<Profile>,
}

impl Client {
    pub fn new() -> Self {
        let profile_path = get_profile_path().unwrap();
        let profiles = load_profiles(&profile_path);
        Self {
            auth_key: get_auth_key().unwrap(),
            prof_path: profile_path,
            client: UnixStream::connect("/var/run/pritunl.sock").unwrap(),
            profiles,
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn get_auth_key() -> Result<String, Error> {
    fs::read_to_string("/var/run/pritunl.auth")
}

fn get_profile_path() -> Result<PathBuf, ()> {
    // TODO: Support linux path "~/.config/pritunl/profiles"
    let user_dirs = directories::UserDirs::new().unwrap();
    Ok(user_dirs
        .home_dir()
        .join("Library/Application Support/pritunl/profiles"))
}

fn load_profiles(profile_path: &Path) -> Vec<Profile> {
    let profile_configs = fs::read_dir(profile_path).unwrap();
    profile_configs
        .filter_map(|x| x.ok())
        .map(|f| f.path())
        .filter(|f| match f.extension() {
            None => false,
            Some(ext) => ext == "conf",
        })
        .map(fs::read_to_string)
        .filter_map(|x| x.ok())
        .map(|x| serde_json::from_str(&x))
        .filter_map(|x| x.ok())
        .collect()
}
