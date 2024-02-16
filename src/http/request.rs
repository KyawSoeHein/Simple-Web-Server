use super::Method;
use super::ParseError;
use std::convert::TryFrom;
use std::str;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(buf) {
            Ok(request) => {}
            Err(_) => return Err(ParseError::InvalidEncoding),
        }
        unimplemented!()
    }
}
