FROM rust:latest as build
WORKDIR /usr/src
RUN apt-get update && apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
RUN useradd -u 1001 nonroot
RUN USER=root cargo new spendr-api
WORKDIR /usr/src/spendr-api
COPY Cargo.toml Cargo.lock ./
RUN cargo build --target x86_64-unknown-linux-musl --release
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /usr/local/cargo/bin/spendr-api .
USER nonroot
EXPOSE 8080
CMD ["./spendr-api"]
