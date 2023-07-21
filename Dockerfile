FROM rust:slim

WORKDIR /app
COPY . .

RUN cargo install --path .

CMD ["terceira-ponte-bot"]
