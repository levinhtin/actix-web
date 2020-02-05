use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
}

async fn greet(req: HttpRequest) -> Result<HttpResponse> {
    let name = req.match_info().get("name").unwrap_or("World");
    let person = Person {
        name: format!("{}", name)
    };

    Ok(HttpResponse::Ok().json(person))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}