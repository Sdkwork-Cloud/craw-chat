# sdkwork-im-ccp-binding-tcp

CCP `ccp/tcp/1` transport binding for stream-oriented link sessions.

Wire format:

- `u32` big-endian payload length
- payload bytes produced by the negotiated CCP codec (`application/ccp+json` by default)

Framing helpers live in this crate; connection acceptors consume them through `sdkwork-im-runtime-link`.
