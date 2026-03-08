# Build stage
FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy ironclaw-engram dependency first
COPY ironclaw-engram/ /app/ironclaw-engram/

# Copy usergrow source
COPY usergrow/ /app/usergrow/

WORKDIR /app/usergrow
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/usergrow/target/release/usergrow /usr/local/bin/geo-agent

# Copy frontend static files
COPY docs/ /app/static/

WORKDIR /app
EXPOSE 3000

CMD ["geo-agent", "--serve", "--port", "3000"]
