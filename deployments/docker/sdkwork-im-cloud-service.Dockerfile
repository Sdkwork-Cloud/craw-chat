# syntax=docker/dockerfile:1

# Runtime-only cloud service image packager.
#
# The Rust binary is built from the complete SDKWork workspace before this
# Dockerfile runs. RUNTIME_IMAGE must be a hardened Linux runtime image pinned
# by OCI digest and must provide CA certificates for outbound TLS. The image
# intentionally contains no source tree, package manager, compiler, or shell.
#
# Example:
#   cargo build --locked --release \
#     --package session-gateway-bin --bin session-gateway
#   docker build \
#     --file deployments/docker/sdkwork-im-cloud-service.Dockerfile \
#     --build-arg RUNTIME_IMAGE=registry.example/runtime@sha256:<real-digest> \
#     --build-arg SERVICE_ARTIFACT=target/release/session-gateway \
#     --build-arg SERVICE_NAME=session-gateway \
#     --build-arg HEALTH_PORT=28080 \
#     --tag ghcr.io/sdkwork/session-gateway:<release-id> .

ARG RUNTIME_IMAGE
FROM ${RUNTIME_IMAGE} AS runtime

ARG SERVICE_ARTIFACT
ARG SERVICE_NAME
ARG HEALTH_PORT

WORKDIR /opt/sdkwork/im
COPY --chown=65532:65532 ${SERVICE_ARTIFACT} /usr/local/bin/service

ENV SDKWORK_IM_ENVIRONMENT=production
ENV SDKWORK_IM_DEPLOYMENT_PROFILE=cloud
ENV SDKWORK_IM_RUNTIME_TARGET=container
ENV SDKWORK_IM_SERVICE_NAME=${SERVICE_NAME}
ENV OTEL_SERVICE_NAME=${SERVICE_NAME}
ENV TMPDIR=/tmp

EXPOSE ${HEALTH_PORT}
USER 65532:65532
ENTRYPOINT ["/usr/local/bin/service"]
