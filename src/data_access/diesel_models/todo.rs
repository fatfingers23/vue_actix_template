use crate::data_access::schema::todos;
use crate::domain::dto::todo::{CreateTodo, Todo};
use diesel;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct TodoDiesel {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub session_id: Uuid,
}

// Factory method for creating a new TodoDiesel from a Todo
impl From<Todo> for TodoDiesel {
    fn from(t: Todo) -> Self {
        TodoDiesel {
            id: t.id,
            description: t.description,
            completed: t.completed,
            session_id: t.session_id,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct CreateTodoDiesel {
    pub description: String,
    pub session_id: Uuid,
}

// Factory method for creating a new Todo from a TodoDiesel
impl Into<Todo> for TodoDiesel {
    fn into(self) -> Todo {
        Todo {
            id: self.id,
            description: self.description,
            completed: self.completed,
            session_id: self.session_id,
        }
    }
}

impl From<CreateTodo> for CreateTodoDiesel {
    fn from(t: CreateTodo) -> Self {
        CreateTodoDiesel {
            description: t.description,
            session_id: t.session_id,
        }
    }
}

impl Into<Todo> for CreateTodoDiesel {
    fn into(self) -> Todo {
        Todo {
            id: 0,
            description: self.description,
            completed: false,
            session_id: self.session_id,
        }
    }
}
