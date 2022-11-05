use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    // Set alias Error for ParseError
    type Error = ParseError;

    // try_from will return Self::Error - Alias for ParseError
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        /*
         * A verbose and less ideal approach. Example:
         *
         * match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
         *  Ok(request) => {}
         *  Err(e) => return Err(e),
         * }
         *
         * A better method using '?'
         * This will look at the Result, if the Result is Ok, it will return
         * If the Result is Err, it will return the error from our function:
         *
         * The '?' will try to convert the error type it receives, if it does not match the error
         * type the function is expected to return. 'from_utf8' returns its own Utf8Error
         *
         * Example: str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
         *
         * We can still make this cleaner by converting the from_utf8 error using 'From' trait, and
         * map it to ParseError::InvalidEncoding
         *
         * Because we know that every time we get a Utf8Error, a InvalidEnding error is good, we can override the default errors
         * using the method below
         *
         * Cleanest Approach with '?', from_utf8 returns Utf8Error, we override with 'From' and
         * return InvalidEncoding
         */

        let request = str::from_utf8(buf)?;

        unimplemented!()
    }
}

// Option<> will contain the next element of the string or return None if iterator has no more
// elements
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // enumerate() gives (i, val), index and value of index.
    for (i, c) in request.chars().enumerate() {
        // If c is = to space, we want to return tuple with two string slices
        if c == ' ' {
            // arg1: Get all of characters up until index of space. All of characters before space
            // arg2: Inclusive, so we get index of space +1 to include everything after the space
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

// Every time we get Utf8Error, let's return our defined InvalidEncoding error
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message());
        unimplemented!()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
