mod base64_image;
mod url_image;
mod binary_image;

extern crate image_base64;

use actix_web::{Error, HttpResponse};
use futures::StreamExt;
use actix_multipart::{Multipart};
use crate::errors::{AppError};

pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut res: Result<(), AppError> = Ok(());
    while let Some(Ok(mut field)) = payload.next().await {
        let disposition_opt = field.content_disposition();
        let field_name = disposition_opt.as_ref()
            .and_then(|cd| cd.get_name())
            .ok_or(AppError::IncorrectParameters(String::from("Found field without a name")))?;
        res = match field_name {
            "base64" => base64_image::handle(&mut field).await,
            "url" => url_image::handle(&mut field).await,
            "files" => binary_image::handle(&mut field).await,
            _ => Err(AppError::IncorrectParameters(String::from("Unknown field"))),
        };
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
