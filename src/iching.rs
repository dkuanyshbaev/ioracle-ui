use crate::errors::IOracleResult;
use crate::models::{Answer, Hexagram, Trigram};
use crate::views::FullAnswer;
use rocket_contrib::databases::diesel::SqliteConnection;

pub fn full_answer(connection: &SqliteConnection, answer: Answer) -> IOracleResult<FullAnswer> {
    let hexagram_binary = answer.hexagram.clone();
    let related_binary = answer.related.clone();

    let hexagram = Hexagram::get_by_binary(&connection, &hexagram_binary.clone())?;
    let related = Hexagram::get_by_binary(&connection, &related_binary.clone())?;

    let core_primary_binary = format!("{}{}", &hexagram_binary[1..4], &hexagram_binary[2..5]);
    let core_related_binary = format!("{}{}", &related_binary[1..4], &related_binary[2..5]);

    let core_primary = Hexagram::get_by_binary(&connection, &core_primary_binary)?;
    let core_related = Hexagram::get_by_binary(&connection, &core_related_binary)?;

    let first_trigram = Trigram::get_by_binary(&connection, (&hexagram_binary[..3]).to_string())?;
    let second_trigram = Trigram::get_by_binary(&connection, (&hexagram_binary[3..]).to_string())?;
    let first_related = Trigram::get_by_binary(&connection, (&related_binary[..3]).to_string())?;
    let second_related = Trigram::get_by_binary(&connection, (&related_binary[3..]).to_string())?;

    let core_primary_first =
        Trigram::get_by_binary(&connection, (&hexagram_binary[1..4]).to_string())?;
    let core_primary_second =
        Trigram::get_by_binary(&connection, (&hexagram_binary[2..5]).to_string())?;
    let core_related_first =
        Trigram::get_by_binary(&connection, (&related_binary[1..4]).to_string())?;
    let core_related_second =
        Trigram::get_by_binary(&connection, (&related_binary[2..5]).to_string())?;

    let generative_answer = generative_answer(&hexagram_binary);

    Ok(FullAnswer {
        answer,
        hexagram,
        related,
        core_primary,
        core_related,
        first_trigram,
        second_trigram,
        first_related,
        second_related,
        core_primary_first,
        core_primary_second,
        core_related_first,
        core_related_second,
        generative_answer,
    })
}

fn generative_answer(hexagram: &String) -> String {
    println!("{}", hexagram);

    "----".to_string()
}
