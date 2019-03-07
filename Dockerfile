FROM rust:1.33

WORKDIR /usr/src/myapp
RUN rustup override set nightly
ENV ROCKET_ENV production

CMD ["cargo", "run"]