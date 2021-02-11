#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

mod config;
mod errors;
mod iching;
mod models;
mod schema;
mod views;

use config::Config;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::{fs, path::Path, process};

const IORACLE_SEND: &str = "/tmp/ioracle.send";
const IORACLE_RETURN: &str = "/tmp/ioracle.return";

#[database("ioracle")]
pub struct Db(diesel::SqliteConnection);

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if Path::new(IORACLE_RETURN).exists() {
        if let Err(error) = fs::remove_file(IORACLE_RETURN) {
            println!("{}", error);
            process::exit(1);
        };
    }

    rocket::ignite()
        .manage(config)
        .attach(Db::fairing())
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount("/", routes![views::index, views::question, views::answer])
        .register(catchers![views::not_found, views::internal_error])
        .launch();
}
