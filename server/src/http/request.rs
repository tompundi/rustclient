use super::method::Method;
use std::convert::TryFrom;


pub struct Request {
    path: String,
    query_string: Option<String>,
    method: super::method::Method,
}

// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {

//     }
// }

impl TryFrom<&[8]> for Request {
    type Error = String;

    fn try_from(buf: &[8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}