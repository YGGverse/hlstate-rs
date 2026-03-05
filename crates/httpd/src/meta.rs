use rocket::serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Meta {
    pub title: String,
    pub version: String,
}
