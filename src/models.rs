use crate::schema::{answers, hexagrams, trigrams};
use crate::views::FullAnswer;
use chrono::NaiveDateTime;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::databases::diesel::SqliteConnection;
use uuid::Uuid;

#[derive(Serialize, Queryable, Identifiable)]
pub struct Hexagram {
    pub id: i32,
    pub binary: String,
    pub king_wen_order: i32,
    pub shao_yong_order: i32,
    pub gua: String,
    pub pin_yin: String,
    pub character: String,
    pub wilheim: String,
    pub huang: String,
    pub hatcher: String,
    pub no2do: String,
    pub inner_ba_gua: String, // first_trigram
    pub outer_ba_gua: String, // second_trigram
    pub host_yao: String,
    pub judgment: String,
    pub image: String,
    pub lines: String,
}

impl Hexagram {
    pub fn get_by_binary(connection: &SqliteConnection, binary: &String) -> QueryResult<Hexagram> {
        hexagrams::table
            .filter(hexagrams::binary.eq(&binary))
            .first(connection)
    }
}

#[derive(Serialize, Queryable, Identifiable)]
pub struct Trigram {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub binary: String,
    pub no: String,
    pub wen: String,
    pub host: String,
    pub element: String,
}

impl Trigram {
    pub fn get_by_binary(connection: &SqliteConnection, binary: String) -> QueryResult<Trigram> {
        trigrams::table
            .filter(trigrams::binary.eq(binary))
            .first(connection)
    }
}

#[derive(Serialize, Queryable, Insertable, Identifiable)]
pub struct Answer {
    pub id: String,
    pub email: String,
    pub question: String,
    pub hexagram: String,
    pub related: String,
    pub created_at: Option<NaiveDateTime>,
}

impl Answer {
    pub fn new(email: &String, question: &String, hexagram: &String, related: &String) -> Answer {
        let uuid = Uuid::new_v4();
        Answer {
            id: uuid.to_string(),
            email: email.to_string(),
            question: question.to_string(),
            hexagram: hexagram.to_string(),
            related: related.to_string(),
            created_at: None,
        }
    }

    pub fn get_by_id(connection: &SqliteConnection, id: String) -> QueryResult<Answer> {
        answers::table.filter(answers::id.eq(&id)).first(connection)
    }

    pub fn insert(connection: &SqliteConnection, new_answer: &Answer) -> QueryResult<usize> {
        diesel::insert_into(answers::table)
            .values(new_answer)
            .execute(connection)
    }

    pub fn send(
        email: &String,
        _question: &String,
        username: &String,
        password: &String,
        full_answer: &FullAnswer,
    ) {
        println!("sending mail..");

        let head_text =
            "<h1>I ORACLE<br>NATURE BASED ETHICAL GUIDANCE SYSTEM<br>FOR MAN AND MACHINE</h1>"
                .to_string();

        let hexagram_text = format!(
            "
                <h4>PRIMARY:</h4>
                <p>{}</p>
                <p>{}</p>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <p>over</p>
                <p>{}</p>
                <p>{}</p>
                ",
            full_answer.hexagram.king_wen_order,
            full_answer.hexagram.hatcher,
            full_answer.hexagram.binary,
            full_answer.second_trigram.image,
            full_answer.second_trigram.name,
            full_answer.first_trigram.image,
            full_answer.first_trigram.name,
            full_answer.hexagram.gua,
            full_answer.hexagram.pin_yin,
            full_answer.hexagram.character,
            full_answer.second_trigram.name,
            full_answer.first_trigram.name,
            full_answer.hexagram.shao_yong_order,
        );

        let related_text = format!(
            "
                <h4>RELATED:</h4>
                <p>{}</p>
                <p>{}</p>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <p>over</p>
                <p>{}</p>
                <p>{}</p>
                ",
            full_answer.related.king_wen_order,
            full_answer.related.hatcher,
            full_answer.related.binary,
            full_answer.second_related.image,
            full_answer.second_related.name,
            full_answer.first_related.image,
            full_answer.first_related.name,
            full_answer.related.gua,
            full_answer.related.pin_yin,
            full_answer.related.character,
            full_answer.second_related.name,
            full_answer.first_related.name,
            full_answer.related.shao_yong_order,
        );

        let core_primary_text = format!(
            "
                <h4>CORE PRIMARY:</h4>
                <p>{}</p>
                <p>{}</p>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <p>over</p>
                <p>{}</p>
                <p>{}</p>
                ",
            full_answer.core_primary.king_wen_order,
            full_answer.core_primary.hatcher,
            full_answer.core_primary.binary,
            full_answer.core_primary_second.image,
            full_answer.core_primary_second.name,
            full_answer.core_primary_first.image,
            full_answer.core_primary_first.name,
            full_answer.core_primary.gua,
            full_answer.core_primary.pin_yin,
            full_answer.core_primary.character,
            full_answer.core_primary_second.name,
            full_answer.core_primary_first.name,
            full_answer.core_primary.shao_yong_order,
        );

        let core_related_text = format!(
            "
                <h4>CORE RELATED:</h4>
                <p>{}</p>
                <p>{}</p>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <h1>{}</h1>
                <p>{}</p>
                <p>over</p>
                <p>{}</p>
                <p>{}</p>
                ",
            full_answer.core_related.king_wen_order,
            full_answer.core_related.hatcher,
            full_answer.core_related.binary,
            full_answer.core_related_second.image,
            full_answer.core_related_second.name,
            full_answer.core_related_first.image,
            full_answer.core_related_first.name,
            full_answer.core_related.gua,
            full_answer.core_related.pin_yin,
            full_answer.core_related.character,
            full_answer.core_related_second.name,
            full_answer.core_related_first.name,
            full_answer.core_related.shao_yong_order,
        );

        let traditional_text = format!(
            "
                <h4>TRADITIONAL INTERPRETATIONS:</h4>
                <br>
                <h4>King Wen's Decision</h4>
                <br>
                {}
                <br>
                <br>
                <h4>The Image</h4>
                <br>
                {}
                <br>
                <br>
                <h4>Duke of Zhou</h4>
                <br>
                {}
                <br>
                <br>
                ",
            full_answer.hexagram.judgment, full_answer.hexagram.image, full_answer.hexagram.lines,
        );

        let traditional_related_text = format!(
            "
                <h4>King Wen's Decision Related</h4>
                <br>
                {}
                <br>
                <br>
                <h4>The Image Related</h4>
                <br>
                {}
                <br>
                <br>
                <h4>Duke of Zhou Related</h4>
                <br>
                {}
                ",
            full_answer.related.judgment, full_answer.related.image, full_answer.related.lines,
        );

        let body_text = format!(
            "{}<hr>{}<br>{}<br>{}<br>{}<hr><p>{}<p>{}{}",
            head_text,
            hexagram_text,
            related_text,
            core_primary_text,
            core_related_text,
            full_answer.generative_answer,
            traditional_text,
            traditional_related_text,
        );

        match EmailBuilder::new()
            .from("readings@ioracle.net".parse::<String>().unwrap())
            .to(email.parse::<String>().unwrap())
            .subject("I ORACLE")
            .html(body_text)
            .build()
        {
            Ok(email) => {
                let credentials = (username.to_owned(), password.to_owned()).into_credentials();
                let mut client = SmtpClient::new_simple("mail.privateemail.com")
                    .unwrap()
                    .credentials(credentials)
                    .transport();

                let _result = client.send(email.into());
                println!("Email sent successfully!");
            }
            Err(error) => println!("Email sending error: {:?}", error),
        }
    }
}
