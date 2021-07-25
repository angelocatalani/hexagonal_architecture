# compute a lock-like file for our project
FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

# build only the  project dependencies
FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# build our application, leveraging the cached deps!
FROM rust:1.52 AS builder
WORKDIR app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .

RUN cargo build --release --bin pokedex

# runtime stage
FROM debian:buster-slim AS runtime
WORKDIR app
# install OpenSSL because it is dynamically linked by some of our dependencies
RUN apt-get update -y \
    && apt-get -y install ca-certificates libssl-dev \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# the binary is statically compiled
COPY --from=builder /app/target/release/pokedex pokedex
COPY configuration configuration
ENV APP_APPLICATION__HOST "0.0.0.0"
EXPOSE 8080
ENTRYPOINT ["./pokedex"]
