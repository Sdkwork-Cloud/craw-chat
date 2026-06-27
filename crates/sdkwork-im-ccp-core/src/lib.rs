use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolVersion {
    pub family: String,
    pub major: u16,
    pub minor: u16,
}

impl ProtocolVersion {
    pub fn new(family: impl Into<String>, major: u16, minor: u16) -> Self {
        Self {
            family: family.into(),
            major,
            minor,
        }
    }

    pub fn wire_id(&self) -> String {
        format!("{}/{}.{}", self.family, self.major, self.minor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportBinding {
    Http1,
    Ws1,
    Sse1,
    Mqtt1,
    Tcp1,
    Udp1,
    Quic1,
}

impl TransportBinding {
    pub fn protocol_id(&self) -> &'static str {
        match self {
            Self::Http1 => "ccp/http/1",
            Self::Ws1 => "ccp/ws/1",
            Self::Sse1 => "ccp/sse/1",
            Self::Mqtt1 => "ccp/mqtt/1",
            Self::Tcp1 => "ccp/tcp/1",
            Self::Udp1 => "ccp/udp/1",
            Self::Quic1 => "ccp/quic/1",
        }
    }

    pub fn is_stream_oriented(&self) -> bool {
        matches!(
            self,
            Self::Ws1 | Self::Sse1 | Self::Mqtt1 | Self::Tcp1 | Self::Quic1
        )
    }

    pub fn is_datagram_oriented(&self) -> bool {
        matches!(self, Self::Udp1)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilitySet {
    pub items: BTreeSet<String>,
}

impl CapabilitySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn supports(&self, capability: &str) -> bool {
        self.items.contains(capability)
    }
}

impl<S> FromIterator<S> for CapabilitySet
where
    S: Into<String>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let items = iter.into_iter().map(Into::into).collect::<BTreeSet<_>>();
        Self { items }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CcpScope {
    pub scope_type: String,
    pub scope_id: String,
}

impl CcpScope {
    pub fn new(scope_type: impl Into<String>, scope_id: impl Into<String>) -> Self {
        Self {
            scope_type: scope_type.into(),
            scope_id: scope_id.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CcpRoute {
    pub tenant_id: String,
    pub principal_id: Option<String>,
    pub device_id: Option<String>,
}

impl CcpRoute {
    pub fn new(
        tenant_id: impl Into<String>,
        principal_id: Option<String>,
        device_id: Option<String>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            principal_id,
            device_id,
        }
    }

    pub fn for_principal(
        tenant_id: impl Into<String>,
        principal_id: impl Into<String>,
        device_id: Option<impl Into<String>>,
    ) -> Self {
        Self::new(
            tenant_id,
            Some(principal_id.into()),
            device_id.map(Into::into),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CcpSender {
    pub principal_id: String,
    pub device_id: Option<String>,
    pub session_id: Option<String>,
}

impl CcpSender {
    pub fn new(
        principal_id: impl Into<String>,
        device_id: Option<impl Into<String>>,
        session_id: Option<impl Into<String>>,
    ) -> Self {
        Self {
            principal_id: principal_id.into(),
            device_id: device_id.map(Into::into),
            session_id: session_id.map(Into::into),
        }
    }

    pub fn sender_id(&self) -> String {
        match self.device_id.as_deref() {
            Some(device_id) => format!("{}:{device_id}", self.principal_id),
            None => self.principal_id.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CcpActor {
    pub actor_id: String,
    pub actor_kind: String,
}

impl CcpActor {
    pub fn new(actor_id: impl Into<String>, actor_kind: impl Into<String>) -> Self {
        Self {
            actor_id: actor_id.into(),
            actor_kind: actor_kind.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CcpAuthority {
    pub tenant_id: String,
    pub sender: CcpSender,
    pub actor: CcpActor,
}

impl CcpAuthority {
    pub fn new(tenant_id: impl Into<String>, sender: CcpSender, actor: CcpActor) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            sender,
            actor,
        }
    }

    pub fn route(&self) -> CcpRoute {
        CcpRoute::new(
            self.tenant_id.clone(),
            Some(self.sender.principal_id.clone()),
            self.sender.device_id.clone(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CcpEnvelope {
    pub protocol: ProtocolVersion,
    pub binding: TransportBinding,
    pub kind: String,
    pub schema: String,
    pub scope: Option<CcpScope>,
    pub route: Option<CcpRoute>,
    pub flags: Vec<String>,
    pub trace_id: Option<String>,
    pub payload: String,
}

impl CcpEnvelope {
    // This constructor intentionally mirrors the full on-wire envelope shape so
    // protocol call sites keep every field explicit instead of hiding required
    // metadata behind partial builders.
    #[allow(clippy::too_many_arguments)]
    pub fn new<I, S>(
        protocol: ProtocolVersion,
        binding: TransportBinding,
        kind: impl Into<String>,
        schema: impl Into<String>,
        scope: Option<CcpScope>,
        route: Option<CcpRoute>,
        flags: I,
        trace_id: Option<String>,
        payload: impl Into<String>,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            protocol,
            binding,
            kind: kind.into(),
            schema: schema.into(),
            scope,
            route,
            flags: flags.into_iter().map(Into::into).collect(),
            trace_id,
            payload: payload.into(),
        }
    }
}
