-- Your SQL goes here
-- TODO collect metadata
CREATE TABLE url_pair (
  id UUID PRIMARY KEY,
  created_at TIMESTAMP NOT NULL,
  original_url VARCHAR(255) NOT NULL,
  short_url VARCHAR(255) NOT NULL
)
