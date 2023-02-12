use crate::data_access::databases::postgresql::db_pool;
use crate::domain::repositories::todo::TodoRepository;
use std::sync::Arc;
// use crate::domain::services::service_context::ServiceContextService;
// use crate::domain::services::todo::TodoService;
// use crate::data_access::databases::postgresql::db_pool;
use crate::data_access::repositories::todo::TodoDieselRepository;
// use crate::data_access::services::service_context::ServiceContextServiceImpl;
// use crate::services::todo::TodoServiceImpl;

pub struct Container {
    // pub todo_service: Arc<dyn TodoService>,
    // pub service_context_service: Arc<dyn ServiceContextService>
}

impl Container {
    pub fn new() -> Self {
        let todo_repository: Arc<dyn TodoRepository> =
            Arc::new(TodoDieselRepository::new(Arc::new(db_pool())));
        // let todo_service = Arc::new(
        //     TodoServiceImpl { repository: todo_repository }
        // );
        // let service_context_service = Arc::new(
        //     ServiceContextServiceImpl::new(Arc::new(db_pool()))
        // );
        Container {}
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
