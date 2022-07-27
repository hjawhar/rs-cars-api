use std::sync::Mutex;
#[macro_use]
extern crate diesel;
use actix_web::{
    middleware::Logger,
    web::{scope, Data},
    App, HttpServer,
};

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = db::establish_connection();

    let data = Data::new(Mutex::new(connection));
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::clone(&data))
            .service(
                scope("/api")
                    .service(crate::routes::index)
                    .service(crate::routes::get_cars)
                    .service(crate::routes::post_cars),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
