FROM rust:1.33

WORKDIR /usr/src/myapp
COPY . .
RUN rustup override set nightly
ENV ROCKET_ENV production
RUN cargo build --release
EXPOSE 80
ENTRYPOINT ["cargo", "run"]

CMD ["--release"]