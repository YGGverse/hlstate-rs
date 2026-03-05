use rocket::serde::Serialize;
use std::{collections::HashSet, path::PathBuf};

#[derive(Clone, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Global {
    pub masters: HashSet<std::net::SocketAddr>,
    pub query: PathBuf,
}
