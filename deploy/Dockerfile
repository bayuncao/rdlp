# Start from the latest Rust base image
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin myproject
WORKDIR /rdlp

# Copy our source code
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Build our application
RUN cargo build --release

# Start a new stage
FROM debian:buster-slim

# Copy the build artifact from the builder stage and set the executable as the command
COPY --from=builder /rdlp/target/release/rdlp /usr/local/bin

# Set the startup command to run the binary
CMD ["rdlp"]