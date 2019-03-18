FROM rustlang/rust:nightly as builder
RUN apt-get update && \
    apt-get install musl-tools -y && \
    rustup target add x86_64-unknown-linux-musl
ARG DIR_NAME
WORKDIR /usr/src/${DIR_NAME}
COPY Cargo.toml Cargo.toml
RUN mkdir src/ && \
    echo "fn main(){}" > src/main.rs && \
    RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl && \
    rm -rf src/
COPY src src
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM rustlang/rust:nightly
ARG DIR_NAME
RUN apk add --no-cache libpq
WORKDIR /root/
COPY --from=builder /usr/src/${DIR_NAME}/target/x86_64-unknown-linux-musl/release/${DIR_NAME} app
ENV RUST_BACKTRACE 1
EXPOSE 8000
CMD ["./app"]
