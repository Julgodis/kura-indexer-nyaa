FROM rust:1.85.0 AS backend-base
WORKDIR /usr/src/app

FROM backend-base AS backend-build
RUN mkdir -p /temp/dev
COPY Cargo.toml Cargo.lock /temp/dev/
COPY --from=frontend-release:latest ./dist ./dist
RUN cd /temp/dev && cargo fetch
RUN cd /temp/dev && cargo build --release

FROM backend-base AS backend-release
COPY --from=backend-build /temp/dev/target/release/kura-indexder-nyaa /usr/local/bin/kura-indexder-nyaa

