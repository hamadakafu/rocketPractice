FROM rust:latest as builder
ARG DIR_NAME
ENV USER=root
LABEL build_state=true
RUN apt-get update && \
    apt-get install musl-tools -y && \
    rustup override set nightly && \
    rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/${DIR_NAME}
RUN mkdir src/
RUN echo "fn main(){println!(\"if you see this, the build broke\")}" > src/main.rs
COPY Cargo.toml Cargo.toml
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -rf /usr/src/${DIR_NAME}/target/x86_64-unknown-linux-musl/release/${DIR_NAME}
RUN rm -f /usr/src/${DIR_NAME}/target/x86_64-unknown-linux-musl/release/deps/${DIR_NAME}*
RUN rm -rf src/
COPY src src
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.9
ARG DIR_NAME
RUN apk add --no-cache libpq
WORKDIR /root/
COPY --from=builder /usr/src/${DIR_NAME}/target/x86_64-unknown-linux-musl/release/${DIR_NAME} app
EXPOSE 8000
ENV RUST_BACKTRACE 1
ENV ROCKET_ENV=production
CMD ["./app"]
