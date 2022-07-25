use std::sync::Mutex;

use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};

use crate::models::{Car, CarPayload, GlobalData, ServerStatus};

#[get("/")]
pub async fn index() -> impl Responder {
    let server_status = ServerStatus { ok: true };
    HttpResponse::Ok().json(server_status)
}

#[get("/cars")]
pub async fn get_cars(data: web::Data<Mutex<GlobalData>>) -> impl Responder {
    let lock_guard = data.lock().unwrap();
    HttpResponse::Ok().json(&lock_guard.garage.cars)
}

#[post("/cars")]
pub async fn post_cars(
    info: Json<CarPayload>,
    data: web::Data<Mutex<GlobalData>>,
) -> impl Responder {
    let mut lock_guard = data.lock().unwrap();

    // println!("{} {}", info.name, info.model);
    // if info.name.trim().is_empty() {
    //     let server_error = ServerError::new("Name not found");
    //     println!("{:#?}", server_error);
    //     let serialized = serde_json::to_string(&server_error).unwrap();
    //     return HttpResponse::Ok().body(server_error.error);
    // }

    // if info.model.trim().is_empty() {
    //     let server_error = ServerError::new("Model not found");
    //     let serialized = serde_json::to_string(&server_error).unwrap();
    //     return HttpResponse::Ok().body(serialized);
    // }

    let car1 = Car {
        name: String::from(&info.name),
        model: String::from(&info.model),
    };

    lock_guard.garage.add(car1);
    HttpResponse::Ok().json(&lock_guard.garage.cars)
}
