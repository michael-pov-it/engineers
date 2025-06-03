FROM rust:1-alpine3.20 AS builder

RUN apk add --no-cache musl-dev build-base pkgconfig openssl-dev perl

WORKDIR /app

COPY Cargo.toml .

COPY src ./src

RUN cargo build --release

COPY static ./static

FROM alpine:3.20

RUN apk add --no-cache curl bash mc net-tools unzip gzip openssl

WORKDIR /app

COPY --from=builder /app/target/release/engineers_incubator_site .

COPY --from=builder /app/static ./static

EXPOSE 8080

ENTRYPOINT ["./engineers_incubator_site"]
