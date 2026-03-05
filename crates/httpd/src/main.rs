#[macro_use]
extern crate rocket;

mod argument;
mod config;
mod global;
mod meta;

use chrono::{DateTime, Utc};
use global::Global;
use meta::Meta;
use rocket::{State, http::Status, serde::Serialize};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index(meta: &State<Meta>, global: &State<Global>) -> Result<Template, Status> {
    #[derive(Serialize)]
    #[serde(crate = "rocket::serde")]
    struct Server {
        name: String,
    }
    let servers: Vec<Server> = Vec::new();
    Ok(Template::render(
        "index",
        context! {
            title: &meta.title,
            servers: servers,
        },
    ))
}

#[launch]
fn rocket() -> _ {
    use clap::Parser;
    let argument = argument::Argument::parse();
    let config: config::Config =
        toml::from_str(&std::fs::read_to_string(argument.config).unwrap()).unwrap();
    rocket::build()
        .attach(Template::fairing())
        .configure(rocket::Config {
            port: config.port,
            address: config.host,
            ..if config.debug {
                rocket::Config::debug_default()
            } else {
                rocket::Config::release_default()
            }
        })
        .manage(Global {
            format_time: config.format_time,
        })
        .manage(Meta {
            description: config.description,
            title: config.title,
            version: env!("CARGO_PKG_VERSION").into(),
        })
        .mount("/", routes![index])
}

const S: &str = " • ";

fn time(timestamp: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap()
}
