use rocket_contrib::databases::diesel::result::Error as DieselError;
use std::convert::From;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum IOracleError {
    NotFound,
    InternalServerError,
}

pub type IOracleResult<T> = Result<T, IOracleError>;

impl fmt::Display for IOracleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IOracleError::NotFound => write!(f, "NotFound"),
            IOracleError::InternalServerError => write!(f, "InternalServerError"),
        }
    }
}

impl error::Error for IOracleError {
    fn description(&self) -> &str {
        match *self {
            IOracleError::NotFound => "Record not found",
            IOracleError::InternalServerError => "Internal server error",
        }
    }
}

impl From<DieselError> for IOracleError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => IOracleError::NotFound,
            _ => IOracleError::InternalServerError,
        }
    }
}

impl From<io::Error> for IOracleError {
    fn from(error: io::Error) -> Self {
        match error {
            _ => IOracleError::InternalServerError,
        }
    }
}
