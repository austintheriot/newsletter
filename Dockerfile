FROM rust:1.63.0

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
# tells sqlx to use the cached sqlx-data.json file to check
# compile-time queries while building the project
ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production
RUN cargo build --release
ENTRYPOINT ["./target/release/newsletter-api"]