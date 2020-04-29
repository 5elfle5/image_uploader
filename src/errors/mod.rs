use serde::Serialize;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::export::fmt::{Error, Formatter};
use std::fmt::{Display, Debug};
use actix_threadpool;
use std::convert::From;

pub enum AppError {
    CouldNotSave(String),
    CouldNotParseInput(String),
    CouldNotCreateThumbnail(String),
    IncorrectParameters(String),
}
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::CouldNotSave(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IncorrectParameters(_) => StatusCode::BAD_REQUEST,
            AppError::CouldNotParseInput(_) => StatusCode::BAD_REQUEST,
            AppError::CouldNotCreateThumbnail(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = match self {
            AppError::CouldNotCreateThumbnail(e) => e,
            AppError::CouldNotSave(e) => e,
            AppError::CouldNotParseInput(e) => e,
            AppError::IncorrectParameters(e) => e,
        }.parse();
        HttpResponse::build(self.status_code())
            .json(AppErrorResponse {
                error: message.unwrap_or(String::from("Unexpected error"))
            })
    }
}

impl From<actix_threadpool::BlockingError<std::io::Error>> for AppError {
        fn from(_: actix_threadpool::BlockingError<std::io::Error>) -> Self {
        AppError::CouldNotSave(String::from("Could not save file"))
    }
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        AppError::CouldNotParseInput(String::from("Could not parse input"))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(_: reqwest::Error) -> AppError {
        AppError::IncorrectParameters(String::from("Bad or no image found by url"))
    }
}

impl From<image::ImageError> for AppError {
    fn from(_: image::ImageError) -> AppError {
        AppError::CouldNotCreateThumbnail(String::from("Could not create thumbnail"))
    }
}
