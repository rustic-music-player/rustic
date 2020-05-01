use actix_web::{get, web, Responder, Result};

use crate::app::ApiState;

#[get("/extensions")]
pub async fn get_extensions(data: web::Data<ApiState>) -> Result<impl Responder> {
    let extensions = data.client.get_extensions().await?;

    Ok(web::Json(extensions))
}
