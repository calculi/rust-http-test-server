use super::method::{Method, MethodError};
use super::{QueryString};
use std::convert::TryFrom;
use std::convert::From;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    // note: Reference to the Option of a reference
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // Alternative 1
        // match str::from_utf8(buf) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // Alternative 2
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }

        // Alternative 3
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        let request = str::from_utf8(buf)?;

        // Long way of doing it!
        // 
        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i]
        //     }
        //     None => {}
        // }

        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..i]
        // }
        //
        // Better way than using two statements above ...
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i]
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {}
    //         None => break,
    //     }
    // }

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
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

// Every time there's UTF8Error, it will parse to InvalidEncoding
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self:: InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self:: InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}