use std::cell::RefCell;
use std::io::{self, Write};
use std::sync::Arc;

use axum::body::{Body, Bytes};
use axum::http::{HeaderName, HeaderValue, StatusCode, header};
use axum::response::Response;
use futures_util::stream;
use im_app_context::AppContext;
use serde::Serialize;
use serde::ser::{Error as _, SerializeMap, SerializeSeq};
use tokio::sync::{OwnedSemaphorePermit, mpsc};

use crate::chain_scan::{AUDIT_CHAIN_SCAN_PAGE_SIZE, AuditChainAccumulator, AuditScanTarget};
use crate::{AuditRuntime, utc_now_rfc3339_millis};

const AUDIT_EXPORT_CHANNEL_CAPACITY: usize = 2;
const AUDIT_EXPORT_STREAM_CHUNK_BYTES: usize = 64 * 1024;
const AUDIT_EXPORT_STREAM_FAILURE_MESSAGE: &str = "audit export stream failed";

type AuditExportChunk = Result<Bytes, io::Error>;

pub(crate) fn streaming_export_response(
    ctx: &sdkwork_web_core::WebRequestContext,
    runtime: Arc<AuditRuntime>,
    auth: AppContext,
    target: AuditScanTarget,
    permit: OwnedSemaphorePermit,
) -> Response {
    let trace_id = ctx.resolved_trace_id();
    let exported_at = utc_now_rfc3339_millis();
    let (sender, receiver) = mpsc::channel(AUDIT_EXPORT_CHANNEL_CAPACITY);
    let error_sender = sender.clone();

    tokio::task::spawn_blocking(move || {
        let _permit = permit;
        let envelope = AuditExportEnvelope {
            runtime: runtime.as_ref(),
            auth: &auth,
            target: &target,
            exported_at: exported_at.as_str(),
            trace_id: trace_id.as_str(),
        };
        let mut writer = BoundedChunkWriter::new(sender);
        match serde_json::to_writer(&mut writer, &envelope) {
            Ok(()) => {
                let _ = writer.finish();
            }
            Err(error) => {
                if error.io_error_kind() != Some(io::ErrorKind::BrokenPipe) {
                    let _ = error_sender
                        .blocking_send(Err(io::Error::other(AUDIT_EXPORT_STREAM_FAILURE_MESSAGE)));
                }
            }
        }
    });

    let body_stream = stream::unfold(receiver, |mut receiver| async move {
        receiver.recv().await.map(|chunk| (chunk, receiver))
    });
    let mut response = Response::new(Body::from_stream(body_stream));
    *response.status_mut() = StatusCode::OK;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    if let Ok(value) = HeaderValue::from_str(&ctx.resolved_trace_id()) {
        response
            .headers_mut()
            .insert(HeaderName::from_static("x-sdkwork-trace-id"), value);
    }
    response
}

struct AuditExportEnvelope<'a> {
    runtime: &'a AuditRuntime,
    auth: &'a AppContext,
    target: &'a AuditScanTarget,
    exported_at: &'a str,
    trace_id: &'a str,
}

impl Serialize for AuditExportEnvelope<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("code", &0i32)?;
        map.serialize_entry(
            "data",
            &AuditExportData {
                runtime: self.runtime,
                auth: self.auth,
                target: self.target,
                exported_at: self.exported_at,
            },
        )?;
        map.serialize_entry("traceId", self.trace_id)?;
        map.end()
    }
}

struct AuditExportData<'a> {
    runtime: &'a AuditRuntime,
    auth: &'a AppContext,
    target: &'a AuditScanTarget,
    exported_at: &'a str,
}

impl Serialize for AuditExportData<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let accumulator = RefCell::new(AuditChainAccumulator::new(self.auth.tenant_id.as_str()));
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("tenantId", self.auth.tenant_id.as_str())?;
        map.serialize_entry("exportedAt", self.exported_at)?;
        map.serialize_entry(
            "items",
            &AuditRecordStream {
                runtime: self.runtime,
                auth: self.auth,
                target: self.target,
                accumulator: &accumulator,
            },
        )?;
        let result = accumulator.borrow().finish(self.target);
        map.serialize_entry("total", &result.total)?;
        map.serialize_entry("chainHeadHash", &result.chain_head_hash)?;
        map.serialize_entry("chainValid", &result.chain_valid)?;
        map.end()
    }
}

struct AuditRecordStream<'a, 'scan> {
    runtime: &'a AuditRuntime,
    auth: &'a AppContext,
    target: &'a AuditScanTarget,
    accumulator: &'scan RefCell<AuditChainAccumulator<'a>>,
}

impl Serialize for AuditRecordStream<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut records = serializer.serialize_seq(None)?;
        let mut after_audit_seq = 0u64;

        while after_audit_seq < self.target.max_audit_seq {
            let page = self
                .runtime
                .scan_records_page(
                    self.auth,
                    after_audit_seq,
                    self.target.max_audit_seq,
                    AUDIT_CHAIN_SCAN_PAGE_SIZE,
                )
                .map_err(|error| {
                    tracing::error!(
                        tenant_id = %self.auth.tenant_id,
                        organization_id = %self.auth.organization_id,
                        code = error.code,
                        "audit export page read failed"
                    );
                    S::Error::custom(AUDIT_EXPORT_STREAM_FAILURE_MESSAGE)
                })?;

            if page.items.is_empty() {
                if page.has_more {
                    return Err(S::Error::custom(
                        "audit export cursor returned an empty page with has_more=true",
                    ));
                }
                break;
            }

            for record in &page.items {
                if record.audit_seq <= after_audit_seq
                    || record.audit_seq > self.target.max_audit_seq
                {
                    return Err(S::Error::custom(
                        "audit export cursor did not advance within the fixed high watermark",
                    ));
                }
                self.accumulator
                    .borrow_mut()
                    .observe(record)
                    .map_err(|_| S::Error::custom(AUDIT_EXPORT_STREAM_FAILURE_MESSAGE))?;
                records.serialize_element(record)?;
            }

            let next_audit_seq = page
                .items
                .last()
                .map(|record| record.audit_seq)
                .ok_or_else(|| S::Error::custom("audit export page unexpectedly became empty"))?;
            if next_audit_seq <= after_audit_seq {
                return Err(S::Error::custom("audit export cursor failed to advance"));
            }
            after_audit_seq = next_audit_seq;
            if !page.has_more {
                break;
            }
        }

        records.end()
    }
}

struct BoundedChunkWriter {
    sender: mpsc::Sender<AuditExportChunk>,
    buffer: Vec<u8>,
}

impl BoundedChunkWriter {
    fn new(sender: mpsc::Sender<AuditExportChunk>) -> Self {
        Self {
            sender,
            buffer: Vec::with_capacity(AUDIT_EXPORT_STREAM_CHUNK_BYTES),
        }
    }

    fn send_buffer(&mut self) -> io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }
        let bytes = Bytes::from(std::mem::replace(
            &mut self.buffer,
            Vec::with_capacity(AUDIT_EXPORT_STREAM_CHUNK_BYTES),
        ));
        self.sender
            .blocking_send(Ok(bytes))
            .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "audit export receiver closed"))
    }

    fn finish(mut self) -> io::Result<()> {
        self.send_buffer()
    }
}

impl Write for BoundedChunkWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let mut remaining = bytes;
        while !remaining.is_empty() {
            let available = AUDIT_EXPORT_STREAM_CHUNK_BYTES.saturating_sub(self.buffer.len());
            if available == 0 {
                self.send_buffer()?;
                continue;
            }
            let count = available.min(remaining.len());
            self.buffer.extend_from_slice(&remaining[..count]);
            remaining = &remaining[count..];
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.send_buffer()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use http_body_util::BodyExt;
    use sdkwork_web_core::{
        ServerRequestId, WebApiSurface, WebAuthMode, WebRequestContext, WebTransportFacts,
    };

    use crate::RecordAuditAnchor;

    #[test]
    fn bounded_writer_never_buffers_more_than_one_chunk() {
        let (sender, mut receiver) = mpsc::channel(4);
        let mut writer = BoundedChunkWriter::new(sender);
        let input = vec![b'x'; AUDIT_EXPORT_STREAM_CHUNK_BYTES + 17];

        writer.write_all(&input).expect("write should succeed");
        assert_eq!(writer.buffer.len(), 17);
        let first = receiver
            .blocking_recv()
            .expect("first chunk")
            .expect("data");
        assert_eq!(first.len(), AUDIT_EXPORT_STREAM_CHUNK_BYTES);
    }

    #[tokio::test]
    async fn export_streams_multiple_pages_and_keeps_the_sync_bundle_bounded() {
        let runtime = Arc::new(AuditRuntime::default());
        let auth = AppContext {
            tenant_id: "tenant-stream".to_owned(),
            organization_id: "organization-stream".to_owned(),
            user_id: "user-1".to_owned(),
            actor_id: "user-1".to_owned(),
            actor_kind: "user".to_owned(),
            session_id: Some("session-1".to_owned()),
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: BTreeSet::from(["audit.read".to_owned()]),
            device_id: None,
        };
        for index in 1..=201 {
            runtime
                .record_anchor(
                    &auth,
                    RecordAuditAnchor {
                        record_id: format!("stream-record-{index}"),
                        aggregate_type: "security_event".to_owned(),
                        aggregate_id: format!("stream-event-{index}"),
                        action: "security.observed".to_owned(),
                        payload: Some("x".repeat(1_024)),
                    },
                )
                .expect("audit record should append");
        }

        let verification = runtime
            .verify_chain(&auth)
            .expect("multi-page verification should succeed");
        assert_eq!(verification.total, 201);
        assert!(verification.chain_valid);
        assert!(
            runtime.export_bundle(&auth).is_err(),
            "the compatibility bundle must fail closed above one bounded page"
        );

        let context = WebRequestContext {
            request_id: ServerRequestId("request-stream".to_owned()),
            api_surface: WebApiSurface::BackendApi,
            auth_mode: WebAuthMode::DualToken,
            principal: None,
            transport: WebTransportFacts {
                path: "/backend/v3/api/audit/export".to_owned(),
                method: "GET".to_owned(),
                auth_token_present: true,
                access_token_present: true,
                api_key_present: false,
                oauth_bearer_present: false,
                agent_token_present: false,
            },
            locale: None,
            client_kind: None,
            operation: None,
            trace_id: Some("trace-stream".to_owned()),
            idempotency_key: None,
        };
        let target = runtime.prepare_scan(&auth).expect("scan target");
        let permit = Arc::new(tokio::sync::Semaphore::new(1))
            .try_acquire_owned()
            .expect("export permit");
        let response = streaming_export_response(&context, runtime, auth, target, permit);
        let mut body = response.into_body();
        let mut chunks = 0usize;
        let mut payload = Vec::new();
        while let Some(frame) = body.frame().await {
            let frame = frame.expect("stream frame should succeed");
            if let Some(data) = frame.data_ref() {
                chunks += 1;
                payload.extend_from_slice(data);
            }
        }

        assert!(chunks > 1, "large audit export should emit multiple chunks");
        let json: serde_json::Value =
            serde_json::from_slice(&payload).expect("streamed export should be valid json");
        assert_eq!(json["code"], 0);
        assert_eq!(json["data"]["total"], 201);
        assert_eq!(
            json["data"]["items"]
                .as_array()
                .expect("items should be an array")
                .len(),
            201
        );
        assert_eq!(json["data"]["chainValid"], true);
        assert_eq!(json["traceId"], "trace-stream");
    }

    #[tokio::test]
    async fn dropping_response_body_cancels_export_and_releases_scan_permit() {
        let runtime = Arc::new(AuditRuntime::default());
        let auth = AppContext {
            tenant_id: "tenant-cancel".to_owned(),
            organization_id: "organization-cancel".to_owned(),
            user_id: "user-1".to_owned(),
            actor_id: "user-1".to_owned(),
            actor_kind: "user".to_owned(),
            session_id: Some("session-1".to_owned()),
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: BTreeSet::from(["audit.read".to_owned()]),
            device_id: None,
        };
        for index in 1..=4 {
            runtime
                .record_anchor(
                    &auth,
                    RecordAuditAnchor {
                        record_id: format!("cancel-record-{index}"),
                        aggregate_type: "security_event".to_owned(),
                        aggregate_id: format!("cancel-event-{index}"),
                        action: "security.observed".to_owned(),
                        payload: Some("x".repeat(128 * 1024)),
                    },
                )
                .expect("audit record should append");
        }
        let context = WebRequestContext {
            request_id: ServerRequestId("request-cancel".to_owned()),
            api_surface: WebApiSurface::BackendApi,
            auth_mode: WebAuthMode::DualToken,
            principal: None,
            transport: WebTransportFacts {
                path: "/backend/v3/api/audit/export".to_owned(),
                method: "GET".to_owned(),
                auth_token_present: true,
                access_token_present: true,
                api_key_present: false,
                oauth_bearer_present: false,
                agent_token_present: false,
            },
            locale: None,
            client_kind: None,
            operation: None,
            trace_id: Some("trace-cancel".to_owned()),
            idempotency_key: None,
        };
        let target = runtime.prepare_scan(&auth).expect("scan target");
        let scan_gate = Arc::new(tokio::sync::Semaphore::new(1));
        let permit = scan_gate
            .clone()
            .try_acquire_owned()
            .expect("export permit");
        let response = streaming_export_response(&context, runtime, auth, target, permit);

        drop(response);

        let reacquired =
            tokio::time::timeout(std::time::Duration::from_secs(2), scan_gate.acquire_owned())
                .await
                .expect("cancelled export should release its permit promptly")
                .expect("scan gate should remain open");
        drop(reacquired);
    }
}
