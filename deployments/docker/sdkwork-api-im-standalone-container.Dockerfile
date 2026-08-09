# syntax=docker/dockerfile:1
# sdkwork-im standalone container image (deploymentProfile = "standalone").
#
# This is a runtime-only image: the gateway Linux binary and the packaged
# runtime assets (web renderer dists, database modules, app manifest) are
# assembled into the build context by
# scripts/build-im-standalone-container.mjs before `docker build`.
#
# Layout of the build context (dist/container-image-build):
#   bin/sdkwork-api-im-standalone-gateway   compiled Linux release binary
#   sdkwork.app.config.json                 application identity manifest
#   database/                               IM database module (manifest + migrations)
#   web/sdkwork-im-pc/dist/                 PC renderer static build
#   web/sdkwork-im-h5/dist/                 H5 renderer static build
#   modules/<workspace>/database/           embedded dependency database modules
#
# The gateway bootstraps the PostgreSQL schema/migrations itself on first
# start (SDKWORK_DATABASE_AUTO_MIGRATE=true), so the image needs no
# init-storage-server stage.

FROM debian:bookworm-slim

ARG GATEWAY_BINARY=sdkwork-api-im-standalone-gateway
ARG INSTALL_ROOT=/opt/sdkwork/im
ARG VERSION=0.0.0

# Runtime dependencies of the gateway binary: libssl3/ca-certificates for
# PostgreSQL TLS and outbound HTTPS, curl for the container healthcheck and
# operational diagnostics. The slim base image does not carry them.
RUN apt-get update \
  && apt-get install -y --no-install-recommends libssl3 ca-certificates curl \
  && rm -rf /var/lib/apt/lists/* \
  && groupadd --system sdkwork \
  && useradd --system --gid sdkwork --home-dir ${INSTALL_ROOT} sdkwork \
  && mkdir -p ${INSTALL_ROOT}/bin ${INSTALL_ROOT}/web \
    ${INSTALL_ROOT}/modules ${INSTALL_ROOT}/database \
    /var/lib/sdkwork/im /var/log/sdkwork/im /run/sdkwork/im \
  && chown -R sdkwork:sdkwork ${INSTALL_ROOT} \
    /var/lib/sdkwork/im /var/log/sdkwork/im /run/sdkwork/im

WORKDIR ${INSTALL_ROOT}
COPY . ${INSTALL_ROOT}
RUN chmod 0755 ${INSTALL_ROOT}/bin/${GATEWAY_BINARY}

# Deployment identity and runtime target (DEPLOYMENT_SPEC standalone posture).
ENV SDKWORK_IM_SERVICE_NAME=sdkwork-api-im-standalone-gateway
ENV SDKWORK_IM_DEPLOYMENT_PROFILE=standalone
ENV SDKWORK_IM_RUNTIME_TARGET=container
# Application roots: the packaged app manifest, the IM database module and the
# embedded dependency database modules all resolve from the install root.
ENV SDKWORK_APP_ROOT=${INSTALL_ROOT} \
    SDKWORK_IM_APP_ROOT=${INSTALL_ROOT} \
    SDKWORK_ACCOUNT_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-account \
    SDKWORK_DRIVE_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-drive \
    SDKWORK_KNOWLEDGEBASE_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-knowledgebase \
    SDKWORK_INVENTORY_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-inventory \
    SDKWORK_INVOICE_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-invoice \
    SDKWORK_MEMBERSHIP_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-membership \
    SDKWORK_MERCHANDISE_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-merchandise \
    SDKWORK_ORDER_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-order \
    SDKWORK_PAYMENT_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-payment \
    SDKWORK_SHOP_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-shop \
    SDKWORK_NOTARY_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-notary \
    SDKWORK_AGENTS_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-agents \
    SDKWORK_IAM_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-iam \
    SDKWORK_PROMOTION_APP_ROOT=${INSTALL_ROOT}/modules/sdkwork-promotion
# Renderer static sites served by the product runtime router (adaptive web:
# desktop UA -> PC app, mobile UA -> H5 app on the same origin).
ENV SDKWORK_IM_ADMIN_SITE_DIR=${INSTALL_ROOT}/web/sdkwork-im-pc/dist \
    SDKWORK_IM_PORTAL_SITE_DIR=${INSTALL_ROOT}/web/sdkwork-im-pc/dist \
    SDKWORK_IM_H5_SITE_DIR=${INSTALL_ROOT}/web/sdkwork-im-h5/dist

LABEL org.opencontainers.image.title="sdkwork-im standalone gateway (container)"
LABEL org.opencontainers.image.version="${VERSION}"
LABEL org.opencontainers.image.vendor="sdkwork"

USER sdkwork
EXPOSE 18079
HEALTHCHECK --interval=15s --timeout=5s --start-period=60s --retries=12 \
  CMD curl -fsS http://127.0.0.1:18079/healthz >/dev/null 2>&1 || exit 1
ENTRYPOINT ["/opt/sdkwork/im/bin/sdkwork-api-im-standalone-gateway"]
