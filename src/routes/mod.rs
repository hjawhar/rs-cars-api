use actix_web::{get, post, HttpResponse, Responder};
use serde_json;

use crate::{data::ALL_CARS, models::Car};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/cars")]
pub async fn get_cars() -> impl Responder {
    unsafe {
        let serialized = serde_json::to_string(&ALL_CARS).unwrap();

        HttpResponse::Ok().body(serialized)
    }
}

#[post("/cars")]
pub async fn post_cars() -> impl Responder {
    unsafe {
        let car1 = Car {
            name: String::from("BMW"),
            model: String::from("1995"),
        };
        ALL_CARS.push(car1);
        let serialized = serde_json::to_string(&ALL_CARS).unwrap();
        HttpResponse::Ok().body(serialized)
    }
}
