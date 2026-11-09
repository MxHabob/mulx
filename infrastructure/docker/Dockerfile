FROM rust:1.97.1-slim

WORKDIR /app
COPY . .

RUN cargo build --release --manifest-path Cargo.toml

EXPOSE 3000

CMD ["cargo", "run", "--release", "--manifest-path", "backend/api/Cargo.toml"]
