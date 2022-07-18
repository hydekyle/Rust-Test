use crate::models::Car;
use actix_web::{get, post};

// GET car
#[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Get a car", body = String)
        )
    )]
#[get("/car")]
pub(super) async fn get_car() -> String {
    let car = Car {
        marca: String::from("Seat"),
        matricula: String::from("4949XL"),
    };
    let serialized = serde_json::to_string(&car).unwrap();
    return format!("{}", serialized);
}

// GET random car
#[utoipa::path(
        path = "/api/car",
        responses(
            (status = 200, description = "Post a new car", body = String)
        )
    )]
#[post("/car")]
pub(super) async fn post_car() -> String {
    let car = Car {
        marca: "aaa".to_string(),
        matricula: "abc".to_string(),
    };
    //TODO: Save post value
    let serialized = serde_json::to_string(&car).unwrap();
    return serialized;
}

// GET random car
#[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Get a random new car", body = String)
        )
    )]
#[get("/car_random")]
pub(super) async fn get_car_random() -> String {
    let random_car = Car::get_random_car();
    let serialized = serde_json::to_string(&random_car).unwrap();
    return format!("{}", serialized);
}
