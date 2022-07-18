use actix_web::{middleware::Logger, web, App, HttpServer};
use std::{error::Error, net::Ipv4Addr};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
mod api_car;
mod api_pokemon;
mod models;

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();
    #[derive(OpenApi)]
    #[openapi(handlers(api_car::get_car, api_car::get_car_random, api_car::post_car))]
    #[openapi(handlers(api_pokemon::get_pokemon))]
    struct ApiDoc1;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/pokemon").service(api_pokemon::get_pokemon))
            .service(
                web::scope("/cars")
                    .service(api_car::get_car)
                    .service(api_car::get_car_random)
                    .service(api_car::post_car),
            )
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
                Url::new("api", "/api-doc/openapi1.json"),
                ApiDoc1::openapi(),
            )]))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
