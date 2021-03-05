#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

mod errors;
mod iching;
mod lang;
mod models;
mod schema;
mod views;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::{env, process};

#[database("ioracle")]
pub struct Db(diesel::SqliteConnection);

pub struct Config {
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new() -> Result<Config, env::VarError> {
        let username = env::var("USERNAME")?;
        let password = env::var("PASSWORD")?;
        Ok(Config { username, password })
    }
}

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    rocket::ignite()
        .manage(config)
        .attach(Db::fairing())
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static/"))
        .mount("/", routes![views::index, views::question, views::answer])
        .register(catchers![views::not_found, views::internal_error])
        .launch();
}
