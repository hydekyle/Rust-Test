use actix_web::{middleware::Logger, web, App, HttpServer};
use std::{error::Error, net::Ipv4Addr};
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
mod api;
mod models;

#[tokio::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();
    #[derive(OpenApi)]
    #[openapi(handlers(api::get_car, api::get_car_random, api::post_car))]
    struct ApiDoc1;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(api::get_car)
                    .service(api::get_car_random)
                    .service(api::post_car),
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
