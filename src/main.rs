use ntex::channel::mpsc;
use ntex::util::Bytes;
use ntex::web::{self, App, Error, HttpResponse};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::server(|| {
        App::new().service((
            web::resource(("/hola/{name}")).route(web::get().to(response_body)),
            web::resource(("/hola")).route(web::get().to(response_body)),
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
