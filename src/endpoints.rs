use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/index")]
pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}