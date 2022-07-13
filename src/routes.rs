use actix_web::{get, post, web, HttpResponse};
use diesel::prelude::*;

use crate::models::{
    DBPool, DBPooledConnection, ParsedPayload, UrlPair, UrlRequestBody,
};

#[get("/ok")]
pub async fn ok() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/shorten")]
pub async fn create(pool: web::Data<DBPool>, req: web::Json<UrlRequestBody>) -> HttpResponse {
    // Sanitizes payload
    let parsed_req = req.into();

    let conn = pool.get().expect("Could not connect to DB");
    let new_acct = web::block(move || create_short_pair(&conn, parsed_req))
        .await
        .unwrap();
    // TODO should returned full shortened URL
    HttpResponse::Created().json(new_acct)
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

#[get("/redirect/{short_url}")]
pub async fn redirect(path: web::Path<String>, pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect("Could not connect to DB");
    let short_url = path.into_inner();
    let original_url = web::block(move || redirect_query(&conn, &short_url))
        .await
        .map_err(|_| return HttpResponse::BadRequest().finish());
    HttpResponse::TemporaryRedirect()
        .insert_header(("Location", original_url.unwrap()))
        .finish()
}

fn redirect_query(conn: &DBPooledConnection, url: &str) -> String {
    use crate::schema::url_pair::dsl::*;
    let target_url = url_pair
        .filter(short_url.eq(url))
        .select(original_url)
        .first(conn)
        .unwrap();
    target_url
    // original_url.original_url
}
