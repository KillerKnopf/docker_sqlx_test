FROM rust:1.71-alpine3.18 as builder
WORKDIR /src
COPY . .
RUN cargo install --path .

FROM alpine3
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/docker_sqlx_test /usr/local/bin/docker_sqlx_test
CMD [ "docker_sqlx_test" ]