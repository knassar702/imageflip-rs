use crate::start;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/{id}")]
async fn index(id: web::Path<i32>) -> impl Responder {
    web::Json(start(Some(*id)))
}

#[get("/")]
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("<a href=\"https://github.com/knassar702/imageflip-rs\"> imageflip-rs </a> API is running")
}


#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(index2)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
