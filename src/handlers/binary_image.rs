use actix_multipart::Field;
use crate::errors::AppError;
use crate::image_utils;
use crate::file_utils;
use futures::StreamExt;

pub async fn handle(field: &mut Field) -> Result<(), AppError> {
    let disposition_opt = field.content_disposition();
    let filename_opt = disposition_opt.as_ref().and_then(|cd| cd.get_filename());
    if let Some(filename) = filename_opt {
        let path = format!("./tmp/{}", &filename);
        let mut file = file_utils::create_file(path).await?;
        while let Some(Ok(data)) = field.next().await {
            file = file_utils::write_to_file(file, data.to_vec()).await?;
        }
        image_utils::save_thumbnail(String::from(filename)).await?;
    }
    Ok(())
}

