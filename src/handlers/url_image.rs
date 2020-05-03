use crate::errors::{AppError};
use actix_web::web::Bytes;

pub async fn get_by_url(data: Bytes) -> Result<(String, Vec<u8>), AppError> {
    let url = String::from_utf8(data.to_vec())?;
    let parts: Vec<&str> = url.split(r#"/"#).collect();
    if parts.len() < 1 {
        return Err(AppError::IncorrectParameters(String::from("Bad URL")));
    }
    let filename = parts[parts.len() - 1];
    let response = reqwest::get(&url).await?;
    let bytes: bytes::Bytes = response.bytes().await?;
    Ok((filename.to_string(), bytes.to_vec()))
}
