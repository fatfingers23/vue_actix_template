use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub session_id: i32,
}

#[derive(Clone)]
pub struct CreateTodo {
    pub description: String,
    pub session_id: i32,
}
