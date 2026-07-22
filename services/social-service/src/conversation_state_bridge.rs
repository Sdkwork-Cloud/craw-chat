use im_platform_contracts::CommitEnvelope;

/// Refresh the disposable co-located Conversation cache after a social commit.
///
/// This bridge is an optimization only. Social queries read normalized Social tables,
/// and startup never replays the journal into the cache.
pub fn try_apply_social_commit_to_conversation_state(envelope: &CommitEnvelope) {
    conversation_runtime::conversation_state::refresh_conversation_cache(envelope);
}

pub fn try_apply_social_commits_to_conversation_state(envelopes: &[CommitEnvelope]) {
    for envelope in envelopes {
        try_apply_social_commit_to_conversation_state(envelope);
    }
}

