FROM rust:slim AS builder

RUN apk add --no-cache musl-dev build-base

WORKDIR /app

COPY Cargo.toml .
COPY src ./src
COPY templates ./templates
COPY locales ./locales

RUN cargo build --release

COPY static ./static

FROM alpine:3.22

RUN apk add --no-cache curl bash mc net-tools unzip gzip

WORKDIR /app

COPY --from=builder /app/target/release/engineers_incubator_site .
COPY --from=builder /app/static ./static

EXPOSE 8080

ENTRYPOINT ["./engineers_incubator_site"]
