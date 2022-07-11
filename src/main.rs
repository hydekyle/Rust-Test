use std::sync::Arc;

use ntex::channel::mpsc;
use ntex::http::{StatusCode, Uri};
use ntex::util::Bytes;
use ntex::web::{self, get, App, Error, HttpResponse};
use utoipa::openapi::Response;
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

#[derive(OpenApi)]
#[openapi(handlers(api1::hello1))]
struct ApiDoc1;

#[derive(OpenApi)]
#[openapi(handlers(api2::hello2))]
struct ApiDoc2;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::server(|| {
        App::new().service((
            web::resource(("/hola/{name}")).route(web::get().to(response_body)),
            web::resource(("/hola")).route(web::get().to(response_body)),
            // web::resource(("/api-doc1.json")).route(web::get().to(handler)),
            web::resource(("/swagger")).route(web::get().to(serve_swagger)),
        ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn response_body(path: web::types::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);
    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));
    HttpResponse::Ok().streaming(rx_body)
}

async fn serve_swagger() -> HttpResponse {
    let config = Arc::new(Config::new(["/api-doc1.json", "/api-doc2.json"]));
    let tail = "/";

    match utoipa_swagger_ui::serve(tail, config) {
        Ok(swagger_file) => swagger_file
            .map(|file| {
                HttpResponse::Ok()
                    .content_type(file.content_type)
                    .body(file.bytes.to_vec())
            })
            .unwrap_or_else(|| HttpResponse::NotFound().finish()),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

mod api1 {

    #[utoipa::path(
        get,
        path = "/hello1",
        responses(
            (status = 200, body = String)
        )
    )]
    pub(super) async fn hello1() -> String {
        "hello from api 1".to_string()
    }
}

mod api2 {
    #[utoipa::path(
        get,
        path = "/hello2",
        responses(
            (status = 200, body = String)
        )
    )]
    pub(super) async fn hello2() -> String {
        "hello from api 2".to_string()
    }
}
