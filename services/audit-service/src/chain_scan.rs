use crate::{AuditError, AuditRecord, AuditRecordHashInput, compute_audit_record_chain_hash};

pub(crate) const AUDIT_CHAIN_SCAN_PAGE_SIZE: usize = 20;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct AuditScanTarget {
    pub(crate) max_audit_seq: u64,
    pub(crate) chain_head_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AuditScanPage {
    pub(crate) items: Vec<AuditRecord>,
    pub(crate) has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AuditChainScanResult {
    pub(crate) total: usize,
    pub(crate) chain_head_hash: Option<String>,
    pub(crate) chain_valid: bool,
}

pub(crate) struct AuditChainAccumulator<'a> {
    tenant_id: &'a str,
    total: usize,
    expected_audit_seq: Option<u64>,
    last_audit_seq: Option<u64>,
    previous_hash: Option<String>,
    chain_valid: bool,
}

impl<'a> AuditChainAccumulator<'a> {
    pub(crate) fn new(tenant_id: &'a str) -> Self {
        Self {
            tenant_id,
            total: 0,
            expected_audit_seq: Some(1),
            last_audit_seq: None,
            previous_hash: None,
            chain_valid: true,
        }
    }

    pub(crate) fn observe(&mut self, item: &AuditRecord) -> Result<(), AuditError> {
        self.total = self.total.checked_add(1).ok_or_else(|| {
            AuditError::internal("audit_count_overflow", "audit count overflowed")
        })?;

        if item.tenant_id != self.tenant_id
            || self.expected_audit_seq != Some(item.audit_seq)
            || item.chain_prev_hash != self.previous_hash
        {
            self.chain_valid = false;
        }

        let expected_hash = compute_audit_record_chain_hash(AuditRecordHashInput {
            tenant_id: item.tenant_id.as_str(),
            record_id: item.record_id.as_str(),
            audit_seq: item.audit_seq,
            aggregate_type: item.aggregate_type.as_str(),
            aggregate_id: item.aggregate_id.as_str(),
            action: item.action.as_str(),
            actor_id: item.actor_id.as_str(),
            actor_kind: item.actor_kind.as_str(),
            actor_session_id: item.actor_session_id.as_deref(),
            payload: item.payload.as_deref(),
            recorded_at: item.recorded_at.as_str(),
            chain_prev_hash: item.chain_prev_hash.as_deref(),
        });
        if item.chain_hash != expected_hash {
            self.chain_valid = false;
        }

        self.expected_audit_seq = item.audit_seq.checked_add(1);
        self.last_audit_seq = Some(item.audit_seq);
        self.previous_hash = Some(item.chain_hash.clone());
        Ok(())
    }

    pub(crate) fn finish(&self, target: &AuditScanTarget) -> AuditChainScanResult {
        let reached_target = match target.max_audit_seq {
            0 => {
                self.total == 0
                    && self.last_audit_seq.is_none()
                    && self.previous_hash.is_none()
                    && target.chain_head_hash.is_none()
            }
            max_audit_seq => {
                self.last_audit_seq == Some(max_audit_seq)
                    && self.previous_hash == target.chain_head_hash
            }
        };

        AuditChainScanResult {
            total: self.total,
            chain_head_hash: self.previous_hash.clone(),
            chain_valid: self.chain_valid && reached_target,
        }
    }
}

pub(crate) fn verify_audit_records_chain(tenant_id: &str, items: &[AuditRecord]) -> bool {
    let target = items
        .last()
        .map_or_else(AuditScanTarget::default, |item| AuditScanTarget {
            max_audit_seq: item.audit_seq,
            chain_head_hash: Some(item.chain_hash.clone()),
        });
    let mut accumulator = AuditChainAccumulator::new(tenant_id);
    for item in items {
        if accumulator.observe(item).is_err() {
            return false;
        }
    }
    accumulator.finish(&target).chain_valid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TenantAuditRecords;

    fn audit_record(audit_seq: u64, chain_prev_hash: Option<String>) -> AuditRecord {
        let mut record = AuditRecord {
            tenant_id: "tenant-a".to_owned(),
            record_id: format!("record-{audit_seq}"),
            audit_seq,
            aggregate_type: "security_event".to_owned(),
            aggregate_id: format!("event-{audit_seq}"),
            action: "security.observed".to_owned(),
            actor_id: "user-1".to_owned(),
            actor_kind: "user".to_owned(),
            actor_session_id: Some("session-1".to_owned()),
            payload: None,
            recorded_at: format!("2026-07-16T00:00:{audit_seq:02}.000Z"),
            chain_prev_hash,
            chain_hash: String::new(),
        };
        record.chain_hash = compute_audit_record_chain_hash(AuditRecordHashInput {
            tenant_id: record.tenant_id.as_str(),
            record_id: record.record_id.as_str(),
            audit_seq: record.audit_seq,
            aggregate_type: record.aggregate_type.as_str(),
            aggregate_id: record.aggregate_id.as_str(),
            action: record.action.as_str(),
            actor_id: record.actor_id.as_str(),
            actor_kind: record.actor_kind.as_str(),
            actor_session_id: record.actor_session_id.as_deref(),
            payload: record.payload.as_deref(),
            recorded_at: record.recorded_at.as_str(),
            chain_prev_hash: record.chain_prev_hash.as_deref(),
        });
        record
    }

    #[test]
    fn accumulator_preserves_chain_state_across_pages() {
        let first = audit_record(1, None);
        let second = audit_record(2, Some(first.chain_hash.clone()));
        let third = audit_record(3, Some(second.chain_hash.clone()));
        let target = AuditScanTarget {
            max_audit_seq: 3,
            chain_head_hash: Some(third.chain_hash.clone()),
        };
        let mut accumulator = AuditChainAccumulator::new("tenant-a");

        accumulator.observe(&first).expect("first page");
        accumulator.observe(&second).expect("first page");
        accumulator.observe(&third).expect("second page");

        assert_eq!(
            accumulator.finish(&target),
            AuditChainScanResult {
                total: 3,
                chain_head_hash: Some(third.chain_hash),
                chain_valid: true,
            }
        );
    }

    #[test]
    fn accumulator_rejects_missing_sequence_even_when_hash_links_match() {
        let first = audit_record(1, None);
        let third = audit_record(3, Some(first.chain_hash.clone()));
        let target = AuditScanTarget {
            max_audit_seq: 3,
            chain_head_hash: Some(third.chain_hash.clone()),
        };
        let mut accumulator = AuditChainAccumulator::new("tenant-a");

        accumulator.observe(&first).expect("first record");
        accumulator.observe(&third).expect("third record");

        assert!(!accumulator.finish(&target).chain_valid);
    }

    #[test]
    fn accumulator_rejects_payload_tampering() {
        let first = audit_record(1, None);
        let mut second = audit_record(2, Some(first.chain_hash.clone()));
        let target = AuditScanTarget {
            max_audit_seq: 2,
            chain_head_hash: Some(second.chain_hash.clone()),
        };
        second.payload = Some("tampered".to_owned());
        let mut accumulator = AuditChainAccumulator::new("tenant-a");

        accumulator.observe(&first).expect("first record");
        accumulator.observe(&second).expect("second record");

        assert!(!accumulator.finish(&target).chain_valid);
    }

    #[test]
    fn fixed_high_watermark_excludes_records_appended_after_scan_start() {
        let first = audit_record(1, None);
        let second = audit_record(2, Some(first.chain_hash.clone()));
        let target = AuditScanTarget {
            max_audit_seq: first.audit_seq,
            chain_head_hash: Some(first.chain_hash.clone()),
        };
        let mut records = TenantAuditRecords::default();
        records.push(first);
        records.push(second);

        let page = records.window_through(0, target.max_audit_seq, 20);

        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].audit_seq, 1);
        assert!(!page.has_more);
        let mut accumulator = AuditChainAccumulator::new("tenant-a");
        accumulator.observe(&page.items[0]).expect("first record");
        assert!(accumulator.finish(&target).chain_valid);
    }
}
