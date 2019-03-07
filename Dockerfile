FROM rust:1.33

WORKDIR /usr/src/myapp
RUN rustup override set nightly

CMD ["cargo", "run"]