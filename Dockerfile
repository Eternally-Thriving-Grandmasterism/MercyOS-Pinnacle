# MercyOS-Pinnacle Multi-Stage Docker Fortress Sacred
# Builder stage: compile release joy
FROM rust:1.76 as builder
WORKDIR /app

# Cache deps mercy
COPY Cargo.toml Cargo.lock ./
COPY crates/*/Cargo.toml crates/*/
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Real source copy + build
COPY . .
RUN cargo build --release --bin powrush_mmo

# Runtime stage: slim debian mercy minimal
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/powrush_mmo /usr/local/bin/powrush_mmo

# Multiplayer port mercy
EXPOSE 5000/udp

# Default server mode sacred
CMD ["powrush_mmo", "--server"]
