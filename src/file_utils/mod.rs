use std::fs::File;
use std::io::Write;
use actix_web::{web};
use crate::errors::AppError;
use image::imageops;
use futures::future::ready;

pub async fn save_files((filename, buf): (String, Vec<u8>)) -> Result<(), AppError> {
    save_to_file(filename.clone(), buf).await?;
    save_thumbnail(filename).await?;
    Ok(())
}

async fn save_to_file(filename: String, buf: Vec<u8>) -> Result<(), AppError> {
    let file = create_file(format!("./tmp/{}", filename.clone())).await?;
    write_to_file(file, buf).await?;
    Ok(())
}

async fn save_thumbnail(filename: String) -> Result<(), AppError> {
    let image_path = format!("./tmp/{}", filename.clone());
    let preview_path = format!("./tmp/preview/{}", filename);
    let img_opt = image::open(&image_path);
    let mut img = img_opt?;
    let preview = imageops::thumbnail(&mut img, 100, 100);
    preview.save(preview_path)?;
    ready(Ok(())).await
}

async fn create_file(filepath: String) -> Result<File, AppError> {
    Ok(web::block(move || std::fs::File::create(filepath)).await?)
}

async fn write_to_file(mut file: File, buf: Vec<u8>) -> Result<File, AppError> {
    Ok(web::block(move || file.write_all(&buf).map(|_| file)).await?)
}
