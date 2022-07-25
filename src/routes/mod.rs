use std::sync::{Arc, Mutex};

use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json;

use crate::models::{Car, Garage};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/cars")]
pub async fn get_cars(data: web::Data<Arc<Mutex<Garage>>>) -> impl Responder {
    let f = data.lock().unwrap();
    let serialized = serde_json::to_string(&f.cars).unwrap();
    HttpResponse::Ok().body(serialized)
}

#[post("/cars")]
pub async fn post_cars(data: web::Data<Arc<Mutex<Garage>>>) -> impl Responder {
    let mut f = data.lock().unwrap();
    let car1 = Car {
        name: String::from("BMW"),
        model: String::from("1995"),
    };

    f.add(car1);
    let serialized = serde_json::to_string(&f.cars).unwrap();
    HttpResponse::Ok().body(serialized)
}
