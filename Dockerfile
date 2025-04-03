
FROM rust:1.85.0 AS backend-base
WORKDIR /usr/src/app

FROM backend-base AS backend-base-chef
RUN cargo install cargo-chef

FROM backend-base-chef AS backend-planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM backend-base-chef AS backend-build
COPY --from=backend-planner /usr/src/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM ubuntu:24.04 AS base
WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

FROM base AS final
COPY --from=backend-build /usr/src/app/target/release/kura-indexer-nyaa kura-indexer-nyaa

ENTRYPOINT ["/usr/src/app/kura-indexer-nya", "--config", "/config/nyaa-indexer.toml"]
