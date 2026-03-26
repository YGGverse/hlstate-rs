#[macro_use]
extern crate rocket;

mod argument;
mod config;
mod global;
mod meta;
mod scrape;

use chrono::{DateTime, Utc};
use global::Global;
use meta::Meta;
use rocket::{
    State,
    http::Status,
    tokio::{spawn, sync::RwLock, time::sleep},
};
use rocket_dyn_templates::{Template, context};
use std::{collections::HashSet, net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};

struct Online {
    scrape: scrape::Result,
    update: DateTime<Utc>,
}
type Snap = Arc<RwLock<Option<Online>>>;

#[get("/")]
async fn index(
    meta: &State<Meta>,
    global: &State<Global>,
    online: &State<Snap>,
) -> Result<Template, Status> {
    let snap = online.read().await;
    Ok(Template::render(
        "index",
        context! {
            masters: &global.masters,
            title: &meta.title,
            version: &meta.version,
            servers: snap.as_ref().map(|s|s.scrape.servers.clone()),
            updated: snap.as_ref().map(|s|s.update.to_rfc2822())
        },
    ))
}

#[launch]
async fn rocket() -> _ {
    use clap::Parser;
    let argument = argument::Argument::parse();
    let config: config::Config =
        toml::from_str(&std::fs::read_to_string(argument.config).unwrap()).unwrap();
    let online: Snap = Arc::new(RwLock::new(None));
    spawn({
        let online = online.clone();
        let query = config.query.clone();
        let masters = config.masters.clone();
        async move {
            loop {
                match scrape(&query, &masters) {
                    Ok(s) => match str::from_utf8(&s.stdout) {
                        Ok(r) => {
                            if s.status.success() {
                                *online.write().await =
                                    match rocket::serde::json::serde_json::from_str(r) {
                                        Ok(scrape) => Some(Online {
                                            scrape,
                                            update: Utc::now(),
                                        }),
                                        Err(e) => {
                                            error!("Could not decode scrape response: `{e}`");
                                            None
                                        }
                                    }
                            } else {
                                error!("Scrape request failed");
                            }
                        }
                        Err(e) => error!("Could not decode UTF-8: `{e}`"),
                    },
                    Err(e) => error!("Scrape error: `{e}`"),
                }
                sleep(Duration::from_secs(config.refresh)).await;
            }
        }
    });
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
        .manage(online)
        .manage(Global {
            masters: config.masters,
            query: config.query,
        })
        .manage(Meta {
            title: config.title,
            version: env!("CARGO_PKG_VERSION").into(),
        })
        .mount("/", routes![index])
}

/// Get servers online using `bin` from given `masters`
fn scrape(
    bin: &PathBuf,
    masters: &HashSet<SocketAddr>,
) -> Result<std::process::Output, std::io::Error> {
    // @TODO: requires library impl
    // https://github.com/FWGS/xash3d-master/issues/4
    std::process::Command::new(bin)
        .arg("all")
        .arg("-M")
        .arg(
            masters
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
        .arg("-j")
        .output()
}
