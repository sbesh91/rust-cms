-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  account_name VARCHAR NOT NULL,
  password_hash VARCHAR NOT NULL
);