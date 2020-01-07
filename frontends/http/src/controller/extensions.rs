use actix_web::{get, HttpResponse, post, Responder, Result, web};

use app::ApiState;
use handler::extensions as extensions_handler;

#[get("/extensions")]
pub fn get_extensions(data: web::Data<ApiState>) -> Result<impl Responder> {
    let rustic = &data.app;
    let extensions = extensions_handler::get_extensions(&rustic);

    Ok(web::Json(extensions))
}
