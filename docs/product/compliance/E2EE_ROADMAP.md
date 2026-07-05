# Sdkwork IM — End-to-End Encryption Roadmap

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-03
Specs: PRIVACY_SPEC.md, SECURITY_SPEC.md, IAM_SPEC.md

## 1. Purpose

This document defines the end-to-end encryption (E2EE) roadmap and architecture for the Sdkwork IM
platform. It scopes how message content is encrypted on the sending client, transmitted as
ciphertext through the IM server, and decrypted only on the receiving client's device.

It extends the encryption summary in [DATA_PROTECTION.md](DATA_PROTECTION.md) and
[COMPLIANCE_FRAMEWORK.md](COMPLIANCE_FRAMEWORK.md) section 3.2 with a phased plan that moves message
content from transport- and at-rest-only protection to full end-to-end confidentiality, where the
server never holds plaintext message bodies or the keys required to decrypt them.

E2EE is planned for Phase 2 (2026 Q4) of the commercialization roadmap and is a prerequisite for
regulated tenants that require strong confidentiality guarantees against server-side data access.

## 2. Current Status

Sdkwork IM currently provides confidentiality through layered transport- and storage-level controls.
Message bodies are decryptable by the server because the platform must serve them to clients over
realtime and historical fetch paths.

### 2.1 Encryption In Place Today

| Layer | Mechanism | Scope | Plaintext Visible to Server |
| --- | --- | --- | --- |
| In transit | TLS 1.2+ for HTTP and WebSocket; mTLS for internal RPC | All client-to-server and service-to-service traffic | Yes (terminates at server) |
| At rest | PostgreSQL transparent data encryption or disk-level encryption (LUKS / cloud KMS) | Durable journal, projections, conversation stores | Yes (database decrypts on read) |
| Field-level | Envelope encryption (KMS-managed) for JWT signing keys and credentials | Credentials and signing keys only | No (for those fields only) |
| Backups | Encrypted pg_dump, object-lock S3 with SSE | Database snapshots and audit evidence | Yes (restorable) |

### 2.2 Gap

- Message content (chat text, attachment metadata) is stored and indexed in plaintext-equivalent
  form inside PostgreSQL projections and the event-sourced journal.
- The server can read, search, moderate, and export message bodies without client cooperation.
- Database compromise, subpoena, or privileged operator access exposes historical message content.
- E2EE is **not** implemented in any Phase 1 release. This roadmap defines the transition.

## 3. Threat Model

E2EE shifts the trust boundary from the server operator to the communicating endpoints. The
following threats are in scope for E2EE; threats outside E2EE are handled by the existing controls
in [COMPLIANCE_FRAMEWORK.md](COMPLIANCE_FRAMEWORK.md).

### 3.1 Threats E2EE Mitigates

| Threat | Description | Mitigation |
| --- | --- | --- |
| Server-side data access | Operator or compromised service inspects message bodies | Plaintext never reaches server; only ciphertext is stored |
| Database compromise | Attacker exfiltrates PostgreSQL journal or projections | Exfiltrated blobs are AEAD ciphertext without keys |
| Subpoena / lawful disclosure | Authority compels server operator to surrender content | Operator can only produce ciphertext and metadata |
| Backup leakage | Encrypted backup media restored outside controls | Restored data remains ciphertext |
| Insider threat | Privileged engineer with DB read access | No key material on server; nothing to misuse |

### 3.2 Threats E2EE Does Not Mitigate

| Threat | Owner |
| --- | --- |
| Endpoint compromise (malware on client) | Client OS hardening, device attestation |
| Metadata exposure (who messaged whom, when) | Server still routes and logs metadata |
| Traffic analysis / timing correlation | Network-layer padding (out of scope) |
| Push notification content leakage | FCM receives delivery metadata only; see [CUSTOMER_OPERATIONS.md](CUSTOMER_OPERATIONS.md) section 4 |
| Lost device key access (no recovery) | Backup and recovery design in section 9 |

### 3.3 Trust Assumptions

- The IM server is honest-but-curious: it routes and stores messages correctly but attempts to read
  content.
- Client devices are trusted up to the key store boundary; private keys never leave the device
  except through explicit user-initiated backup.
- The IAM identity layer is trusted to bind device identity keys to authenticated users.

## 4. Architecture Design

The E2EE design is inspired by the Signal Protocol and adapted to the Sdkwork IM CQRS + Event
Sourcing runtime. Encryption and decryption happen exclusively on clients; the server becomes a
ciphertext store-and-forward relay.

### 4.1 Protocol Components

| Component | Purpose |
| --- | --- |
| X3DH key agreement | Establishes an initial shared secret using long-term identity keys and ephemeral prekeys |
| Double Ratchet | Derives a new message key per message, providing forward secrecy and post-compromise security |
| X25519 (Curve25519) | Ephemeral and prekey Diffie-Hellman exchange |
| Ed25519 | Long-term device identity key; signs X25519 prekeys |
| AEAD (AES-256-GCM or ChaCha20-Poly1305) | Authenticated encryption of message plaintext |
| HKDF-SHA256 | Key derivation from ratchet output to message keys |

### 4.2 Per-Conversation Session Keys

Each 1:1 conversation maintains a dedicated Double Ratchet session per device pair. A session is
initialized once via X3DH and then advances independently on each message, deriving:

- A **root key** updated with each DH ratchet step.
- A **chain key** advanced per message to produce a unique message key.
- A **ratchet header** (`dhPub`, previous chain length `pn`, message number `n`) shipped with each
  ciphertext so the receiver can advance the matching state.

### 4.3 Client-Side Encryption Flow

The server receives, journals, and projects only ciphertext. The existing CQRS pipeline is
preserved; the `MessageBody` field that previously held plaintext now carries the E2EE envelope.

```text
Sender (Alice)                            Server                              Recipient (Bob)
     |                                       |                                      |
     |  fetch Bob prekey bundle              |                                      |
     | ------------------------------------> | (prekey directory)                   |
     | <------------------------------------ |                                      |
     |  X3DH -> shared secret SK             |                                      |
     |  init Double Ratchet session          |                                      |
     |  encrypt(plaintext, messageKey)       |                                      |
     |  POST /im/v3/api/messages             |                                      |
     |  { conversationId, e2eeEnvelope }     |                                      |
     | ------------------------------------> | (journal ciphertext, project blob)   |
     |                                       |  WebSocket realtime push             |
     |                                       | -----------------------------------> |
     |                                       |                                      |  fetch Alice identity/prekey
     |                                       |                                      |  X3DH -> SK
     |                                       |                                      |  init / advance ratchet
     |                                       |                                      |  decrypt(ciphertext, messageKey)
```

### 4.4 Encrypted Message Envelope

The ciphertext payload replaces the plaintext body in the message contract. The envelope follows
the HTTP response envelope rules in `API_SPEC.md` — `data.item` carries the encrypted blob; the
server treats it as opaque bytes.

```json
{
  "messageId": "msg_01H...",
  "conversationId": "conv_01H...",
  "senderUserId": "usr_alice",
  "senderDeviceId": "dev_alice_pc",
  "algorithm": "aes-256-gcm",
  "ratchetHeader": {
    "dhPub": "base64-X25519-public",
    "pn": 0,
    "n": 1
  },
  "ciphertext": "base64-AEAD-ciphertext",
  "associatedData": "base64-conversationId||messageId"
}
```

### 4.5 Server Never Sees Plaintext

- The IM server stores `ciphertext` and `ratchetHeader` as opaque bytes in the event-sourced journal
  and projection tables.
- Server-side full-text search, content moderation, and AI summarization operate on ciphertext and
  are therefore disabled for E2EE conversations (see section 9).
- The server does not hold, derive, or proxy any X25519 private key material.

## 5. Key Management

### 5.1 Device Key Pairs

Each client device generates and owns a long-term asymmetric keypair set on first enrollment:

| Key | Type | Lifetime | Stored On Server |
| --- | --- | --- | --- |
| Identity key | Ed25519 | Device lifetime (rotated only on re-enrollment) | Public only |
| Signed prekey | X25519 | 30–60 days, rotatable | Public only |
| One-time prekeys | X25519 | Single use, replenished from a pool | Public only, consumed on use |

Private halves never leave the device secure enclave / key store. The Ed25519 identity key signs
every published X25519 prekey so recipients can detect substitution.

### 5.2 Key Distribution

Prekey publication and lookup reuse the existing IM infrastructure:

- **Publication**: clients upload signed prekey and one-time prekey bundles through the
  `/im/v3/api/devices/*` surface, backed by the conversation-service projection store.
- **Lookup**: a sender fetches the recipient's prekey bundle before the first message. The bundle is
  returned through the standard `SdkWorkApiResponse` envelope.
- **Identity binding**: the IAM layer vouches for the binding between a user, a device, and the
  Ed25519 identity key. Device enrollment is gated by IAM authentication.
- **Revocation**: lost or replaced devices are revoked through the device management API; their
  prekeys are marked consumed and excluded from future bundles.

### 5.3 Key Rotation

| Key | Rotation Trigger | Behavior |
| --- | --- | --- |
| One-time prekey | Consumed by a session | Removed from pool; client replenishes |
| Signed prekey | Age (30–60 days) or compromise | New key published and signed; old key retained for in-flight sessions |
| Identity key | Device re-enrollment or user-initiated reset | New identity invalidates prior sessions; verified through IAM |
| Message key | Every message | Derived and discarded; never reused |

### 5.4 Multi-Device Support

A user with multiple devices (e.g., PC and mobile) maintains independent sessions per device pair.
Sending to a recipient fans out one ciphertext per recipient device; the server multiplexes delivery
but cannot combine the per-device ciphertexts.

## 6. Implementation Phases

E2EE is delivered incrementally so each phase is independently shippable and auditable. Phases align
with the commercialization roadmap and are gated by the audit criteria in section 10.

### 6.1 Phase Schedule

| Phase | Target | Scope | Out of Scope |
| --- | --- | --- | --- |
| Phase 2.1 | 2026-10 | Core E2EE for 1:1 direct chats, text messages only | Group, media, search |
| Phase 2.2 | 2026-11 | Group chat E2EE via MLS (Messaging Layer Security) | Media, search |
| Phase 2.3 | 2026-12 | E2EE for media attachments (encrypted file upload) | Search |
| Phase 3.1 | 2027-Q1 | E2EE-compatible search via searchable encryption | — |

### 6.2 Phase 2.1 — Core E2EE for 1:1 Direct Chats

- X3DH key agreement and Double Ratchet session establishment between two devices.
- Text message encryption with AES-256-GCM; ciphertext journaled through the existing event store.
- Per-device session state persisted in the client key store.
- E2EE is opt-in per conversation at the tenant level; non-E2EE conversations keep current behavior.

### 6.3 Phase 2.2 — Group Chat E2EE (MLS)

- Adopt Messaging Layer Security (RFC 9420) for group conversations where membership changes
  frequently.
- MLS group keys are ratcheted on every membership change (add/remove), providing forward secrecy
  across the group.
- The server acts as the MLS delivery service (commits and proposals) without access to group keys.

### 6.4 Phase 2.3 — E2EE for Media Attachments

- Attachment bytes are encrypted on the client with a per-attachment symmetric key.
- The encrypted blob is uploaded to SDKWork Drive; the key is delivered inside the E2EE message
  envelope.
- The server and SDKWork Drive store only ciphertext; thumbnails are generated client-side.

### 6.5 Phase 3.1 — E2EE-Compatible Search

- Searchable symmetric encryption (e.g., blind index / SSE) so the server can match encrypted
  queries against encrypted indices without revealing content.
- Replaces the Phase 1 Postgres FTI search path for E2EE conversations.
- Search index freshness lags message delivery; exact-match and prefix search only in the first cut.

## 7. Client Integration

All clients share a single E2EE core so protocol logic is implemented once and bound to each
platform's key store. The core is consumed by every client surface listed in
[CUSTOMER_OPERATIONS.md](CUSTOMER_OPERATIONS.md) section 4.

### 7.1 Shared E2EE Core

A shared crypto core (Rust, exposed to each client via the appropriate FFI / WASM bridge) provides:

- X3DH bundle generation and session initialization.
- Double Ratchet state machine and message key derivation.
- AEAD encrypt/decrypt with associated data binding (`conversationId || messageId`).
- Prekey pool management and signed-prekey rotation.

### 7.2 Per-Client Binding

| Client | Platform | Crypto Bridge | Key Store |
| --- | --- | --- | --- |
| PC | React (desktop) | Native Rust E2EE core via Electron Node binding | OS keychain (Windows Credential Manager / macOS Keychain) |
| H5 | React (web) | WASM build of the E2EE core | WebCrypto non-extractable keys + IndexedDB session state |
| Flutter | Flutter (mobile) | Rust E2EE core via `flutter_rust_bridge` FFI | Android Keystore / iOS Secure Enclave |

### 7.3 Session Lifecycle

1. **Enrollment**: first login on a device generates the identity keypair and uploads signed +
   one-time prekeys through the device API.
2. **Session init**: on first message to a peer, fetch the peer prekey bundle and run X3DH.
3. **Steady state**: each message advances the ratchet; session state is persisted locally.
4. **Device change**: re-enrollment rotates the identity key; peers detect the change and re-init.
5. **Logout / wipe**: local session state and private keys are destroyed; server-held prekeys are
   revoked.

## 8. Server Responsibilities

With E2EE enabled, the IM server's role narrows to key distribution, ciphertext routing, and
metadata handling. The server explicitly does **not** acquire plaintext access.

### 8.1 In-Scope Server Duties

| Duty | Description |
| --- | --- |
| Prekey directory | Store and serve public prekey bundles; consume one-time prekeys atomically |
| Identity attestation | Bind device identity keys to IAM-authenticated users |
| Ciphertext routing | Deliver encrypted envelopes over WebSocket realtime and history APIs |
| Ciphertext storage | Journal and project ciphertext blobs through the existing CQRS pipeline |
| Metadata handling | Persist delivery state, read receipts, timestamps (still in plaintext, by design) |
| Multi-device fan-out | Replicate one ciphertext per recipient device |

### 8.2 Explicitly Out of Scope for the Server

- Decrypting, indexing, or moderating message content.
- Holding any X25519 / Ed25519 private key.
- Generating or rotating message keys.
- Recovering plaintext on behalf of a user without device cooperation.

### 8.3 API Impact

The existing `/im/v3/api/messages` and WebSocket realtime contracts are preserved; the `body` field
carries the E2EE envelope as opaque bytes. The HTTP response envelope (`data.item`) and
`application/problem+json` error mapping are unchanged. No vendor compatibility `open-api` route is
affected because E2EE is applied to SDKWork-owned business operations only.

## 9. Limitations and Trade-offs

E2EE introduces deliberate capability reductions and operational complexity. These are accepted
trade-offs documented for customer decision-making.

### 9.1 Capability Reductions

| Capability | Non-E2EE | E2EE | Mitigation |
| --- | --- | --- | --- |
| Server-side full-text search | Available (Postgres FTI) | Not available | Phase 3.1 searchable encryption |
| Server-side content moderation | Available | Not available | Client-side reporting + reported-message escrow |
| Server-side AI summarization | Available (Phase 4) | Not available for E2EE chats | On-device summarization (future) |
| Server-side message export | Available via admin API | Ciphertext only | Client-driven export with keys |
| Cross-device history sync | Server replayed | Requires per-device sessions or encrypted backup | Encrypted backup store |

### 9.2 Operational Complexity

- **Backup**: encrypted message backup requires a recoverable key wrapped by a user secret; lost
  user secrets mean unrecoverable history. Backup design is a separate deliverable tracked under
  Phase 2.3.
- **Key recovery**: no server-side key escrow by default; enterprise tenants may opt into a managed
  recovery key, which weakens the threat model in section 3 and must be disclosed.
- **Multi-device UX**: users see per-device sessions; revoking a device invalidates its sessions but
  does not recover already-delivered messages.
- **Support**: customer support cannot read E2EE message content; support workflows must rely on
  metadata and client-side reports.

### 9.3 Compliance Interaction

E2EE interacts with the retention and legal-hold controls in
[COMPLIANCE_FRAMEWORK.md](COMPLIANCE_FRAMEWORK.md) section 7:

- Retention purge still applies to ciphertext and metadata.
- Legal hold preserves ciphertext; it does **not** grant plaintext access.
- Data subject export returns ciphertext plus the keys the user already holds; the server cannot add
  plaintext.

## 10. Security Audit Criteria

E2EE must pass the following before any phase goes live. Evidence is stored under
`docs/engineering/reviews/` per [COMPLIANCE_FRAMEWORK.md](COMPLIANCE_FRAMEWORK.md) section 12.

### 10.1 Cryptographic Review

| Criterion | Evidence |
| --- | --- |
| Protocol design reviewed against Signal Protocol / MLS RFCs | Signed design review record |
| No custom cryptography; only audited primitives (X25519, Ed25519, AEAD, HKDF) | Implementation inventory |
| Forward secrecy verified across ratchet steps | Test vector suite |
| Post-compromise security verified after DH ratchet | Test vector suite |
| Key zeroization on session teardown | Code review + memory test |

### 10.2 Implementation Review

| Criterion | Evidence |
| --- | --- |
| Third-party penetration test of E2EE clients and key distribution | Pentest report (encrypted S3, 3-year retention) |
| Constant-time comparison for all authentication tags | Static analysis + code review |
| No plaintext or private keys logged in telemetry | Telemetry redaction verification |
| No plaintext or private keys in error responses | API contract test |

### 10.3 Operational Readiness

| Criterion | Evidence |
| --- | --- |
| Key rotation runbook published | `docs/runbooks/RUNBOOK-e2ee-key-rotation.md` |
| Device revocation workflow tested | DR drill record |
| Encrypted backup restore tested end-to-end | Restore test report |
| Tenant opt-in / opt-out toggle verified | Feature flag test |

### 10.4 Compliance Sign-off

- Privacy review confirms E2EE does not weaken data subject rights handling.
- Security review confirms the threat model in section 3 holds under the deployed configuration.
- Customer-facing disclosure documents the capability reductions in section 9 before enablement.

## 11. References

- [COMPLIANCE_FRAMEWORK.md](COMPLIANCE_FRAMEWORK.md) — Regulatory compliance framework and encryption summary (section 3.2).
- [DATA_PROTECTION.md](DATA_PROTECTION.md) — Data protection and privacy controls.
- [CUSTOMER_OPERATIONS.md](CUSTOMER_OPERATIONS.md) — Customer operations guide and support boundaries.
- [SLA_SLO.md](SLA_SLO.md) — Service level agreements and objectives.
- [docs/product/roadmap/README.md](../roadmap/README.md) — Commercialization roadmap and phase tracking.
- [docs/architecture/tech/TECH_ARCHITECTURE.md](../../architecture/tech/TECH_ARCHITECTURE.md) — Technical architecture (CQRS, event sourcing, runtime topology).
- `../sdkwork-specs/PRIVACY_SPEC.md` — Platform privacy standard.
- `../sdkwork-specs/SECURITY_SPEC.md` — Platform security standard.
- `../sdkwork-specs/IAM_SPEC.md` — Identity and access management standard.
