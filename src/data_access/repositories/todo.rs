use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::query_dsl::positional_order_dsl::IntoOrderColumn;
use std::process::id;
use std::sync::Arc;

use crate::data_access::databases::sqlite::DBConn;
use crate::data_access::error::DieselRepositoryError;
use crate::data_access::models::todo::{CreateTodoDiesel, TodoDiesel};
use crate::domain::models::todo::{CreateTodo, Todo};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::todo::{TodoQueryParams, TodoRepository};

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
        let mut conn = self.pool.get().unwrap();
        let _ = diesel::insert_into(todos)
            .values(new_todo_diesel)
            .execute(&mut conn);

        let last_todo = todos
            .order(id.desc())
            .limit(1 as i64)
            .load(&mut conn)?
            .into_iter()
            .rev()
            .collect::<Vec<_>>();
        Ok(last_todo[0].into())
    }

    async fn list(&self, params: TodoQueryParams) -> RepositoryResult<ResultPaging<Todo>> {
        use crate::data_access::schema::todos::dsl::todos;
        let pool = self.pool.clone();
        let builder = todos.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<TodoDiesel>(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, todo_id: i32) -> RepositoryResult<Todo> {
        use crate::data_access::schema::todos::dsl::{id, todos};
        let mut conn = self.pool.get().unwrap();
        run(move || todos.filter(id.eq(todo_id)).first::<TodoDiesel>(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> Todo { v.into() })
    }

    async fn delete(&self, todo_id: i32) -> RepositoryResult<()> {
        use crate::data_access::schema::todos::dsl::{id, todos};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            diesel::delete(todos)
                .filter(id.eq(todo_id))
                .execute(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(())
    }
}
