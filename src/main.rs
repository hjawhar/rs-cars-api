use actix_web::{App, HttpServer};
mod routes;
mod models;
mod data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(crate::routes::hello)
            .service(crate::routes::get_cars)
            .service(crate::routes::post_cars)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
