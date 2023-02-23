use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{guard, Error, HttpServer};
use actix_web::{web, App};
use dotenv::dotenv;
use std::env;
use vue_actix_template::api::controllers::hello_world_controller::{
    repeat_handler, say_hello_handler,
};
use vue_actix_template::api::controllers::todo_controller::todo_controller;
use vue_actix_template::container::Container;
use vue_actix_template::middleware::get_user_id::GetUserId;

async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./spa/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let container = Container::new().await;

    let session_key = env::var("SESSION_KEY").expect("SESSION_KEY not set!");
    let session_key = Key::from(session_key.as_bytes());

    let ssl: bool = match env::var("SSL") {
        Ok(val) => val.parse::<bool>().unwrap(),
        Err(_e) => false,
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(container.todo_repository.clone()))
            .wrap(Logger::default())
            .wrap(GetUserId)
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), session_key.clone())
                    .cookie_path("/".to_string())
                    .cookie_http_only(!ssl)
                    .cookie_secure(ssl)
                    .build(),
            )
            .service(
                web::scope("/api")
                    .service(say_hello_handler)
                    .service(repeat_handler)
                    .service(todo_controller()),
            )
            .service(Files::new("/", "./spa").index_file("index.html"))
            .default_service(web::route().guard(guard::Not(guard::Get())).to(index))
    })
    .bind(("0.0.0.0", 8080));
    server.unwrap().run().await
}
