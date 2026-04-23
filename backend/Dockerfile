FROM rust:1.86-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    perl \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY entity ./entity
COPY migration ./migration
COPY src ./src
COPY templates ./templates

RUN cargo build --release

FROM rust:1.86-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    perl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/hackari-backend /app/
COPY --from=builder /app/templates /app/templates

EXPOSE 3000

CMD ["./hackari-backend"]
