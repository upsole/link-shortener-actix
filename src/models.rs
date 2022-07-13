use actix_web::web;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel::{Insertable, Queryable};
use r2d2::{Pool, PooledConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::url_pair;

// Aliases for Connection Pool Types
pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Serialize, Deserialize)]
pub struct UrlPair {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub original_url: String,
    pub shortened_url: String,
}

impl UrlPair {
    fn new(id: String, created_at: NaiveDateTime, original_url: String, shortened_url: String) -> Self {
        Self{
            id,
            created_at,
            original_url,
            shortened_url
        }
    }
}

#[derive(Queryable, Insertable)]
#[table_name = "url_pair"]
pub struct UrlPairDB {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub original_url: String,
    pub short_url: String,
}

impl UrlPairDB {
    pub fn to_url_pair(&self) -> UrlPair {
        UrlPair::new(
            self.id.to_string(),
            self.created_at,
            self.original_url.to_string(),
            self.short_url.to_string()
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UrlRequestBody {
    url: String,
}

// TODO refactor to TryForm
// struct ParseError;
pub struct ParsedPayload(web::Json<UrlRequestBody>);
impl From<web::Json<UrlRequestBody>> for ParsedPayload {
    fn from(value: web::Json<UrlRequestBody>) -> ParsedPayload {
        ParsedPayload(value)
    }
}

use rand::{distributions::Alphanumeric, thread_rng, Rng};
fn gen_short_url(length: u16) -> String {
    let mut thread = thread_rng();
    (0..length)
        .map(|_| thread.sample(Alphanumeric) as char)
        .collect::<String>()
}

impl ParsedPayload {
    pub fn to_url_db(&self) -> UrlPairDB {
        UrlPairDB {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            original_url: self.0.url.clone(),
            short_url: gen_short_url(8),
        }
    }
}
