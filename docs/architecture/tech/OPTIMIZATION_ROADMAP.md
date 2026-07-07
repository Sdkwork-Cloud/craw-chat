# SDKWork IM Feature Gap Analysis & Optimization Roadmap

**Status**: Active  
**Updated**: 2026-07-06  
**Owner**: SDKWork IM Team  
**Priority**: P0 (Critical) → P3 (Nice-to-have)

## Executive Summary

IM 核心（社交图、会话写路径、投影读模型、session-gateway 实时、呼叫信令）已达 **Pre-Release 基线**。2026-07-06 对齐轮次已交付：

- **Portal 真实聚合**：`portal-service` + `im-portal-snapshots` 从 ops 健康面与 audit 记录构建 dashboard/governance/access 快照；`product-runtime` 不再返回静态 JSON stub
- **Console/Admin 运营面**：Security 无审计数据 fail-closed；Dashboard 空指标不伪造零值；Infra 指标标签与信号源对齐
- **Flutter Inbox**：游标分页（HTTP wire `page_size=20`，最多 10 页同步）
- **P1 技术债关闭**：多组织 Inbox 隔离、realtime mutex 锁序、JWT jti Redis 重放、Social 读路径与单 commit PG 事务物化

剩余差距主要为 **产品级**（E2EE、FEC Phase 2、Telegram 级超大群）与 **P2 性能** 项。

### Shipped baseline (2026-07-06)

| Area | Capability |
| --- | --- |
| Portal API | `portal-service` HTTP handlers + gateway assembly mount；`SdkWorkApiResponse` 信封 |
| Console Security | `healthScore: null` when no audit data |
| Console Dashboard | `hasPortalMetrics()` — empty trends when ops 无数据 |
| Admin Infra | `realtimeWindowHealth` / `Projection Persist Ops` 标签对齐 |
| Flutter Inbox | Cursor `fetchInboxPage` + bounded multi-page sync |
| RPC projection plane | Eight inbox/projection-owned app RPC ops via `rpc_projection_dispatch` |
| Desktop offline | SQLite cache + send queue + `retryFailedMessage` |
| Outbox / Journal / WS | relay fail-closed；`commit_seq`；pre-auth budget + rate limits |
| Social / Projection | PG materialize-before-append；HS256 keyset cursors |

Authoritative audit snapshot: `docs/COMMUNICATION_FEATURE_AUDIT_REPORT.md`.

---

## Feature Comparison Matrix

| Feature | Current | WeChat | Telegram | Discord | Priority | Effort |
|---------|---------|--------|----------|---------|----------|--------|
| **Weak Network Optimization** | ✅ WS 重连 + checkpoint + `events.nack` ARQ 重放（2026-07-06）；FEC 自适应 Phase 2 | ✅ Excellent | ✅ Excellent | ✅ Good | **P1** | FEC wiring |
| **End-to-End Encryption** | ❌ None | ❌ None | ✅ Yes | ✅ Yes | **P0** | 4 weeks |
| **Multi-Device Sync** | ✅ Per-device read cursors + max-seq inbox/receipts | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Done | - |
| **Large Room Support** | ✅ 10K cap + 成员目录索引分页（MVP 2026-07-06）；Telegram 200K Phase 2 | ✅ 10K | ✅ 200K | ✅ 1K | **P1** | 3 weeks |
| **Desktop Offline Cache** | ✅ SQLite 缓存 + 读回退 + 文本/媒体（Drive 已上传）待发队列 + claim/lease + 重连 flush（2026-07-07） | ✅ | ✅ | ✅ | ✅ Done | - |
| **H5 Offline Queue** | ✅ IndexedDB + claim/lease + legacy sessionStorage 迁移（2026-07-07） | ✅ | ✅ | ✅ | ✅ Done | - |
| **Flutter Offline Queue** | ✅ SharedPreferences v2 + claim/lease（2026-07-07） | ✅ | ✅ | ✅ | ✅ Done | full SQLite message cache Phase 2 |
| **Message Recall** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Done | - |
| **Read Receipts** | ✅ Yes (cursor-derived summary) | ✅ Yes | ❌ No | ✅ Yes | ✅ Done | - |
| **E2E Latency** | ~200ms | ~100ms | ~150ms | ~200ms | **P1** | 2 weeks |
| **Rich Media** | ⚠️ Basic | ✅ Full | ✅ Full | ✅ Full | **P2** | 2 weeks |
| **Message Reactions** | ✅ Yes | ❌ No | ✅ Yes | ✅ Yes | ✅ Done | - |
| **Threads/Replies** | ✅ Yes (thread conversations) | ❌ No | ✅ Yes | ✅ Yes | ⚠️ Partial | 1 week |
| **Per-user message hide** | ✅ Yes (visibility + durable snapshot) | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Done | - |

---

## P0: Critical Path (Must-Have for Commercial Launch)

> **2026-07-05 基线说明**：IM 核心通信、PC 聊天、RTC 信令 + Volcengine 媒体已可上线。本节 P0 项为**对标 Telegram/行业顶级**的增量能力，不是当前上线的阻塞项。

### 1. Weak Network Optimization (FEC + ARQ)

**Current State**: WebSocket 重连、checkpoint catch-up 与 **`events.nack` ARQ 重放已接入**（IM SDK → session-gateway → runtime-link）。`im-domain-core::network_optimization` 中的 FEC / `NetworkQualityEstimator` 仍为库级实现，**尚未接入** WS 帧层（Phase 2）。

**Implementation Plan**:

```rust
// crates/im-domain-core/src/network_optimization.rs

/// Forward Error Correction (FEC) encoder
pub struct FecEncoder {
    /// Reed-Solomon parameters
    data_shards: usize,
    parity_shards: usize,
}

/// Automatic Repeat reQuest (ARQ) manager
pub struct ArqManager {
    /// Unacknowledged message buffer
    pending: HashMap<SequenceNumber, PendingMessage>,
    /// Retry timeout (exponential backoff)
    retry_timeout: Duration,
    /// Max retries before giving up
    max_retries: u32,
}

/// Network quality estimator
pub struct NetworkQualityEstimator {
    /// RTT samples (EWMA)
    rtt: f64,
    /// Packet loss rate
    loss_rate: f64,
    /// Bandwidth estimate
    bandwidth_bps: u64,
}
```

**Key Features**:
- FEC: Add parity packets for critical messages (configurable redundancy)
- ARQ: NACK-based retransmission with exponential backoff
- Adaptive quality: Adjust FEC redundancy based on loss rate
- Priority queuing: Critical messages (call signaling) get higher priority

**Metrics**:
- Target: 95% message delivery in <500ms on 30% packet loss
- Benchmark: WeChat achieves 98% on 50% loss

---

### 2. End-to-End Encryption (E2EE)

**Current State**: TLS only (transport encryption), no E2EE.

**Implementation Plan**:

```rust
// crates/im-domain-core/src/e2ee.rs

/// Signal Protocol implementation (Double Ratchet)
pub struct E2eeSession {
    /// Root key for deriving chain keys
    root_key: RootKey,
    /// Sending chain
    sending_chain: Chain,
    /// Receiving chains (one per sender)
    receiving_chains: HashMap<DeviceId, Chain>,
    /// Prekey bundle
    prekeys: PrekeyBundle,
}

/// Encrypted message envelope
pub struct EncryptedEnvelope {
    /// Ephemeral public key
    ephemeral_key: PublicKey,
    /// Encrypted payload (ciphertext)
    ciphertext: Vec<u8>,
    /// Message number (for ratchet)
    message_number: u32,
    /// Previous chain length (for out-of-order)
    previous_chain_length: u32,
}
```

**Key Features**:
- Signal Protocol: Double Ratchet + X3DH key exchange
- Per-device keys: Each device has its own identity key
- Key rotation: Automatic ratchet after each message
- Forward secrecy: Compromise of current key doesn't expose past messages
- Future secrecy: Compromise doesn't expose future messages (after ratchet)

**Compliance**:
- Follow Signal Protocol specification (https://signal.org/docs/)
- Support X3DH for initial key exchange
- Support Double Ratchet for ongoing communication

---

### 3. Multi-Device Synchronization

**Status (2026-07-05):** Shipped per-device read cursors (`memberId#deviceId` storage keys), inbox unread aggregation via `max(readSeq)` across devices, and cursor-derived read receipts. Remaining optional work: vector-clock differential sync for offline edit conflicts (not required for launch).

---

## P1: High Priority (Competitive Advantage)

### 4. Large Room Support (SFU/MCU Architecture)

**Current State**: P2P mesh (doesn't scale beyond ~10 participants).

**Implementation Plan**:

```rust
// crates/im-domain-core/src/large_room.rs

/// Scalable broadcast room
pub struct BroadcastRoom {
    /// Room capacity tier
    tier: RoomTier,
    /// Message distribution strategy
    strategy: DistributionStrategy,
    /// Active participants
    participants: HashMap<UserId, ParticipantState>,
}

/// Room capacity tier
pub enum RoomTier {
    /// Small: 1-50 participants (P2P mesh)
    Small,
    /// Medium: 51-500 participants (SFU)
    Medium,
    /// Large: 501-10000 participants (MCU)
    Large,
    /// Huge: 10000+ participants (Cascade)
    Huge,
}

/// Message distribution strategy
pub enum DistributionStrategy {
    /// Direct mesh (small rooms)
    Mesh,
    /// Selective Forwarding Unit (medium rooms)
    Sfu { server: String },
    /// Multi-point Control Unit (large rooms)
    Mcu { servers: Vec<String> },
    /// Cascade distribution (huge rooms)
    Cascade { tree: DistributionTree },
}
```

**Key Features**:
- Adaptive tier selection based on participant count
- SFU: Forward media without mixing (lower CPU, higher scalability)
- MCU: Mix media for large rooms (higher CPU, lower bandwidth for clients)
- Cascade: Tree-based distribution for 10K+ rooms

---

### 5. Read Receipts Enhancement

**Status (2026-07-05):** Shipped cursor-derived read receipts on `GET .../interaction_summary` via `readReceipt` (`activeMemberCount`, `readCount`, `readers`). Sender is excluded from counts by default. **Per-device read cursors** are supported via optional `deviceId` on `ConversationReadCursor` / `ReadCursorView`; storage keys are `memberId#deviceId` with legacy member-only fallback. Inbox unread and read receipts aggregate `max(readSeq)` across a member's devices.

**Remaining (optional):**
- Conversation-level `ReceiptPolicy` (always / one-on-one / never)
- **Delivery receipts** (separate from read receipts): requires client device ack of realtime `message.posted` seq and projection of checkpoint state — not cursor-derived

---

### 6. Latency Optimization

**Current State**: ~200ms E2E latency.

**Optimization Targets**:

| Component | Current | Target | Optimization |
|-----------|---------|--------|--------------|
| Client → Gateway | 50ms | 30ms | Edge deployment, WebSocket connection pooling |
| Gateway → Service | 40ms | 20ms | Service mesh (gRPC), connection reuse |
| Service → DB | 80ms | 40ms | Read replicas, query optimization, caching |
| DB → Service | 30ms | 10ms | Result streaming, async I/O |
| **Total** | **200ms** | **100ms** | **50% reduction** |

---

## P2: Medium Priority (User Experience)

### 7. Rich Media Support

**Current State**: Basic text messages.

**Add Support For**:
- Image: WebP/AVIF compression, thumbnails, EXIF stripping
- Video: Adaptive bitrate streaming, transcoding
- Audio: Opus codec, voice messages
- File: Chunked upload, resumable transfer, virus scanning
- Location: Live location sharing

---

### 8. Message Reactions

```rust
pub struct MessageReaction {
    message_id: String,
    user_id: UserId,
    emoji: String, // Unicode emoji or custom emoji ID
    reacted_at: String,
}
```

---

### 9. Threads/Replies

```rust
pub struct MessageThread {
    thread_id: String,
    root_message_id: String,
    reply_count: u32,
    last_reply_at: String,
    participants: HashSet<UserId>,
}
```

---

## Implementation Timeline (forward-looking)

| Phase | Focus | Status |
|-------|-------|--------|
| **Pre-Release baseline** | ARQ nack、10K 成员目录、桌面离线、Portal 真实聚合、Console fail-closed | ✅ Shipped 2026-07-06 |
| **Phase 2 — Weak network** | FEC + `NetworkQualityEstimator` 接入 WS 帧层 | 📋 Planned |
| **Phase 2 — Security** | E2EE Signal Protocol | 📋 Planned |
| **Phase 2 — Scale** | Telegram 200K cascade / notification·call·stream RPC watch surfaces | 📋 Planned |
| **Phase 2 — Performance** | E2E latency <100ms p50 | 📋 Planned |

---

## Success Metrics

### Shipped baseline (2026-07-06)

| Metric | Current |
|--------|---------|
| Message delivery ARQ (`events.nack`) | ✅ Client + gateway + runtime-link |
| Max chat group members | 10,000 (index-backed directory) |
| Multi-device read cursors | ✅ Per-device keys + max-seq aggregation |
| Read receipts | ✅ Cursor-derived interaction summary |
| Desktop offline | ✅ SQLite cache + send queue |
| Portal aggregation | ✅ ops + audit (no static stub) |
| RPC realtime streams | ✅ `presence.watch` + `realtime.events.watch` in session-gateway |

### Phase 2 targets

| Metric | Current | Target | Industry Benchmark |
|--------|---------|--------|-------------------|
| Message delivery (30% loss, with FEC) | ARQ only | 95% | WeChat: 98% |
| E2E latency (p50) | ~200ms | 100ms | WeChat: 100ms |
| Max room size (broadcast) | 10K members | 200K cascade | Telegram: 200,000 |
| E2EE overhead | TLS only | ~15% | Signal: 12% |

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| FEC complexity | Medium | Use existing Reed-Solomon library |
| E2EE key management | High | Integrate with KMS, secure enclave |
| Large room scalability | High | Incremental rollout, load testing |
| Latency optimization | Medium | A/B testing, gradual rollout |

---

## References

- [Signal Protocol Specification](https://signal.org/docs/)
- [WebRTC FEC/ARQ Best Practices](https://webrtc.org/getting-started/overview/)
- [Vector Clocks for Distributed Systems](https://en.wikipedia.org/wiki/Vector_clock)
- [SFU vs MCU Architecture](https://webrtcglossary.com/sfu-vs-mcu/)
