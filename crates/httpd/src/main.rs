#[macro_use]
extern crate rocket;

mod argument;
mod config;
mod global;
mod meta;
mod scrape;

use global::Global;
use meta::Meta;
use rocket::{State, http::Status};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index(meta: &State<Meta>, global: &State<Global>) -> Result<Template, Status> {
    // @TODO: requires library impl
    // https://github.com/FWGS/xash3d-master/issues/4
    let scrape = std::process::Command::new("xash3d-query")
        .arg("all")
        .arg("-M")
        .arg(
            global
                .masters
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
        .arg("-j")
        .output()
        .map_err(|e| {
            error!("Make sure `xash3d-query` is installed: {e}");
            Status::InternalServerError
        })?;
    if scrape.status.success() {
        let result: scrape::Result = rocket::serde::json::serde_json::from_str(
            str::from_utf8(&scrape.stdout).map_err(|e| {
                error!("stdout parse error: {e}");
                Status::InternalServerError
            })?,
        )
        .map_err(|e| {
            error!("JSON parse error: {e}");
            Status::InternalServerError
        })?;
        Ok(Template::render(
            "index",
            context! {
                masters: &global.masters,
                title: &meta.title,
                version: &meta.version,
                servers: result.servers,
            },
        ))
    } else {
        error!("Make sure `xash3d-query` is installed!");
        Err(Status::InternalServerError)
    }
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
            masters: config.masters,
        })
        .manage(Meta {
            title: config.title,
            version: env!("CARGO_PKG_VERSION").into(),
        })
        .mount("/", routes![index])
}
