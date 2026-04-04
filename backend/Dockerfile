FROM rust:1.80-slim AS builder

RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    libssl-dev \
    build-essential \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/backend

COPY Cargo.toml Cargo.lock ./
COPY . ./

RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
WORKDIR /usr/src/backend

EXPOSE 8080
CMD ["backend"]
