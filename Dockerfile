FROM rust as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install pkg-config openssl -y
RUN cargo install --path .

CMD ["terceira-ponte-bot"]
