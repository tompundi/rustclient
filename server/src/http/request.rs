use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatterResult;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: super::method::Method,
}


impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
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

impl Error for ParseError {

}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FormatterResult {
        write!(f, "{}", self.message())
    }
    
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FormatterResult {
        write!(f, "{}", self.message())
    }
    
}