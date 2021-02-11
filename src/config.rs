use std::env;

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
