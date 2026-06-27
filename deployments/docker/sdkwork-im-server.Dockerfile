# syntax=docker/dockerfile:1

FROM rust:1.85-bookworm AS builder
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY services ./services
COPY adapters ./adapters
COPY apis ./apis
RUN cargo build --release -p sdkwork-im-standalone-gateway --bin sdkwork-im-standalone-gateway

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates curl \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /opt/sdkwork/im
COPY --from=builder /src/target/release/sdkwork-im-standalone-gateway /usr/local/bin/sdkwork-im-standalone-gateway
COPY deployments/templates/server.env.example /etc/sdkwork/im/server.env.example
ENV SDKWORK_IM_SERVICE_NAME=sdkwork-im-standalone-gateway
EXPOSE 18079
HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
  CMD curl -fsS http://127.0.0.1:18079/healthz || exit 1
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/sdkwork-im-standalone-gateway"]
