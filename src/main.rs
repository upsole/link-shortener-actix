use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/ok")]
async fn ok() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ok))
        .bind("127.0.0.1:4000")?
        .run()
        .await
}
