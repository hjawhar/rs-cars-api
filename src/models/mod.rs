use serde::{Deserialize, Serialize};
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
