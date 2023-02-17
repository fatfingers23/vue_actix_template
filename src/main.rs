use actix_files::{Files, NamedFile};
use actix_web::middleware::Logger;
use actix_web::{guard, Error, HttpServer};
use actix_web::{web, App};
use dotenv::dotenv;
use vue_actix_template::api::controllers::hello_world_controller::{
    repeat_handler, say_hello_handler,
};
use vue_actix_template::api::controllers::todo_controller::list_todos_handler;
use vue_actix_template::container::Container;

// #[cfg(test)]
// mod tests;

async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./spa/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let container = Container::new().await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(container.todo_repository.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(say_hello_handler)
                    .service(repeat_handler)
                    .service(web::scope("/todo").service(list_todos_handler)),
            )
            .service(Files::new("/", "./spa").index_file("index.html"))
            .default_service(web::route().guard(guard::Not(guard::Get())).to(index))
    })
    .bind(("127.0.0.1", 8080));
    server.unwrap().run().await
}
