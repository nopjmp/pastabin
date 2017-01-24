use std::fmt;
use std::str::FromStr;

use rand::{self, Rng};

use self::ParseError::*;

const BASE62: &'static [u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
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
        let mut rng = rand::thread_rng();
        let mut id = String::with_capacity(size);
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }
        PasteID(id)
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
