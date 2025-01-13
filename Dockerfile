FROM rust:latest AS builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    musl-tools

RUN apt-get install -y ca-certificates tzdata

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY . .

ENV OPENSSL_DIR=/usr/include/openssl
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /usr/share/zoneinfo /usr/share/zoneinfo

COPY ./src/infrastructure/local_db/credentials.json /src/infrastructure/local_db/credentials.json
COPY ./src/infrastructure/local_db/last_update.json /src/infrastructure/local_db/last_update.json

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/WeatherReport /WeatherReport

ENV RUST_LOG=debug

CMD ["/WeatherReport"]
