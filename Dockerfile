FROM rust:1-stretch as builder

WORKDIR /usr/src/app

RUN USER=root cargo init

COPY Cargo.toml .

RUN cargo build --release

COPY src src

RUN cargo build --release


# Final image
FROM debian:stretch-slim
COPY --from=builder /usr/src/app/target/release/gitlab-hook-deploy /bin/
CMD gitlab-hook-deploy
