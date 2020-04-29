use actix_multipart::Field;
use crate::errors::{AppError};
use actix_web::web::Bytes;
use crate::file_utils;
use crate::image_utils;
use futures::StreamExt;
use uuid::Uuid;
use std::panic;

pub async fn handle(field: &mut Field) -> Result<(), AppError> {
    if let Some(Ok(data)) = field.next().await {
        let new_file_name = format!("{}.png", Uuid::new_v4());
        let image = get_by_base64(data, &new_file_name)?;
        file_utils::save_to_file(image).await?;
        image_utils::save_thumbnail(new_file_name).await?;
    }
    Ok(())
}

fn get_by_base64(data: Bytes, filename: &String) -> Result<(String, Vec<u8>), AppError> {
    let base64 = String::from_utf8(data.to_vec())?;
    let path = format!("./tmp/{}", filename);
    panic::set_hook(Box::new(|_info| {
        // do not log panic, I mean, I know, that's why I caught unwind
    }));
    let image_res = panic::catch_unwind(|| image_base64::from_base64(String::from(base64)));
    let image = match image_res {
        Ok(i) => i,
        Err(_) => {
            return Err(AppError::IncorrectParameters(String::from("Not valid base64")));
        }
    };
    Ok((path, image))
}
