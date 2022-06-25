use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
/// Root Config
/// 
/// Structure for the base config, either the developing or production one.
/// If a path is required, you should always use absolute paths, although relative ones might be possible too.
pub struct Config {
    pub server: Server,
    pub cyrkensia: Cyrkensia,
    pub shigure: Shigure
}

/// Server Config
/// 
/// The config for the main server: Bind address, Unix Socket or not, WWW-Path.
#[derive(Deserialize, Serialize)]
pub struct Server {
    /// Bind address
    /// 
    /// The bind address with port for the server, e.g. `127.0.0.1:80`, `0.0.0.0:9000` or `/var/run/raspberrypi.socket` (Unix only)
    pub bindaddr: String,

    /// Webfiles
    /// 
    /// Path to the compiled Svelte frontend and server assets
    pub path: String
}

/// Cyrkensia Config
/// 
/// Configuration for the Cyrkensia sub-server implementation
#[derive(Deserialize, Serialize)]
pub struct Cyrkensia {
    /// Cyrkensia Album Storage
    /// 
    /// Location where all of your albums are stored in for Cyrkensia.
    pub path: String,

    /// UUIDv4
    /// 
    /// Version UUID for your Cyrkensia server.
    pub uuid: String,

    /// Server Name
    /// 
    /// The name of your Cyrkensia server.
    pub name: String,

    /// Hosticon
    /// 
    /// The [Rich Presence Key](https://github.com/Stridsvagn69420/Cyrkensia/wiki/RPC-Assets) for the image that should represent your server.
    pub hosticon: String,

    /// Htpasswd user database
    /// 
    /// Path to your .htpasswd user database
    pub htpasswd: String
}

#[derive(Deserialize, Serialize)]
pub struct Shigure {
    pub path: String
}

pub fn read_config(path: impl AsRef<Path>) -> Result<Config, &'static str> {
    match fs::read_to_string(path) {
        Ok(data) => {
            match serde_json::from_str(data.as_str()) {
                Err(_) => Err("Error parsing config file! Config file isn't valid JSON!"),
                Ok(cfg) => Ok(cfg)
            }
        },
        Err(_) => Err("Could not or read config file! It either doesn't exist or you don't have permission to read from it!")
    }
}