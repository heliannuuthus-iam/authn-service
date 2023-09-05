FROM rust:latest

WORKDIR /app

COPY target/release/forum-server .

CMD ["./forum-server"]