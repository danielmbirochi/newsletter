FROM rust:1.65.0

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

ENV APP_ENV production

EXPOSE 3000

ENTRYPOINT ["./target/release/newsletter"]