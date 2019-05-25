# rust-cms
A rustlang based api for cms use

# Install Diesel CLI with only postgres support
requires sudo apt install libpq-dev

ALTER USER postgres PASSWORD 'root';

echo DATABASE_URL=postgres://postgres:root@localhost:5432/cms_db > .env

login sudo -u postgres psql

cargo install diesel_cli --no-default-features --features postgres

diesel migration run

diesel print-schema > src/endpoints/lib/schema.rs

# Set up ENV secret > .env
SECRET=asdf123


