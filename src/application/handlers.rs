use actix_web::{get, Responder, HttpResponse};

#[get("/hello1")]
pub async fn hello1() -> impl Responder {
    HttpResponse::Ok().body("Hello world1!")
}

#[get("/hello2")]
pub async fn hello2() -> impl Responder {
    HttpResponse::Ok().body("Hello world2!")
}