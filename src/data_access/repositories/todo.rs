use crate::data_access::databases::postgresql::DBConn;
use crate::data_access::diesel_models::todo::{CreateTodoDiesel, TodoDiesel};
use crate::data_access::error::DieselRepositoryError;
use crate::data_access::schema::todos::{completed, session_id};
use crate::domain::dto::todo::{CreateTodo, Todo};
use crate::domain::repositories::repository::RepositoryResult;
use crate::domain::repositories::todo::TodoRepository;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;

use diesel_async::RunQueryDsl;
use std::sync::Arc;
use uuid::Uuid;

pub struct TodoDieselRepository {
    pub pool: Arc<DBConn>,
}

impl TodoDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        TodoDieselRepository { pool: db }
    }
}

#[async_trait]
impl TodoRepository for TodoDieselRepository {
    async fn create(&self, new_todo: &CreateTodo) -> RepositoryResult<Todo> {
        use crate::data_access::schema::todos::dsl::todos;
        let new_todo_diesel: CreateTodoDiesel = CreateTodoDiesel::from(new_todo.clone());
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        let result: TodoDiesel = diesel::insert_into(todos)
            .values(new_todo_diesel)
            .get_result(&mut conn)
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        Ok(result.into())
    }

    async fn list_by_session_id(&self, id: Uuid) -> RepositoryResult<Vec<Todo>> {
        use crate::data_access::schema::todos::dsl::todos;
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        let result = todos
            .filter(session_id.eq(id))
            .load::<TodoDiesel>(&mut conn)
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into_iter().map(|v| v.into()).collect())
    }

    async fn delete(&self, todo_id: i32, todo_session_id: Uuid) -> RepositoryResult<()> {
        use crate::data_access::schema::todos::dsl::{id, todos};
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        diesel::delete(todos)
            .filter(id.eq(todo_id))
            .filter(session_id.eq(todo_session_id))
            .execute(&mut conn)
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(())
    }

    async fn complete_todo(&self, todo_id: i32, user_session_id: Uuid) -> RepositoryResult<()> {
        use crate::data_access::schema::todos::dsl::{id, todos};
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;

        diesel::update(todos)
            .set(completed.eq(true))
            .filter(id.eq(todo_id))
            .filter(session_id.eq(user_session_id))
            .execute(&mut conn)
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(())
    }
}
