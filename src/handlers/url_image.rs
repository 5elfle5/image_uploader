use actix_multipart::Field;
use crate::errors::{AppError};
use crate::image_utils;
use crate::file_utils;
use futures::StreamExt;

pub async fn handle(field: &mut Field) -> Result<(), AppError> {
    if let Some(Ok(data)) = field.next().await {
        let url = String::from_utf8(data.to_vec())?;
        let parts: Vec<&str> = url.split(r#"/"#).collect();
        if parts.len() < 1 {
            return Err(AppError::IncorrectParameters(String::from("Bad URL")));
        }
        let filename = parts[parts.len() - 1];
        let image = get_by_url(&url, &filename).await?;
        let save_res = file_utils::save_to_file(image).await;
        if let Err(_) = save_res {
            return Err(AppError::IncorrectParameters(String::from("URL does not contain a file")));
        }
        image_utils::save_thumbnail(String::from(filename)).await?;
    }
    Ok(())
}

async fn get_by_url(url: &str, filename: &str) -> Result<(String, Vec<u8>), AppError> {
    let response = reqwest::get(url).await?;
    let bytes: bytes::Bytes = response.bytes().await?;
    let path = format!("./tmp/{}", filename);
    Ok((path, bytes.to_vec()))
}
