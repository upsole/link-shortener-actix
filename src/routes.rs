use actix_web::{get, post, web, HttpResponse};
use diesel::prelude::*;

use crate::models::{DBPool, DBPooledConnection, ParsedPayload, UrlPair, UrlRequestBody};

#[get("/ok")]
pub async fn ok() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/short")]
pub async fn create(pool: web::Data<DBPool>, req: web::Json<UrlRequestBody>) -> HttpResponse {
    // Sanitizes payload
    let parsed_req = req.into();

    let conn = pool.get().expect("Could not connect to DB");
    let new_acct = web::block(move || create_short_pair(&conn, parsed_req))
        .await
        .unwrap();
    HttpResponse::Ok().json(new_acct)
}

fn create_short_pair(conn: &DBPooledConnection, payload: ParsedPayload) -> UrlPair {
    use crate::schema::url_pair::dsl::*;
    let payload_db = payload.to_url_db();
    let _ = diesel::insert_into(url_pair)
        .values(&payload_db)
        .execute(conn)
        .expect("Insert failed");
    // TODO cast payload_db into url_pair
    let new_url_pair = payload_db.to_url_pair();
    // Ok(url_pair)
    new_url_pair
}
