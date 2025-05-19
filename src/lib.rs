use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    // println!("good!");
    HttpResponse::Ok().finish()
}

// async fn subscribe() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// input -> deserialize -> FormData
#[derive(serde::Deserialize)]
struct FormData{
    email: String,
    name: String
}

// public as this isnt our entrypoint anymore
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
        })
        // .bind(address)?
        .listen(listener)?
        .run();
        // .await()
    Ok(server)
}
