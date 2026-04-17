FROM rust:1.85-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY entity ./entity
COPY migration ./migration
COPY src ./src

RUN cargo build --release

FROM rust:1.85-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/hackari-backend /app/

EXPOSE 3000

CMD ["./hackari-backend"]
