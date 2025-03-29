FROM oven/bun:1 AS frontend-base
WORKDIR /usr/src/app

FROM frontend-base AS frontend-install
RUN mkdir -p /temp/dev
COPY frontend/package.json frontend/bun.lock /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

RUN mkdir -p /temp/prod
COPY frontend/package.json frontend/bun.lock /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

FROM frontend-base AS frontend-build
COPY --from=frontend-install /temp/dev/node_modules node_modules
COPY frontend/ .

ENV NODE_ENV=production
ENV VITE_API_URL=
RUN bun test
RUN bun run build

FROM rust:1.85.0 AS backend-base
WORKDIR /usr/src/app

FROM backend-base AS backend-base-chef
RUN cargo install cargo-chef

FROM backend-base-chef AS backend-planner
COPY backend .
RUN cargo chef prepare --recipe-path recipe.json

FROM backend-base AS backend-build
COPY --from=backend-planner /usr/src/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY backend .
COPY --from=frontend-build /usr/src/app/dist /temp/dev/frontend/dist
RUN cargo build --release

FROM ubuntu:24.04 AS base
WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

FROM base AS final
COPY --from=backend-build /usr/src/app/target/release/kura-indexer-nyaa kura-indexer-nyaa

