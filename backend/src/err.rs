#![allow(dead_code)]

use actix_web::ResponseError;
use std::fmt;

#[derive(Debug)]
struct StdErr;

impl std::error::Error for StdErr {}

impl fmt::Display for StdErr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "StdErr")
	}
}

#[derive(Debug)]
pub struct Error(String);

impl Error {
	pub fn new_boxed(value: &str) -> Box<Error> {
		Box::new(Error(value.to_string()))
	}

	pub fn from_str(value: &str) -> Self {
		Error(value.to_string())
	}

	pub fn new_res<T>(value: &str) -> Result<T, Self> {
		Err(Error(value.to_string()))
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", &self.0)
	}
}

impl From<Error> for Box<dyn std::error::Error> {
	fn from(_: Error) -> Self {
		Box::new(StdErr)
	}
}

impl<T: std::error::Error + Send + Sync + 'static> From<T> for Error {
	fn from(e: T) -> Self {
		Error(e.to_string())
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
