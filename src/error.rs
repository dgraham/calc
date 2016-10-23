use std::error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEof,
    UnexpectedToken,
    InvalidToken(usize),
    InvalidGroup(usize),
    FactorExpected(usize),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::UnexpectedEof => write!(f, "{}", self.description()),
            ParseError::UnexpectedToken => write!(f, "{}", self.description()),
            ParseError::InvalidToken(pos) => write!(f, "Unrecognized token: {}", pos),
            ParseError::InvalidGroup(pos) => write!(f, "Expected group close: {}", pos),
            ParseError::FactorExpected(pos) => {
                write!(f, "Expected integer, negation, or group: {}", pos)
            }
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::UnexpectedEof => "Unexpected end of file",
            ParseError::UnexpectedToken => "Unconsumed input",
            ParseError::InvalidToken(_) => "Unrecognized token",
            ParseError::InvalidGroup(_) => "Expected group close",
            ParseError::FactorExpected(_) => "Expected integer, negation, or group",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None,
        }
    }
}
