use std::sync::Mutex;

use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};
use db::establish_connection;
use models::{Garage, GlobalData};
use sqlx::{Pool, Postgres};
mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _db_connection = establish_connection().await;
    let pool: Pool<Postgres>;
    match _db_connection {
        Ok(_pool) => {
            pool = _pool;
            println!("Successfully connected to DB")
        }
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let global_data = GlobalData {
        garage: Garage::new(),
        db: pool,
    };

    let data = Data::new(Mutex::new(global_data));

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
