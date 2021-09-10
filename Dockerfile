FROM rust:latest as build

USER root

WORKDIR /rust-counter-strings

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM rust:latest

COPY --from=build /rust-counter-strings/target/release/rust-counter-strings .

ENTRYPOINT ["./rust-counter-strings"]
