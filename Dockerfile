# Build Stage
FROM rust:1.77-alpine AS builder

RUN apk add --no-cache build-base pkgconfig openssl-dev

WORKDIR /app

COPY Cargo.toml ./

COPY . .
RUN cargo build --release

# Final Stage
FROM alpine:latest

WORKDIR /app

# OpenSSL for Alpine
RUN apk add --no-cache openssl ca-certificates

COPY --from=builder /app/target/release/api-processor /app/api-processor

EXPOSE 3000
CMD ["./api-processor"]