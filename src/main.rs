use actix_web::{web::scope, App, HttpServer};
mod data;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            scope("/api")
                .service(crate::routes::hello)
                .service(crate::routes::get_cars)
                .service(crate::routes::post_cars),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
