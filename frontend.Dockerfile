FROM oven/bun:1 AS frontend-base
WORKDIR /usr/src/app

FROM frontend-base AS frontend-install
RUN mkdir -p /temp/dev
COPY frontend/package.json frontend/bun.lock /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

RUN mkdir -p /temp/prod
COPY frontend/package.json frontend/bun.lock /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

FROM frontend-base AS frontend-prerelease
COPY --from=frontend-install /temp/dev/node_modules node_modules
COPY frontend/ .

ENV NODE_ENV=production
RUN bun test
RUN bun run build

FROM frontend-base AS frontend-release
COPY --from=frontend-prerelease /usr/src/app/dist ./dist
