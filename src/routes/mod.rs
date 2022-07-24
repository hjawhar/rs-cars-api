use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    name: String,
    model: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/cars")]
pub async fn cars() -> impl Responder {
    let car = Car {
        name: String::from("BMW"),
        model: String::from("1995"),
    };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&car).unwrap();

    HttpResponse::Ok().body(serialized)
}
