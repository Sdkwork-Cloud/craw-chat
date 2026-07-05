# SDKWork IM Feature Gap Analysis & Optimization Roadmap

**Status**: Active  
**Updated**: 2026-07-05  
**Owner**: SDKWork IM Team  
**Priority**: P0 (Critical) → P3 (Nice-to-have)

## Executive Summary

The server-side IM core (social graph, conversation write path, projection read models, session-gateway realtime, call signaling) is **production-baseline capable**. Remaining gaps versus WeChat/Telegram/Discord are primarily **product-level** (E2EE, weak-network FEC/ARQ, mega-groups) and **P2 performance** items.

### Production baseline shipped (2026-07-05)

| Area | Capability |
| --- | --- |
| Outbox relay | conversation / social / rtc: relay fail-closed；写入侧跳过无 deliverable recipients（`rtc_outbox` 共享解析） |
| Journal ordering | Unified `commit_seq` per conversation; materialize-before-append + compensate on social/space writes |
| WebSocket | Pre-auth budget + post-auth semaphore; frame/upgrade Redis rate limits; Ping rate-limited |
| Social | PG materializer + multi-commit single PG transaction + terminal idempotency replay; friendship `update_status` conflict detection |
| RTC | Ring timeout reaper; terminal evict + DB `retention_until` purge; PC 远端视频 Volcengine 绑定 |
| Projection | HS256 keyset cursors; production rejects numeric offset; Postgres tiered timeline + `SDKWORK_IM_PROJECTION_TIMELINE_MEMORY_CAP` |
| Presence / routes | Stale device expiry; ClientRoute cleanup on disconnect |

Authoritative audit snapshot: `docs/COMMUNICATION_FEATURE_AUDIT_REPORT.md`.

---

## Feature Comparison Matrix

| Feature | Current | WeChat | Telegram | Discord | Priority | Effort |
|---------|---------|--------|----------|---------|----------|--------|
| **Weak Network Optimization** | ⚠️ WS 重连 + checkpoint；FEC/ARQ **库未接入运行时** | ✅ Excellent | ✅ Excellent | ✅ Good | **P1** | 2 weeks |
| **End-to-End Encryption** | ❌ None | ❌ None | ✅ Yes | ✅ Yes | **P0** | 4 weeks |
| **Multi-Device Sync** | ✅ Per-device read cursors + max-seq inbox/receipts | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Done | - |
| **Large Room Support** | ⚠️ Tiered timeline (Postgres + hot cache) | ✅ 10K | ✅ 200K | ✅ 1K | **P1** | 3 weeks |
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

**Current State**: WebSocket 重连与 checkpoint catch-up 已上线。`im-domain-core::network_optimization` 提供 XOR 示范级 FEC/ARQ **单元测试库**，**尚未接入** `session-gateway` 帧层。

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

## Implementation Timeline

| Phase | Duration | Features | Milestone |
|-------|----------|----------|-----------|
| **Phase 1** | Weeks 1-2 | FEC + ARQ | 95% delivery on 30% loss |
| **Phase 2** | Weeks 3-4 | E2EE (Signal Protocol) | Forward secrecy achieved |
| **Phase 3** | Weeks 5-6 | Multi-device sync | 3+ devices per user |
| **Phase 4** | Weeks 7-9 | Large rooms (SFU/MCU) | 10K participant rooms |
| **Phase 5** | Week 10 | Read receipts | Per-user tracking |
| **Phase 6** | Weeks 11-12 | Latency optimization | <100ms E2E |

---

## Success Metrics

### Performance Targets

| Metric | Current | Target | Industry Benchmark |
|--------|---------|--------|-------------------|
| Message delivery (30% loss) | 70% | 95% | WeChat: 98% |
| E2E latency (p50) | 200ms | 100ms | WeChat: 100ms |
| E2E latency (p99) | 500ms | 200ms | Telegram: 150ms |
| Max room size | 10 | 10,000 | Telegram: 200,000 |
| Multi-device sync | 1 | 5 | Discord: 5+ |
| Encryption overhead | 0% | 15% | Signal: 12% |

### Reliability Targets

| Metric | Target |
|--------|--------|
| Message delivery rate | 99.9% |
| Uptime SLA | 99.95% |
| Data durability | 99.999999% (11 9s) |
| Failover time | <30s |

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| FEC complexity | Medium | Use existing Reed-Solomon library |
| E2EE key management | High | Integrate with KMS, secure enclave |
| Large room scalability | High | Incremental rollout, load testing |
| Latency optimization | Medium | A/B testing, gradual rollout |

---

## Next Steps

1. **Week 1**: Implement FEC encoder/decoder in `network_optimization.rs`
2. **Week 1**: Implement ARQ manager with NACK-based retransmission
3. **Week 2**: Add network quality estimator and adaptive FEC
4. **Week 2**: Benchmark against WeChat on simulated lossy network

---

## References

- [Signal Protocol Specification](https://signal.org/docs/)
- [WebRTC FEC/ARQ Best Practices](https://webrtc.org/getting-started/overview/)
- [Vector Clocks for Distributed Systems](https://en.wikipedia.org/wiki/Vector_clock)
- [SFU vs MCU Architecture](https://webrtcglossary.com/sfu-vs-mcu/)
