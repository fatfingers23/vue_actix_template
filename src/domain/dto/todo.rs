use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub session_id: Uuid,
}

#[derive(Clone, Deserialize)]
pub struct CreateTodo {
    pub description: String,
    pub session_id: Uuid,
}

#[derive(Clone, Deserialize)]
pub struct CreateTodoDTO {
    pub description: String
}