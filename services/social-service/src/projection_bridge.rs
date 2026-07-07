use im_adapters_local_disk::read_commit_journal_file;
use im_platform_contracts::CommitEnvelope;
use im_platform_contracts::ContractError;

use crate::runtime::SocialRuntime;

/// Apply a committed social event to the embedded projection runtime.
///
/// Unified-process hosts call this immediately after journal append so contact
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
/// Used during unified-process bootstrap to heal contact projections when the
/// social authority already contains friendships that were never projected.
pub fn replay_social_journal_to_projection(runtime: &SocialRuntime) {
    let commits = match runtime.recorded_commits() {
        Ok(commits) => commits,
        Err(error) => {
            tracing::warn!(
                error = %error,
                "social projection replay skipped because commit journal readback is unavailable"
            );
            return;
        }
    };

    if commits.is_empty() {
        return;
    }

    for commit in &commits {
        try_apply_social_commit_to_projection(commit);
    }

    tracing::info!(
        replayed_commits = commits.len(),
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
    let commits = match runtime.recorded_commits() {
        Ok(commits) => commits,
        Err(error) => {
            tracing::warn!(
                error = %error,
                "social postgres read-model replay skipped because commit journal readback is unavailable"
            );
            return;
        }
    };
    if commits.is_empty() {
        return;
    }
    let failures = materializer.try_materialize_commits(commits.as_slice());
    if failures > 0 {
        crate::social_materializer_metrics::record_postgres_materialization_failures(
            failures as u64,
        );
    }
    tracing::info!(
        replayed_commits = commits.len(),
        materialization_failures = failures,
        "replayed social commit journal into supplemental postgres read model"
    );
}

pub fn replay_social_journal_file_to_projection(journal_path: &std::path::Path) {
    let commits = match read_commit_journal_file(journal_path) {
        Ok(commits) => commits,
        Err(error) => {
            tracing::warn!(
                journal_path = %journal_path.display(),
                error = %contract_error_message(error),
                "social projection replay skipped because commit journal file could not be read"
            );
            return;
        }
    };

    if commits.is_empty() {
        return;
    }

    for commit in &commits {
        try_apply_social_commit_to_projection(commit);
    }

    tracing::info!(
        journal_path = %journal_path.display(),
        replayed_commits = commits.len(),
        "replayed social commit journal file into embedded projection runtime"
    );
}

fn contract_error_message(error: ContractError) -> String {
    match error {
        ContractError::UnsupportedCapability(message)
        | ContractError::Conflict(message)
        | ContractError::Unavailable(message)
        | ContractError::Invalid(message) => message,
    }
}
