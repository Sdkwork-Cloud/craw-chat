use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, MutexGuard};

use sdkwork_im_ccp_core::TransportBinding;

pub use sdkwork_im_ccp_binding_quic::{
    quic_framed_message_length, CCP_QUIC_FRAME_HEADER_BYTES, CCP_QUIC_MAX_FRAME_BYTES,
    QuicBinding, QuicBindingMessage,
};
pub use sdkwork_im_ccp_binding_tcp::{
    framed_message_length as tcp_framed_message_length, CCP_TCP_FRAME_HEADER_BYTES,
    CCP_TCP_MAX_FRAME_BYTES, TcpBinding, TcpBindingMessage,
};
pub use sdkwork_im_ccp_binding_udp::{
    validate_datagram_payload as validate_udp_datagram_payload, CCP_UDP_MAX_DATAGRAM_BYTES,
    UdpBinding, UdpBindingMessage,
};

#[cfg(test)]
pub use sdkwork_im_ccp_binding_tcp::{
    decode_length_prefixed_frame as decode_tcp_length_prefixed_frame,
    encode_length_prefixed_frame as encode_tcp_length_prefixed_frame,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LinkTransportKind {
    WebSocket,
    Tcp,
    Udp,
    Quic,
}

impl LinkTransportKind {
    pub fn binding(&self) -> TransportBinding {
        match self {
            Self::WebSocket => TransportBinding::Ws1,
            Self::Tcp => TransportBinding::Tcp1,
            Self::Udp => TransportBinding::Udp1,
            Self::Quic => TransportBinding::Quic1,
        }
    }

    pub fn protocol_id(&self) -> &'static str {
        self.binding().protocol_id()
    }

    pub fn is_stream_oriented(&self) -> bool {
        self.binding().is_stream_oriented()
    }

    pub fn is_datagram_oriented(&self) -> bool {
        self.binding().is_datagram_oriented()
    }
}

pub fn link_transport_kind_for_binding(binding: &TransportBinding) -> Option<LinkTransportKind> {
    match binding {
        TransportBinding::Ws1 => Some(LinkTransportKind::WebSocket),
        TransportBinding::Tcp1 => Some(LinkTransportKind::Tcp),
        TransportBinding::Udp1 => Some(LinkTransportKind::Udp),
        TransportBinding::Quic1 => Some(LinkTransportKind::Quic),
        TransportBinding::Http1 | TransportBinding::Sse1 | TransportBinding::Mqtt1 => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkConnectionKey {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
}

impl Hash for LinkConnectionKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tenant_id.hash(state);
        self.principal_id.hash(state);
        self.device_id.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkConnectionRecord {
    pub key: LinkConnectionKey,
    pub transport: LinkTransportKind,
    pub shard_id: usize,
    pub session_id: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct LinkConnectionRegistry {
    connections: Arc<Mutex<HashMap<LinkConnectionKey, LinkConnectionRecord>>>,
}

impl LinkConnectionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, record: LinkConnectionRecord) -> Result<(), &'static str> {
        let mut connections = lock_registry_mutex(&self.connections, "connection registry");
        if connections.contains_key(&record.key) {
            return Err("connection already registered for device scope");
        }
        connections.insert(record.key.clone(), record);
        Ok(())
    }

    pub fn replace(&self, record: LinkConnectionRecord) {
        lock_registry_mutex(&self.connections, "connection registry")
            .insert(record.key.clone(), record);
    }

    pub fn unregister(&self, key: &LinkConnectionKey) -> Option<LinkConnectionRecord> {
        lock_registry_mutex(&self.connections, "connection registry").remove(key)
    }

    pub fn lookup(&self, key: &LinkConnectionKey) -> Option<LinkConnectionRecord> {
        lock_registry_mutex(&self.connections, "connection registry")
            .get(key)
            .cloned()
    }

    pub fn len(&self) -> usize {
        lock_registry_mutex(&self.connections, "connection registry").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone, Debug)]
pub struct LinkShardDispatcher {
    shard_count: usize,
}

impl LinkShardDispatcher {
    pub fn new(shard_count: usize) -> Result<Self, &'static str> {
        if shard_count == 0 {
            return Err("shard count must be positive");
        }
        Ok(Self { shard_count })
    }

    pub fn default_realtime() -> Self {
        Self::new(256).expect("default realtime shard count should remain valid")
    }

    pub fn shard_count(&self) -> usize {
        self.shard_count
    }

    pub fn shard_for_device(&self, tenant_id: &str, principal_id: &str, device_id: &str) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        tenant_id.hash(&mut hasher);
        principal_id.hash(&mut hasher);
        device_id.hash(&mut hasher);
        (hasher.finish() as usize) % self.shard_count
    }

    pub fn shard_for_key(&self, key: &LinkConnectionKey) -> usize {
        self.shard_for_device(
            key.tenant_id.as_str(),
            key.principal_id.as_str(),
            key.device_id.as_str(),
        )
    }
}

fn lock_registry_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    label: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovering poisoned mutex in runtime-link/acceptor: {label}");
            poisoned.into_inner()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_shard_dispatcher_is_stable_for_device_scope() {
        let dispatcher = LinkShardDispatcher::default_realtime();
        let first = dispatcher.shard_for_device("100001", "1", "d_pad");
        let second = dispatcher.shard_for_device("100001", "1", "d_pad");
        assert_eq!(first, second);
        assert!(first < dispatcher.shard_count());
    }

    #[test]
    fn test_connection_registry_replaces_existing_device_session() {
        let registry = LinkConnectionRegistry::new();
        let key = LinkConnectionKey {
            tenant_id: "100001".into(),
            principal_id: "1".into(),
            device_id: "d_pad".into(),
        };
        let first = LinkConnectionRecord {
            key: key.clone(),
            transport: LinkTransportKind::Tcp,
            shard_id: 1,
            session_id: Some("s_old".into()),
        };
        registry
            .register(first)
            .expect("first connection should register");
        let replacement = LinkConnectionRecord {
            key: key.clone(),
            transport: LinkTransportKind::Tcp,
            shard_id: 1,
            session_id: Some("s_new".into()),
        };
        registry.replace(replacement);
        let stored = registry
            .lookup(&key)
            .expect("connection should remain registered");
        assert_eq!(stored.session_id.as_deref(), Some("s_new"));
    }

    #[test]
    fn test_tcp_framing_helpers_reexport_round_trip() {
        let payload = b"{\"kind\":\"hello\"}";
        let framed = encode_tcp_length_prefixed_frame(payload).expect("frame should encode");
        assert_eq!(
            tcp_framed_message_length(&framed).expect("length should parse"),
            framed.len()
        );
        assert_eq!(
            decode_tcp_length_prefixed_frame(&framed).expect("frame should decode"),
            payload.as_slice()
        );
    }

    #[test]
    fn test_udp_datagram_validation_enforces_mtu_budget() {
        validate_udp_datagram_payload(&[1]).expect("small datagram should pass");
        let oversized = vec![0_u8; CCP_UDP_MAX_DATAGRAM_BYTES + 1];
        let error = validate_udp_datagram_payload(&oversized).expect_err("oversized datagram");
        assert!(error.to_string().contains("maximum size"));
    }
}
