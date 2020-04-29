use std::fs::File;
use std::io::Write;
use actix_web::{web};
use crate::errors::AppError;

pub async fn save_to_file((filepath, buf): (String, Vec<u8>)) -> Result<(), AppError> {
    let file = create_file(filepath).await?;
    write_to_file(file, buf).await?;
    Ok(())
}

pub async fn create_file(filepath: String) -> Result<File, AppError> {
    Ok(web::block(move || std::fs::File::create(filepath)).await?)
}

pub async fn remove_file(filepath: String) -> Result<(), AppError> {
    Ok(web::block(move || std::fs::remove_file(filepath)).await?)
}

pub async fn write_to_file(mut file: File, buf: Vec<u8>) -> Result<File, AppError> {
    Ok(web::block(move || file.write_all(&buf).map(|_| file)).await?)
}
