use crate::domain::dto::todo::{CreateTodo, Todo};
use crate::domain::repositories::repository::RepositoryResult;
use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, new_todo: &CreateTodo) -> RepositoryResult<Todo>;
    async fn list_by_session_id(&self, session_id: i32) -> RepositoryResult<Vec<Todo>>;
    async fn delete(&self, todo_id: i32, session_id: i32) -> RepositoryResult<()>;
}
