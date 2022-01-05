use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{App, HttpServer};
use crate::application::handlers::{hello1, hello2};
use crate::persistence::data_context::DataContext;

#[actix_web::main]
pub async fn run() -> std::io::Result<()>{
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec!["GET","POST","PUT","DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .data(DataContext::new())
            .wrap(cors)
            .service(hello1)
            .service(hello2)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}