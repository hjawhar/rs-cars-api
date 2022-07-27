use crate::*;
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use std::sync::Mutex;
use validator::{Validate, ValidationError};

use diesel::{prelude::*, *};

use models::{Car, NewCar};
use schema::{self, cars::dsl};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/cars")]
pub async fn get_cars(data: web::Data<Mutex<PgConnection>>) -> impl Responder {
    let lock_guard = data.lock().unwrap();
    let cars_vec = dsl::cars
        .load::<Car>(&*lock_guard)
        .expect("Error loading cars");
    HttpResponse::Ok().json(cars_vec)
}

#[post("/cars")]
pub async fn post_cars(info: Json<NewCar>, data: web::Data<Mutex<PgConnection>>) -> impl Responder {
    let lock_guard = data.lock().unwrap();

    let ser_data = info.into_inner();

    match ser_data.validate() {
        Ok(_) => {
            insert_into(dsl::cars)
                .values((
                    dsl::name.eq(&ser_data.name.unwrap()),
                    dsl::model.eq(&ser_data.model.unwrap()),
                ))
                .execute(&*lock_guard)
                .unwrap();

            HttpResponse::Ok().body("Successfully added car")
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
