FROM oven/bun:1 AS frontend-base
WORKDIR /usr/src/app

FROM base AS frontend-install
RUN mkdir -p /temp/dev
COPY package.json bun.lock /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

RUN mkdir -p /temp/prod
COPY package.json bun.lock /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

FROM base AS frontend-prerelease
COPY --from=frontend-install /temp/dev/node_modules node_modules
COPY frontend/ .

ENV NODE_ENV=production
RUN bun test
RUN bun run build

FROM base AS frontend-release
COPY --from=frontend-prerelease /usr/src/app/dist ./dist
