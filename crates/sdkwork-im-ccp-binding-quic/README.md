# sdkwork-im-ccp-binding-quic

CCP `ccp/quic/1` transport binding for encrypted stream-oriented link sessions.

Requirements:

- TLS certificate and private key configured through `SDKWORK_IM_REALTIME_QUIC_TLS_CERT_PATH` and `SDKWORK_IM_REALTIME_QUIC_TLS_KEY_PATH`
- Acceptors are wired through `sdkwork-im-runtime-link` and `session-gateway` link transport runtime

Use QUIC for non-loopback link transports. Raw TCP/UDP link transports require loopback bind addresses unless `SDKWORK_IM_REALTIME_LINK_ALLOW_INSECURE_BIND=true` is explicitly set for development.
