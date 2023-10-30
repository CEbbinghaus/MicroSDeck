#![allow(dead_code)]

use std::error;
use std::fmt;

use actix_web::ResponseError;

#[derive(Debug)]
pub enum Error {
	Error(String),
}

impl Error {
	pub fn new_boxed(value: &str) -> Box<Error> {
		Box::new(Error::Error(value.to_string()))
	}

	pub fn from_str(value: &str) -> Self {
		Error::Error(value.to_string())
	}

	pub fn new_res<T>(value: &str) -> Result<T, Self> {
		Err(Error::Error(value.to_string()))
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

impl<T: error::Error + Send + Sync + 'static> From<T> for Error {
	fn from(e: T) -> Self {
		Error::Error(e.to_string())
	}
}

impl ResponseError for Error {
	fn status_code(&self) -> actix_web::http::StatusCode {
		actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
	}

	fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
		let res = actix_web::HttpResponse::new(self.status_code());
		res.set_body(actix_web::body::BoxBody::new(format!("{}", self)))
	}
}

impl ResponseError for Box<Error> {
	fn status_code(&self) -> actix_web::http::StatusCode {
		actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
	}

	fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
		let res = actix_web::HttpResponse::new(self.status_code());
		res.set_body(actix_web::body::BoxBody::new(format!("{}", self)))
	}
}
