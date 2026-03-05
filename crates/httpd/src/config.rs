use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub description: Option<String>,
    pub format_time: String,
    pub host: IpAddr,
    pub port: u16,
    pub debug: bool,
}
