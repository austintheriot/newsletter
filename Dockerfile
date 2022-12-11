FROM rust:1.63.0-slim AS chef
WORKDIR /app
# We only pay the installation cost once for installing cargo-chef, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef && apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
# this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
# Only need this environment variable while compiling
# to tell our sqlx to use the cached version of our sqlx-data.json file
ENV SQLX_OFFLINE true
RUN cargo build --release --bin newsletter-api

# Using a very slim OS here, so some things need to be installed manually below
# Optionally, we could use `FROM debian:buster-slim AS runtime` instead without the dependency headache
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSLL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/newsletter-api newsletter-api
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./newsletter-api"]