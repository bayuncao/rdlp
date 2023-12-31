# This is a pre-built Dockerfile, which may change along with updates to the rdlp project.
# If you intend to run rdlp in a production environment in a containerized manner,
# please ensure that you are proficient in the operations of building containerized Rust applications.
FROM rust:1.75-slim-bookworm as builder

WORKDIR /usr/src/rdlp

COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies

COPY --from=builder /usr/local/cargo/bin/rdlp /usr/local/bin/rdlp

CMD ["rdlp"]