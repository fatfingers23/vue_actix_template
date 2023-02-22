use crate::data_access::databases::postgresql::db_pool;
use crate::data_access::repositories::todo::TodoDieselRepository;
use crate::domain::repositories::todo::TodoRepository;
use std::sync::Arc;

pub struct Container {
    pub todo_repository: Arc<dyn TodoRepository>,
}

impl Container {
    pub async fn new() -> Self {
        let db_pool = db_pool().await;
        let todo_repository: Arc<dyn TodoRepository> =
            Arc::new(TodoDieselRepository::new(Arc::new(db_pool)));
        Container { todo_repository }
    }
}
