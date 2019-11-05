FROM rust:1-stretch

WORKDIR /usr/src/app

RUN USER=root cargo init

COPY Cargo.toml .

RUN cargo build --release

COPY src src

RUN cargo build --release

CMD RUST_LOG=hello_world::web=debug /usr/src/app/target/release/gitlab-hook-deploy
