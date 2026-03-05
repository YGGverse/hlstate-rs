use rocket::serde::Serialize;
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Global {
    pub format_time: String,
    pub masters: HashSet<std::net::SocketAddr>,
}
