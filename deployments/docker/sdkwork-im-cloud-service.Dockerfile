# syntax=docker/dockerfile:1
# Cloud-split service Dockerfile for SDKWork IM.
# Builds a single service binary for cloud-deployment profile.
#
# Usage:
#   docker build -f deployments/docker/sdkwork-im-cloud-service.Dockerfile \
#     --build-arg SERVICE_BIN=session-gateway-bin \
#     --build-arg SERVICE_NAME=session-gateway \
#     --build-arg HEALTH_PORT=18080 \
#     -t ghcr.io/sdkwork/session-gateway:latest .
#
# Available SERVICE_BIN values (matching Cargo package names):
#   sdkwork-im-cloud-gateway, session-gateway-bin,
#   sdkwork-comms-conversation-service-bin, projection-service-bin,
#   media-service-bin, streaming-service-bin, notification-service-bin,
#   governance-service-bin, audit-service-bin, automation-service-bin,
#   comms-social-service-bin, space-service-bin, contact-service,
#   interaction-service, ops-service-bin

ARG RUST_IMAGE=rust:1.85-bookworm
ARG RUNTIME_IMAGE=debian:bookworm-slim

# ── Builder stage: compile the service binary ───────────────────────────────
FROM ${RUST_IMAGE} AS builder
WORKDIR /src

# Step 1: Copy manifest files first to maximize Docker layer cache hits.
# Dependency compilation is cached unless Cargo.toml or Cargo.lock changes.
COPY Cargo.toml Cargo.lock ./

# Step 2: Create dummy source directories so `cargo build` can resolve the
# workspace without compiling actual source code. This caches all external
# crate dependencies in a dedicated layer.
RUN mkdir -p crates services adapters apis && \
    find crates services adapters apis -type d -exec \
      sh -c 'echo "" > "$1/lib.rs"' _ {} \; 2>/dev/null || true

# Step 3: Build dependencies only (dummy sources will fail to produce final
# binaries, but all dep crates are cached). We use --package to avoid
# building the whole workspace.
ARG SERVICE_BIN
RUN test -n "${SERVICE_BIN}" || (echo "SERVICE_BIN build arg is required" && false)

# Create a temporary Cargo.toml that only includes external dependencies
# by building a dummy crate that depends on the same external crates.
# If the workspace has a lib target, build it; otherwise just fetch deps.
RUN cargo fetch --locked 2>/dev/null || true

# Step 4: Copy actual source files (this layer changes on every code edit,
# but the dependency layer above is cached).
COPY crates ./crates
COPY services ./services
COPY adapters ./adapters
COPY apis ./apis

# Step 5: Build the actual service binary. Cargo will reuse cached deps.
RUN cargo build --release -p ${SERVICE_BIN}

# ── Runtime stage: minimal image with CA certs and healthcheck ──────────────
FROM ${RUNTIME_IMAGE} AS runtime
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates curl \
  && rm -rf /var/lib/apt/lists/*

ARG SERVICE_BIN
ARG SERVICE_NAME
ARG HEALTH_PORT=18080

WORKDIR /opt/sdkwork/im
COPY --from=builder /src/target/release/${SERVICE_BIN} /usr/local/bin/service
COPY deployments/templates/server.env.example /etc/sdkwork/im/server.env.example

ENV SDKWORK_IM_ENVIRONMENT=production
ENV SDKWORK_IM_DEPLOYMENT_PROFILE=cloud
ENV SDKWORK_IM_SERVICE_NAME=${SERVICE_NAME}
ENV TMPDIR=/tmp
EXPOSE ${HEALTH_PORT}

HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
  CMD curl -fsS http://127.0.0.1:${HEALTH_PORT}/healthz || exit 1

USER 65532:65532
ENTRYPOINT ["/usr/local/bin/service"]
