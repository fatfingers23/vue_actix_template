use crate::api::view_models::hello_world::{RepeatDTO, SayHelloDTO};
use crate::domain::error::ApiError;
use actix_web::web::Path;
use actix_web::{get, web, Result};

#[get("/hello")]
pub async fn say_hello_handler() -> Result<web::Json<SayHelloDTO>, ApiError> {
    Ok(web::Json(SayHelloDTO {
        message: "Hello World!".to_string(),
    }))
}

#[get("/repeat/{repeat_value}")]
pub async fn repeat_handler(params: Path<RepeatDTO>) -> Result<web::Json<SayHelloDTO>, ApiError> {
    let value = &params.repeat_value;

    Ok(web::Json(SayHelloDTO {
        message: format!("Hello {}", value),
    }))
}
