use im_platform_contracts::CommitEnvelope;

use crate::runtime::SocialRuntime;

/// Apply a committed social event to the embedded projection runtime.
///
/// Co-located standalone hosts call this immediately after journal append so contact
/// read models stay consistent without waiting for replay polling.
pub fn try_apply_social_commit_to_projection(envelope: &CommitEnvelope) {
    projection_service::try_apply_commit_envelope(envelope);
}

pub fn try_apply_social_commits_to_projection(envelopes: &[CommitEnvelope]) {
    for envelope in envelopes {
        try_apply_social_commit_to_projection(envelope);
    }
}

/// Replay persisted social commits into the embedded projection runtime.
///
/// Used during standalone embedded bootstrap to heal contact projections when the
/// social authority already contains friendships that were never projected.
pub fn replay_social_journal_to_projection(runtime: &SocialRuntime) {
    let replayed_commits = match runtime.replay_recorded_commit_pages(|commits| {
        try_apply_social_commits_to_projection(commits);
    }) {
        Ok(replayed_commits) => replayed_commits,
        Err(error) => {
            tracing::warn!(
                error = %error,
                "social projection replay skipped because commit journal readback is unavailable"
            );
            return;
        }
    };

    if replayed_commits == 0 {
        return;
    }

    tracing::info!(
        replayed_commits,
        "replayed social commit journal into embedded projection runtime"
    );
}

/// Replay persisted social commits into supplemental Postgres read tables.
///
/// Heals drift when journal authority is ahead of `im_friend_requests` / `im_friendships`
/// supplemental stores (for example after partial materialize failures in legacy paths).
pub fn replay_social_journal_to_postgres_read_model(
    runtime: &SocialRuntime,
    materializer: &crate::commit_materializer::SocialPostgresMaterializer,
) {
    let mut failures = 0_usize;
    let replayed_commits = match runtime.replay_recorded_commit_pages(|commits| {
        failures = failures.saturating_add(materializer.try_materialize_commits(commits));
    }) {
        Ok(replayed_commits) => replayed_commits,
        Err(error) => {
            tracing::warn!(
                error = %error,
                "social postgres read-model replay skipped because commit journal readback is unavailable"
            );
            return;
        }
    };
    if replayed_commits == 0 {
        return;
    }
    if failures > 0 {
        crate::social_materializer_metrics::record_postgres_materialization_failures(
            failures as u64,
        );
    }
    tracing::info!(
        replayed_commits,
        materialization_failures = failures,
        "replayed social commit journal into supplemental postgres read model"
    );
}
