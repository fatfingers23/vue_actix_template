use crate::domain::dto::todo::{CreateTodo, CreateTodoDTO, Todo};
use crate::domain::error::{ApiError, CommonError};
use crate::domain::repositories::todo::TodoRepository;
use actix_session::Session;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result, Scope};
use uuid::Uuid;

#[get("/list")]
pub async fn list_todos_handler(
    todo_repo: web::Data<dyn TodoRepository>,
    session: Session,
) -> Result<web::Json<Vec<Todo>>, ApiError> {
    let session_id = session.get::<Uuid>("user_id").unwrap().unwrap();
    let todos = todo_repo.list_by_session_id(session_id).await;
    match todos {
        Ok(todos) => Ok(web::Json(todos)),
        Err(e) => Err(ApiError::from(CommonError {
            message: e.message,
            code: 500,
        })),
    }
}

#[post("/create")]
pub async fn create_todo_handler(
    todo_repo: web::Data<dyn TodoRepository>,
    session: Session,
    post_data: web::Json<CreateTodoDTO>,
) -> Result<web::Json<Todo>, ApiError> {
    let session_id = session.get::<Uuid>("user_id").unwrap().unwrap();
    let create_todo = CreateTodo {
        description: post_data.description.clone(),
        session_id,
    };
    let new_todo = todo_repo.create(&create_todo).await;
    match new_todo {
        Ok(todo) => Ok(web::Json(todo)),
        Err(e) => Err(ApiError::from(CommonError {
            message: e.message,
            code: 500,
        })),
    }
}

#[delete("/delete/{todo_id}")]
pub async fn delete_todo_handler(
    req: HttpRequest,
    todo_repo: web::Data<dyn TodoRepository>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let session_id = session.get::<Uuid>("user_id").unwrap().unwrap();
    let todo_id: i32 = todo_id_extractor(&req).unwrap();

    let result = todo_repo.delete(todo_id, session_id).await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(ApiError::from(CommonError {
            message: e.message,
            code: 500,
        })),
    }
}

#[patch("/complete/{todo_id}")]
pub async fn complete_todo_handler(
    req: HttpRequest,
    todo_repo: web::Data<dyn TodoRepository>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let session_id = session.get::<Uuid>("user_id").unwrap().unwrap();
    let todo_id: i32 = todo_id_extractor(&req).unwrap();
    let result = todo_repo.complete_todo(todo_id, session_id).await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(ApiError::from(CommonError {
            message: e.message,
            code: 500,
        })),
    }
}

pub fn todo_controller() -> Scope {
    web::scope("/todo")
        .service(list_todos_handler)
        .service(create_todo_handler)
        .service(delete_todo_handler)
        .service(complete_todo_handler)
}

fn todo_id_extractor(req: &HttpRequest) -> Result<i32, ApiError> {
    let todo_id: i32 = match req.match_info().get("todo_id") {
        Some(id) => id.parse().unwrap(),
        None => {
            return Err(ApiError::from(CommonError {
                message: "No todo_id provided".to_string(),
                code: 500,
            }));
        }
    };
    Ok(todo_id)
}
