use image::imageops;
use crate::errors::{AppError};
use crate::file_utils;

pub async fn save_thumbnail(filename: String) -> Result<(), AppError> {
    let path = format!("./tmp/{}", filename);
    let img_opt = image::open(&path);
    let mut img = match img_opt {
        image::ImageResult::Ok(i) => i,
        image::ImageResult::Err(_) => {
            file_utils::remove_file(path).await?;
            return Err(AppError::IncorrectParameters(String::from("File is not an image")));
        }
    };
    let preview = imageops::thumbnail(&mut img, 100, 100);
    preview.save(format!("./tmp/preview/{}", filename))?;
    Ok(())
}

