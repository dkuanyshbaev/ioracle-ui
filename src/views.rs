use crate::errors::IOracleResult;
use crate::models::{Answer, Hexagram, Trigram};
use crate::{iching, Config, Db};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use std::io::{prelude::*, BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::{fs, path::Path, process};

const IORACLE_SEND: &str = "/tmp/ioracle.send";
const IORACLE_RETURN: &str = "/tmp/ioracle.return";

#[derive(FromForm)]
pub struct Data {
    email: String,
    question: String,
}

#[derive(Serialize)]
struct NoContext {}

#[derive(Serialize)]
pub struct FullAnswer {
    pub answer: Answer,
    pub hexagram: Hexagram,
    pub related: Hexagram,
    pub core_primary: Hexagram,
    pub core_related: Hexagram,
    pub first_trigram: Trigram,
    pub second_trigram: Trigram,
    pub first_related: Trigram,
    pub second_related: Trigram,
    pub core_primary_first: Trigram,
    pub core_primary_second: Trigram,
    pub core_related_first: Trigram,
    pub core_related_second: Trigram,
    pub generative_answer: String,
}

#[get("/")]
pub fn index() -> Template {
    Template::render("index", NoContext {})
}

#[post("/question", data = "<data>")]
pub fn question(
    connection: Db,
    config: State<Config>,
    data: Form<Data>,
) -> IOracleResult<Redirect> {
    if let Ok(mut stream) = UnixStream::connect(IORACLE_SEND) {
        stream.write_all(b"read")?;
    };

    if Path::new(IORACLE_RETURN).exists() {
        if let Err(error) = fs::remove_file(IORACLE_RETURN) {
            println!("{}", error);
            process::exit(1);
        };
    }

    let mut location = "/".to_string();

    if let Ok(listener) = UnixListener::bind(IORACLE_RETURN) {
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                if let Some(result) = BufReader::new(stream).lines().nth(0) {
                    if let Ok(result) = result {
                        let result: Vec<&str> = result.split("|").collect();
                        let new_answer = Answer::new(
                            &data.email,
                            &data.question,
                            &result[0].to_string(),
                            &result[1].to_string(),
                        );

                        Answer::insert(&connection, &new_answer)?;
                        location = format!("/answer/{}", new_answer.id);
                        Answer::send(
                            &data.email,
                            &data.question,
                            &config.username,
                            &config.password,
                            &iching::full_answer(&connection, new_answer)?,
                        );
                    }
                }
                break;
            }
        }
    };

    Ok(Redirect::to(location))
}

#[get("/answer/<id>")]
pub fn answer(connection: Db, id: String) -> IOracleResult<Template> {
    let answer = Answer::get_by_id(&connection, id)?;
    Ok(Template::render(
        "answer",
        iching::full_answer(&connection, answer)?,
    ))
}

#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to("/")
}

#[catch(500)]
pub fn internal_error() -> Redirect {
    Redirect::to("/")
}
