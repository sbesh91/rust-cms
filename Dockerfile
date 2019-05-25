ARG ARTIFACT

FROM rust:1.35.0

RUN rustup update nightly
RUN rustup default nightly
RUN cargo install diesel_cli

WORKDIR /usr/src/rust-cms
COPY . .

RUN cargo install --path .

CMD ["rust-cms"]