// User-visible errors.

use std::fmt;

#[derive(PartialEq)]
pub enum AppError {
    InvalidCommandErr,
    InvalidGuessLengthErr,
    NonAlphaGuessErr,
    NotEnglishGuessErr,
}

use AppError::*;
use crate::constants::WORD_LENGTH;

impl AppError {
    pub fn to_str<T: fmt::Debug>(&self, arg: Option<&T>) -> String {
        match self {
            InvalidCommandErr => format!("\nInvalid command. Choose only from the following commands: {:?}.", arg.unwrap()),
            InvalidGuessLengthErr => format!("The guessed word must have {} characters only.\n", WORD_LENGTH),
            NonAlphaGuessErr => "The guessed word must contain alphabetical characters only.\n".to_string(),
            NotEnglishGuessErr => "The guessed word is not a valid English word.\n".to_string(),
        }
    }
}
