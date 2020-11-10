use actix_web::web::Bytes;
use futures::future::ready;
use crate::errors::AppError;

pub async fn get_by_base64(data: Bytes) -> Result<(String, Vec<u8>), AppError> {
    let base64 = String::from_utf8(data.to_vec())?;
    let image = image_base64::from_base64(String::from(base64));
    let filename = format!("{}.png", uuid::Uuid::new_v4());
    Ok(ready((filename, image)).await)
}
+