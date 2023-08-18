#![allow(dead_code)]

use std::error;
use std::fmt;

use actix_web::ResponseError;

#[derive(Debug)]
pub enum Error {
    Error(String),
}

// unsafe impl Send for Error {
    
// }

// unsafe impl Sync for Error {
    
// }

impl Error {
    pub fn new_boxed<T>(value: &str) -> Result<T, Box<dyn Send + Sync + std::error::Error>> {
        Err::<T, Box<dyn Send + Sync + std::error::Error>>(Box::new(Error::Error(value.to_string())))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            Error::Error(err) => write!(f, "Error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            Error::Error(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        return Some(self);
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut res = actix_web::HttpResponse::new(self.status_code());
        res.set_body(actix_web::body::BoxBody::new(format!("{}",self)))
    }
}
