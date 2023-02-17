use crate::api::view_models::hello_world::{RepeatDTO, SayHelloDTO};
use crate::domain::dto::todo::Todo;
use crate::domain::error::{ApiError, CommonError};
use crate::domain::repositories::todo::TodoRepository;
use actix_web::{get, web, Result};

pub async fn say_hello_handler() -> Result<web::Json<SayHelloDTO>, ApiError> {
    Ok(web::Json(SayHelloDTO {
        message: "Hello World!".to_string(),
    }))
}

#[get("/list")]
pub async fn list_todos_handler(
    todo_repo: web::Data<dyn TodoRepository>,
) -> Result<web::Json<Vec<Todo>>, ApiError> {
    let todos = todo_repo.list_by_session_id(1).await;
    match todos {
        Ok(todos) => Ok(web::Json(todos)),
        Err(e) => Err(ApiError::from(CommonError {
            message: e.message,
            code: 500,
        })),
    }
}
