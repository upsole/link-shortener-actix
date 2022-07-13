table! {
    url_pair (id) {
        id -> Uuid,
        created_at -> Timestamp,
        original_url -> Varchar,
        short_url -> Varchar,
    }
}
