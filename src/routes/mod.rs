use actix_web::{get, HttpResponse, Responder};
use serde_json;

use crate::{data::ALL_CARS, models::Car};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/cars")]
pub async fn cars() -> impl Responder {
    let car1 = Car {
        name: String::from("BMW"),
        model: String::from("1995"),
    };
    let car2 = Car {
        name: String::from("BMW"),
        model: String::from("1995"),
    };

    let car3 = Car {
        name: String::from("BMW"),
        model: String::from("1995"),
    };
    let car4 = Car {
        name: String::from("BMW"),
        model: String::from("1995"),
    };

    unsafe {
        ALL_CARS.push(car1);
        ALL_CARS.push(car2);
        ALL_CARS.push(car3);
        ALL_CARS.push(car4);
        let serialized = serde_json::to_string(&ALL_CARS).unwrap();

        HttpResponse::Ok().body(serialized)
    }
}
