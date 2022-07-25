use std::sync::{Arc, Mutex};

use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};
use models::Garage;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let arc = Arc::new(Mutex::new(Garage::new()));
    let data = Data::new(arc);

    HttpServer::new(move || {
        App::new().app_data(data.clone()).service(
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
