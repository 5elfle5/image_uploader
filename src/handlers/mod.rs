mod base64_image;
mod url_image;

extern crate image_base64;

use actix_web::{Error, HttpResponse};
use actix_multipart::{Multipart};
use crate::errors::{AppError};
use crate::file_utils::save_files;
use futures::StreamExt;
use base64_image::get_by_base64;
use url_image::get_by_url;

macro_rules! pipe {
    ($f:expr , $g:expr) => (|x| Box::pin(async { $g($f(x).await?).await }));
}

pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut res: Result<(), AppError> = Ok(());
    while let Some(Ok(mut field)) = payload.next().await {
        let disposition = field.content_disposition()
            .ok_or(AppError::CouldNotParseInput("Could not parse the form".to_string()))?;
        let filename = disposition.get_filename()
            .ok_or(AppError::CouldNotParseInput("Found file without a name".to_string()))?;
        let field_name = disposition.get_name()
            .ok_or(AppError::IncorrectParameters("Found field without a name".to_string()))?;
        if let Some(Ok(data)) = field.next().await {
            res = match field_name {
                "base64" => pipe!(get_by_base64, save_files)(data).await,
                "url" => pipe!(get_by_url, save_files)(data).await,
                "files" => save_files((filename.to_string(), data.to_vec())).await,
                _ => Err(AppError::IncorrectParameters(String::from("Unknown field"))),
            };
        }
    }
    match res {
        Ok(_) => Ok(HttpResponse::Created().into()),
        Err(e) => Err(e.into()),
    }
}

pub fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html"))
}

