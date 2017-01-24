use std::fmt;
use std::str::FromStr;

use rand::{self, Rng};

use self::ParseError::*;

const MAX_SIZE: usize = 64;

pub struct PasteID(String);

#[derive(Debug)]
pub enum ParseError {
    LengthTooLong,
    InvalidCharacters,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LengthTooLong => write!(f, "ID length too long"),
            InvalidCharacters => write!(f, "ID contains invalid characters"),
        }
    }
}

impl PasteID {
    // Returns a new randomly generated id.
    pub fn new(size: usize) -> PasteID {
        PasteID(strgen::generate(size))
    }

    // returns the filename associated with the id
    pub fn filename(&self) -> String {
        format!("upload/{}", self.0)
    }
}

impl fmt::Display for PasteID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PasteID {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<PasteID, ParseError> {
        if s.len() > MAX_SIZE {
            return Err(LengthTooLong);
        }

        if !s.as_bytes().iter().all(|&x| BASE62.contains(&x)) {
            return Err(InvalidCharacters);
        }

        Ok(PasteID(String::from(s)))
    }
}
