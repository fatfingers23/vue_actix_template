use crate::api::view_models::hello_world::{RepeatDTO, SayHelloDTO};
use crate::domain::dto::todo::Todo;
use crate::domain::error::{ApiError, CommonError};
use crate::domain::repositories::todo::TodoRepository;
use actix_session::Session;
use actix_web::{get, web, HttpRequest, Result};
use diesel::insert_into;
use uuid::Uuid;

pub async fn say_hello_handler() -> Result<web::Json<SayHelloDTO>, ApiError> {
    Ok(web::Json(SayHelloDTO {
        message: "Hello World!".to_string(),
    }))
}

#[get("/list")]
pub async fn list_todos_handler(
    req: HttpRequest,
    todo_repo: web::Data<dyn TodoRepository>,
    session: Session,
) -> Result<web::Json<Vec<Todo>>, ApiError> {
    let session_id = session.get::<Uuid>("user_id").unwrap().unwrap();

    println!("session_id: {:?}", session_id);
    let todos = todo_repo.list_by_session_id(session_id).await;
    match todos {
        Ok(todos) => Ok(web::Json(todos)),
        Err(e) => Err(ApiError::from(CommonError {
            message: e.message,
            code: 500,
        })),
    }
}
