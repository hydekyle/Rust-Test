use crate::models::Car;
use actix_web::{get, post};

// GET pokemon by ID
#[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Get a pokemon", body = String)
        )
    )]
#[get("/pokemon")]
pub(super) async fn get_pokemon() -> String {
    let resp = reqwest::get("https://pokeapi.co/api/v2/pokemon/ditto")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    return resp;
}
