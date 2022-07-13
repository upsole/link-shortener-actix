use actix_web::{web, App, HttpServer};

#[macro_use]
extern crate diesel;

pub mod models;
pub mod routes;
pub mod schema;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use r2d2::Pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Build connection poool
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pol");

    // Run Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::ok)
            .service(routes::create)
            .service(routes::redirect)
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
