use std::collections::BTreeMap;

use im_domain_core::stream::{StreamFrame, StreamSession};
use serde::{Deserialize, Serialize};

const STREAM_SESSION_DELIVERY_PROOF_VERSION: &str = "stream.session.delivery-proof.v1";
const STREAM_FRAME_DELIVERY_PROOF_VERSION: &str = "stream.frame.delivery-proof.v1";

#[derive(Clone, Debug)]
pub struct AppendStreamFrameOutcome {
    pub frame: StreamFrame,
    pub applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamFrameDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamFrameMutationResponse {
    #[serde(flatten)]
    pub frame: StreamFrame,
    pub request_key: String,
    pub delivery_status: StreamFrameDeliveryStatus,
    pub proof_version: String,
}

impl StreamFrameMutationResponse {
    pub fn from_outcome(outcome: AppendStreamFrameOutcome, request_key: String) -> Self {
        Self {
            frame: outcome.frame,
            request_key,
            delivery_status: if outcome.applied {
                StreamFrameDeliveryStatus::Applied
            } else {
                StreamFrameDeliveryStatus::Replayed
            },
            proof_version: STREAM_FRAME_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StreamSessionMutationOutcome {
    pub session: StreamSession,
    pub applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamSessionDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSessionMutationResponse {
    #[serde(flatten)]
    pub session: StreamSession,
    pub request_key: String,
    pub delivery_status: StreamSessionDeliveryStatus,
    pub proof_version: String,
}

impl StreamSessionMutationResponse {
    pub fn from_outcome(outcome: StreamSessionMutationOutcome, request_key: String) -> Self {
        Self {
            session: outcome.session,
            request_key,
            delivery_status: if outcome.applied {
                StreamSessionDeliveryStatus::Applied
            } else {
                StreamSessionDeliveryStatus::Replayed
            },
            proof_version: STREAM_SESSION_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenStreamRequest {
    pub stream_id: String,
    pub stream_type: String,
    pub scope_kind: String,
    pub scope_id: String,
    pub durability_class: String,
    pub schema_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckpointStreamRequest {
    pub frame_seq: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteStreamRequest {
    pub frame_seq: u64,
    pub result_message_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbortStreamRequest {
    pub frame_seq: Option<u64>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendStreamFrameRequest {
    pub frame_seq: u64,
    pub frame_type: String,
    pub schema_ref: Option<String>,
    pub encoding: String,
    pub payload: String,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStreamFramesQuery {
    pub after_frame_seq: Option<u64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamFrameWindow {
    pub items: Vec<StreamFrame>,
    pub next_after_frame_seq: Option<u64>,
    pub has_more: bool,
}
