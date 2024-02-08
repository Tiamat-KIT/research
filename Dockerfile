FROM rust:alpine3.18 as builder
WORKDIR /build

RUN apk update && \
    apk upgrade --no-cache && \
    apk add pkgconfig libressl musl-dev 

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked cargo-leptos

COPY . .

RUN cargo leptos build --release

FROM alpine:3.18 AS runner
WORKDIR /var/www/app

RUN addgroup -S server && \
	adduser -S www-data -G server && \
	chown -R www-data:server /var/www/app

COPY --chown=www-data:server --from=builder /build/target/server/release/research ./server/research
COPY --chown=www-data:server --from=builder /build/target/front/wasm32-unknown-unknown/release/research.wasm ./front/research.wasm
COPY --chown=www-data:server --from=builder /build/target/site ./site

USER www-data

ENV LEPTOS_OUTPUT_NAME "research"
ENV LEPTOS_SITE_ROOT "/var/www/app/site"
ENV LEPTOS_ENV "PROD"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"

EXPOSE 3000

CMD ["./server/portfolio"]