FROM rust:latest

WORKDIR /app

COPY target/release/ha-auth-server .

CMD ["./ha-auth-server"]