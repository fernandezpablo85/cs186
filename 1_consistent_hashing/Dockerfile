# First stage: build the Rust program.
FROM rust:latest as builder
WORKDIR /usr/src

# Copy over your manifest and source files
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

# Build for release.
RUN cargo build --release

# Use a smaller image for the second stage.
FROM debian:buster-slim
COPY --from=builder /usr/src/target/release/bcache .

# Set the start command to our binary.
CMD ["./bcache", "server"]
