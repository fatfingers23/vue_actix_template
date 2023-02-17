use crate::api::controllers::hello_world_controller::{repeat_handler, say_hello_handler};
use crate::api::controllers::todo_controller::list_todos_handler;
use actix_files::{Files, NamedFile};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::web::service;
use actix_web::{guard, Error, HttpRequest};
use actix_web::{web, App};

use crate::container::Container;

async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./spa/index.html")?)
}

pub async fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    // let todo_service = container.todo_service.clone();
    // let service_context_service = container.service_context_service.clone();
    let container = Container::new().await;

    App::new()
        .app_data(web::Data::from(container.todo_repository))
        .wrap(Logger::default())
        .service(
            web::scope("/api")
                .service(say_hello_handler)
                .service(repeat_handler)
                .service(web::scope("/todo").service(list_todos_handler)),
        )
        .service(Files::new("/", "./spa").index_file("index.html"))
        .default_service(web::route().guard(guard::Not(guard::Get())).to(index))
}
