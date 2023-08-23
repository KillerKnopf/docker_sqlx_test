# First build image from base rust image
FROM rust:1.71.1 as builder

# Create new empty rust project and cd into new project folder
RUN USER=root cargo new --bin docker_sqlx_test
WORKDIR /docker_sqlx_test

# Copy Cargo.lock and Cargo.toml file from real project on my machine into docker image
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build the emtpy cargo project which will already download the crates from crates.io
# compile and cache them.
# This way if the dependencies don't change they won't be rebuild each time the source code changes
RUN cargo build --release
# Remove contents from src folder (main.rs with Hello World)
RUN rm src/*.rs

# Copy the real source code into docker image
COPY ./src ./src

# Remove previous build (Hello World) and build again with actual project
RUN rm ./target/release/deps/docker_sqlx_test*
RUN cargo build --release

# Final base image for reducing image size because th€rust images is over 1GB
# Debain is an linux distribution and buster-slim is a release that stripped of most unnecessary stuff
# This result in a very small container
FROM debian:stable-slim

# Copy the build artifact from the first layer into this final debian layer
COPY --from=builder /docker_sqlx_test/target/release/docker_sqlx_test .

# Startup command to run program
CMD ["./docker_sqlx_test"]