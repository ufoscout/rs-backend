use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    // Application status
    let db = Arc::new(Mutex::new(HashMap::<String, User>::new()));

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

