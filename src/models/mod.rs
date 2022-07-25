use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    pub name: String,
    pub model: String,
}

pub struct Garage {
    pub cars: Vec<Car>,
}

impl Garage {
    pub fn new() -> Garage {
        Garage { cars: Vec::new() }
    }

    pub fn add(&mut self, car: Car) {
        self.cars.push(car);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerStatus {
    pub ok: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerError {
    pub ok: bool,
    pub error: String,
}

// impl ServerError {
//     pub fn new(error: &str) -> ServerError {
//         ServerError {
//             ok: false,
//             error: String::from(error),
//         }
//     }
// }

#[derive(Deserialize, Debug)]
pub struct CarPayload {
    pub name: String,
    pub model: String,
}

pub struct GlobalData {
    pub garage: Garage,
    pub db: Pool<Postgres>,
}
