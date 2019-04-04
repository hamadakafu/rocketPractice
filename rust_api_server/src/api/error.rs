use mongodb;
use failure::{Fail, Context, Backtrace};
use rocket::http;

use std::fmt::Display;

use crate::othello::error::OthelloError;

#[derive(Debug)]
pub struct APIError {
    inner: Context<APIErrorKind>,
}

impl Fail for APIError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl APIError {
    pub fn kind(&self) -> APIErrorKind {
        self.inner.get_context().clone()
    }
    pub fn to_status(&self) -> http::Status {
        match self.kind() {
            APIErrorKind::AlreadyExistRoom(_) => http::Status::NotAcceptable,
            APIErrorKind::NotExistRoom(_) => http::Status::NotAcceptable,
            APIErrorKind::MongoError(_) => http::Status::ServiceUnavailable,
            APIErrorKind::GameSystemError(_) => http::Status::InternalServerError,
        }
    }
}

impl From<APIErrorKind> for APIError {
    fn from(kind: APIErrorKind) -> APIError {
        APIError { inner: Context::new(kind) }
    }
}

impl From<Context<APIErrorKind>> for APIError {
    fn from(inner: Context<APIErrorKind>) -> APIError {
        APIError { inner }
    }
}

#[derive(Fail, Debug, Clone, Eq, PartialEq)]
pub enum APIErrorKind {
    #[fail(display = "{} does not exist.", _0)]
    NotExistRoom(String),
    #[fail(display = "{} already exist.", _0)]
    AlreadyExistRoom(String),
    #[fail(display = "mongo error: {}", _0)]
    MongoError(String),
    #[fail(display = "game system error: {}", _0)]
    GameSystemError(OthelloError)
}

impl From<mongodb::Error> for APIErrorKind {
    fn from(m_error: mongodb::Error) -> APIErrorKind {
        APIErrorKind::MongoError(m_error.to_string())
    }
}

impl From<OthelloError> for APIErrorKind {
    fn from(o_error: OthelloError) -> APIErrorKind {
        APIErrorKind::GameSystemError(o_error)
    }
}
