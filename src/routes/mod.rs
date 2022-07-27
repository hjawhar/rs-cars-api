use crate::*;
use std::sync::Mutex;

use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};

use diesel::{prelude::*, *};

use models::{Car, NewCar};
use schema::{self, cars::dsl::*};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/cars")]
pub async fn get_cars(data: web::Data<Mutex<PgConnection>>) -> impl Responder {
    let lock_guard = data.lock().unwrap();
    let cars_vec = cars.load::<Car>(&*lock_guard).expect("Error loading cars");
    HttpResponse::Ok().json(cars_vec)
}

#[post("/cars")]
pub async fn post_cars(info: Json<NewCar>, data: web::Data<Mutex<PgConnection>>) -> impl Responder {
    let lock_guard = data.lock().unwrap();
    let car_name: String;
    let car_model: String;

    match &info.name {
        Some(car_name_value) => car_name = car_name_value.to_string(),
        None => return HttpResponse::BadRequest().body("No car name specified"),
    }

    match &info.model {
        Some(car_model_value) => car_model = car_model_value.to_string(),
        None => return HttpResponse::BadRequest().body("No car model specified"),
    }

    if car_name.len() > 100 {
        return HttpResponse::BadRequest().body("Car name cannot exceed 100 characters");
    }

    if car_model.len() > 100 {
        return HttpResponse::BadRequest().body("Car model cannot exceed 100 characters");
    }

    insert_into(cars)
        .values((name.eq(&car_name), model.eq(&car_model)))
        .execute(&*lock_guard)
        .unwrap();

    HttpResponse::Ok().body("Successfully added car")
}
