use crate::api::controllers::hello_world_handler::{repeat_handler, say_hello_handler};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::Error;
use actix_web::{web, App};
// use crate::api::controllers::todo_handler::{create_todo_handler, delete_todo_handler, get_todo_handler, list_todos_handler};
// use crate::api::middleware::{ServiceContextMaintenanceCheck};
use crate::container::Container;

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let container = Container::new();
    // let todo_service = container.todo_service.clone();
    // let service_context_service = container.service_context_service.clone();

    App::new()
        // .app_data(web::Data::from(todo_service.clone()))
        // .app_data(web::Data::from(service_context_service.clone()))
        .wrap(Logger::default())
        .service(
            web::scope("/api")
                .service(say_hello_handler)
                .service(repeat_handler),
        )
}
