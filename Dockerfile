# MercyOS-Pinnacle Docker Deployment – Multiplayer Server Fortress Sacred
# Multi-stage build for minimal image joy

FROM rust:1.76 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin powrush_mmo

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/powrush_mmo /usr/local/bin/powrush_mmo
EXPOSE 5000/udp  # Renet multiplayer port

CMD ["powrush_mmo", "--server"]
