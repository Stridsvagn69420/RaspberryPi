use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub server: Server,
    pub cyrkensia: Cyrkensia
}

#[derive(Deserialize, Serialize)]
pub struct Server {
    pub bindaddr: String,
    pub socket: bool
}

#[derive(Deserialize, Serialize)]
pub struct Cyrkensia {
    pub uuid: String,
    pub name: String,
    pub hosticon: String,
    pub htpasswd: String
}