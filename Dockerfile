FROM rust:1.61.0-alpine AS builder

RUN apk update
RUN apk add pkgconfig openssl openssl-dev musl-dev

RUN rustup target add aarch64-unknown-linux-musl
RUN rustup toolchain install stable-aarch64-unknown-linux-musl

COPY . /app
WORKDIR /app

RUN cargo build --target=x86_64-unknown-linux-musl --release

FROM scratch
#FROM debian

COPY --from=builder /app/target/release/rust-practice /
CMD ["./rust-practice"]




