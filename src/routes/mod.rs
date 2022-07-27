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

    insert_into(cars)
        .values((name.eq(&info.name), model.eq(&info.model)))
        .execute(&*lock_guard)
        .unwrap();

    HttpResponse::Ok()
}
