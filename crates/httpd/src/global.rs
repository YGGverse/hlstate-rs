use rocket::serde::Serialize;
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Global {
    pub masters: HashSet<std::net::SocketAddr>,
}
