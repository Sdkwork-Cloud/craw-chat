# sdkwork-im-ccp-binding-udp

CCP `ccp/udp/1` transport binding for datagram-oriented link sessions.

Each UDP datagram carries exactly one CCP codec payload. Datagram size is capped at
`CCP_UDP_MAX_DATAGRAM_BYTES` to stay within safe MTU budgets for edge devices.
