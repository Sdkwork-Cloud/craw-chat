use im_domain_core::stream::{StreamFrame, StreamSession};
use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamStateRecord {
    pub tenant_id: String,
    pub stream_id: String,
    pub session: StreamSession,
    pub frames: Vec<StreamFrame>,
    pub updated_at: String,
}

pub trait StreamStateStore: Send + Sync {
    fn load_state(
        &self,
        tenant_id: &str,
        stream_id: &str,
    ) -> Result<Option<StreamStateRecord>, ContractError>;

    fn save_state(&self, record: StreamStateRecord) -> Result<(), ContractError>;

    fn clear_state(&self, tenant_id: &str, stream_id: &str) -> Result<bool, ContractError>;
}

impl StreamStateRecord {
    pub fn merge_monotonic(self, next: Self) -> Self {
        let previous_session = self.session;
        let next_session = next.session;
        let previous_updated_at = self.updated_at;
        let next_updated_at = next.updated_at;
        let mut frames_by_seq = BTreeMap::new();

        for frame in self.frames {
            frames_by_seq.entry(frame.frame_seq).or_insert(frame);
        }
        for frame in next.frames {
            frames_by_seq.entry(frame.frame_seq).or_insert(frame);
        }

        let max_frame_seq = frames_by_seq.keys().next_back().copied().unwrap_or(0);
        let mut session = merge_stream_session(
            &previous_session,
            previous_updated_at.as_str(),
            &next_session,
            next_updated_at.as_str(),
        );
        session.last_frame_seq = session.last_frame_seq.max(max_frame_seq);

        Self {
            tenant_id: next.tenant_id,
            stream_id: next.stream_id,
            session,
            frames: frames_by_seq.into_values().collect(),
            updated_at: if previous_updated_at >= next_updated_at {
                previous_updated_at
            } else {
                next_updated_at
            },
        }
    }
}

fn merge_stream_session(
    previous: &StreamSession,
    previous_updated_at: &str,
    next: &StreamSession,
    next_updated_at: &str,
) -> StreamSession {
    let mut session = if stream_session_merge_score(next, next_updated_at)
        >= stream_session_merge_score(previous, previous_updated_at)
    {
        next.clone()
    } else {
        previous.clone()
    };

    session.last_frame_seq = previous.last_frame_seq.max(next.last_frame_seq);
    session.last_checkpoint_seq =
        max_optional_seq(previous.last_checkpoint_seq, next.last_checkpoint_seq);

    session
}

fn stream_session_merge_score<'a>(
    session: &'a StreamSession,
    updated_at: &'a str,
) -> (u8, u64, u64, &'a str, u8) {
    (
        stream_session_state_group_rank(&session.state),
        stream_session_high_water_mark(session),
        session.last_checkpoint_seq.unwrap_or(0),
        updated_at,
        stream_session_state_tie_rank(&session.state),
    )
}

fn stream_session_state_group_rank(state: &im_domain_core::stream::StreamSessionState) -> u8 {
    match state {
        im_domain_core::stream::StreamSessionState::Created => 0,
        im_domain_core::stream::StreamSessionState::Opened => 1,
        im_domain_core::stream::StreamSessionState::Active
        | im_domain_core::stream::StreamSessionState::Checkpointed => 2,
        im_domain_core::stream::StreamSessionState::Expired => 3,
        im_domain_core::stream::StreamSessionState::Aborted
        | im_domain_core::stream::StreamSessionState::Completed => 4,
    }
}

fn stream_session_state_tie_rank(state: &im_domain_core::stream::StreamSessionState) -> u8 {
    match state {
        im_domain_core::stream::StreamSessionState::Created => 0,
        im_domain_core::stream::StreamSessionState::Opened => 1,
        im_domain_core::stream::StreamSessionState::Active => 2,
        im_domain_core::stream::StreamSessionState::Checkpointed => 3,
        im_domain_core::stream::StreamSessionState::Expired => 4,
        im_domain_core::stream::StreamSessionState::Aborted => 5,
        im_domain_core::stream::StreamSessionState::Completed => 6,
    }
}

fn stream_session_high_water_mark(session: &StreamSession) -> u64 {
    [
        session.last_frame_seq,
        session.last_checkpoint_seq.unwrap_or(0),
        session.complete_frame_seq.unwrap_or(0),
        session.abort_frame_seq.unwrap_or(0),
    ]
    .into_iter()
    .max()
    .unwrap_or(0)
}

fn max_optional_seq(left: Option<u64>, right: Option<u64>) -> Option<u64> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im_domain_core::message::Sender;
    use im_domain_core::stream::{StreamDurabilityClass, StreamSessionState};
    use std::collections::BTreeMap;

    fn stream_state_record(
        state: StreamSessionState,
        last_frame_seq: u64,
        last_checkpoint_seq: Option<u64>,
        complete_frame_seq: Option<u64>,
        frame_seqs: Vec<u64>,
        updated_at: &str,
    ) -> StreamStateRecord {
        StreamStateRecord {
            tenant_id: "100001".into(),
            stream_id: "st_demo".into(),
            session: StreamSession {
                tenant_id: "100001".into(),
                stream_id: "st_demo".into(),
                owner_principal_id: "1".into(),
                owner_principal_kind: "user".into(),
                stream_type: "custom.delta.text".into(),
                scope_kind: "request".into(),
                scope_id: "req_demo".into(),
                durability_class: StreamDurabilityClass::DurableSession,
                ordering_scope: "stream".into(),
                schema_ref: Some("custom.delta.text.v1".into()),
                state,
                last_frame_seq,
                last_checkpoint_seq,
                result_message_id: complete_frame_seq.map(|_| "msg_done".into()),
                complete_frame_seq,
                abort_frame_seq: None,
                abort_reason: None,
                opened_at: "2026-05-06T00:00:00.000Z".into(),
                closed_at: complete_frame_seq.map(|_| "2026-05-06T00:00:03.000Z".into()),
                expires_at: None,
            },
            frames: frame_seqs.into_iter().map(stream_frame).collect(),
            updated_at: updated_at.into(),
        }
    }

    fn stream_frame(frame_seq: u64) -> StreamFrame {
        StreamFrame {
            tenant_id: "100001".into(),
            stream_id: "st_demo".into(),
            stream_type: "custom.delta.text".into(),
            scope_kind: "request".into(),
            scope_id: "req_demo".into(),
            frame_seq,
            frame_type: "delta".into(),
            schema_ref: Some("custom.delta.text.v1".into()),
            encoding: "json".into(),
            payload: format!("{{\"seq\":{frame_seq}}}"),
            sender: Sender {
                id: "1".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: BTreeMap::new(),
            },
            attributes: BTreeMap::new(),
            occurred_at: format!("2026-05-06T00:00:0{frame_seq}.000Z"),
        }
    }

    #[test]
    fn test_stream_state_record_merge_rejects_stale_cursor_and_frame_regression() {
        let current = stream_state_record(
            StreamSessionState::Completed,
            3,
            Some(2),
            Some(3),
            vec![1, 2, 3],
            "2026-05-06T00:00:03.000Z",
        );
        let stale = stream_state_record(
            StreamSessionState::Active,
            1,
            None,
            None,
            vec![1],
            "2026-05-06T00:00:01.000Z",
        );

        let merged = current.merge_monotonic(stale);

        assert_eq!(merged.session.state, StreamSessionState::Completed);
        assert_eq!(merged.session.last_frame_seq, 3);
        assert_eq!(merged.session.last_checkpoint_seq, Some(2));
        assert_eq!(merged.session.complete_frame_seq, Some(3));
        assert_eq!(
            merged.session.result_message_id.as_deref(),
            Some("msg_done")
        );
        assert_eq!(
            merged
                .frames
                .iter()
                .map(|frame| frame.frame_seq)
                .collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert_eq!(merged.updated_at, "2026-05-06T00:00:03.000Z");
    }
}
