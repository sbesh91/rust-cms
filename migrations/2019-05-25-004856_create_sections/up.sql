-- Your SQL goes here
CREATE TABLE sections (
  id SERIAL PRIMARY KEY,
  module VARCHAR NOT NULL,
  href VARCHAR NOT NULL,
  section_type VARCHAR NOT NULL
);