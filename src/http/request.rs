use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

// Define lifetime for our buffer
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

// TODO: WTF LIFETIMES
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    // Set alias Error for ParseError
    type Error = ParseError;

    // Example Request: GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...

    // try_from will return Self::Error - Alias for ParseError
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
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
         * Because we know that every time we get a Utf8Error, an InvalidEnding error is good, we can override the default errors
         * using the method below
         *
         * Cleanest Approach with '?', from_utf8 returns Utf8Error, we override with 'From' and
         * return InvalidEncoding
         */

        let request = str::from_utf8(buf)?;

        // Example Request: GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
        /*
         * ok_or() transforms an option into a result
         *
         * get_next_word() returns an option
         *
         * if the return Option is == Some(), it will be converted into an Ok (enum) variant of the
         * result.
         *
         * if None, we will return our defined error, ParseError::InvalidRequest
         */
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        /*
        * Examples of more verbose, solutions followed by cleanest
        *
        * match path.find('?') {
        *     some(i) => {
        *         // pad with +1 so ? isn't in query string
        *         query_string = some(&path[i + 1..]);
        *         // assign everything before '?' to path
        *         path = &path[..i];
        *     }
        *     none => {}
        * }

        * let q = path.find('?');
        * if q.is_some() {
        *     let i = q.unwrap();
        *     // pad with +1 so ? isn't in query string
        *     query_string = some(&path[i + 1..]);
        *     // assign everything before '?' to path
        *     path = &path[..i];
        * }
        *
        */

        let method: Method = method.parse()?;
        let mut query_string = None;

        if let Some(i) = path.find('?') {
            // Pad with +1 so ? isn't in query string
            query_string = Some(QueryString::from(&path[i + 1..]));

            // Assign everything before '?' to path
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

// Option<> will contain the next element of the string or return None if iterator has no more
// elements
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // enumerate() gives (i, val), index and value of index.
    for (i, c) in request.chars().enumerate() {
        // If c is = to space, we want to return tuple with two string slices
        if c == ' ' || c == '\r' {
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
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

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
