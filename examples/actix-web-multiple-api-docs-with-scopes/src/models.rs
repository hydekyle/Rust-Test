use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    pub marca: String,
    pub matricula: String,
}

impl Car {
    pub fn get_random_car() -> Car {
        Car {
            marca: "".to_string(),
            matricula: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u8,
    pub name: String,
}
