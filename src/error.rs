use std::error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    InvalidToken,
    InvalidGroup,
    FactorExpected,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::UnexpectedToken => "Unconsumed input",
            ParseError::InvalidToken => "Unrecognized token",
            ParseError::InvalidGroup => "Expected group close",
            ParseError::FactorExpected => "Expected integer, negation, or group",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None,
        }
    }
}
