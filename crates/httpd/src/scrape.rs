use rocket::serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Result {
    pub protocol: Vec<i32>,
    pub master_timeout: u32,
    pub server_timeout: u32,
    pub masters: Vec<SocketAddr>,
    pub filter: String,
    pub servers: Vec<Info>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Info {
    pub time: i64,
    pub address: SocketAddr,
    pub ping: f64,
    pub status: String,
    pub gamedir: String,
    pub map: String,
    pub host: String,
    pub protocol: i32,
    pub numcl: u32,
    pub maxcl: u32,
    pub dm: bool,
    pub team: bool,
    pub coop: bool,
    pub password: bool,
    pub dedicated: bool,
}
