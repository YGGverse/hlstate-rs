use rocket::serde::Deserialize;
use std::{
    collections::HashSet,
    net::{IpAddr, SocketAddr},
};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub debug: bool,
    pub description: Option<String>,
    pub format_time: String,
    pub host: IpAddr,
    pub masters: HashSet<SocketAddr>,
    pub port: u16,
    pub title: String,
}
