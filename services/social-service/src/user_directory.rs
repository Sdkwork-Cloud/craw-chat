//! IAM-backed target user validation and friend-request privacy gates.

use std::sync::Arc;

use im_adapters_social_postgres::{SocialPostgresPool, postgres_pool_client};
use serde_json::Value;

use crate::friendship::SocialServiceError;

const IAM_USER_EXISTS_SQL: &str = r#"
SELECT 1
FROM iam_user
WHERE tenant_id = $1
  AND id = $2
  AND is_deleted = 0
LIMIT 1
"#;

const TARGET_PRIVACY_SQL: &str = r#"
SELECT im_privacy_settings
FROM im_user_profiles
WHERE tenant_id = $1
  AND organization_id = $2
  AND user_id = $3
LIMIT 1
"#;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FriendRequestPrivacySettings {
    pub allow_friend_requests: bool,
}

impl Default for FriendRequestPrivacySettings {
    fn default() -> Self {
        Self {
            allow_friend_requests: true,
        }
    }
}

impl FriendRequestPrivacySettings {
    fn from_json(value: &Value) -> Self {
        let allow_friend_requests = value
            .get("allowFriendRequests")
            .or_else(|| value.get("allow_friend_requests"))
            .and_then(Value::as_bool)
            .unwrap_or(true);
        Self {
            allow_friend_requests,
        }
    }
}

pub(crate) trait SocialUserDirectory: Send + Sync {
    fn validate_friend_request_target(
        &self,
        tenant_id: &str,
        organization_id: &str,
        target_user_id: &str,
    ) -> Result<(), SocialServiceError>;
}

#[derive(Clone, Default)]
pub(crate) struct PermissiveSocialUserDirectory;

impl SocialUserDirectory for PermissiveSocialUserDirectory {
    fn validate_friend_request_target(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _target_user_id: &str,
    ) -> Result<(), SocialServiceError> {
        Ok(())
    }
}

/// Fail-closed directory used when Postgres IAM/profile stores are unavailable in production.
#[derive(Clone, Default)]
pub(crate) struct FailClosedSocialUserDirectory;

impl SocialUserDirectory for FailClosedSocialUserDirectory {
    fn validate_friend_request_target(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _target_user_id: &str,
    ) -> Result<(), SocialServiceError> {
        Err(SocialServiceError::dependency_unavailable(
            "social_user_directory_unconfigured",
            "social user directory requires Postgres in production-like environments",
        ))
    }
}

#[derive(Clone)]
pub(crate) struct PostgresSocialUserDirectory {
    pool: SocialPostgresPool,
}

impl PostgresSocialUserDirectory {
    pub(crate) fn new(pool: SocialPostgresPool) -> Self {
        Self { pool }
    }

    fn lookup_privacy(
        pool: &SocialPostgresPool,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
    ) -> Result<FriendRequestPrivacySettings, SocialServiceError> {
        let pool = pool.inner().clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let user_id = user_id.to_owned();
        let operation = move || {
            let mut client = postgres_pool_client(&pool, "friend request target privacy lookup")
                .map_err(|error| {
                    SocialServiceError::dependency_unavailable(
                        "target_user_privacy_lookup_failed",
                        format!("target user privacy lookup failed: {error:?}"),
                    )
                })?;
            let row = client
                .query_opt(
                    TARGET_PRIVACY_SQL,
                    &[&tenant_id, &organization_id, &user_id],
                )
                .map_err(|error| {
                    SocialServiceError::dependency_unavailable(
                        "target_user_privacy_lookup_failed",
                        format!("target user privacy lookup failed: {error}"),
                    )
                })?;
            Ok::<FriendRequestPrivacySettings, SocialServiceError>(
                row.map(|record| {
                    let settings: Value = record.get("im_privacy_settings");
                    FriendRequestPrivacySettings::from_json(&settings)
                })
                .unwrap_or_default(),
            )
        };
        if let Ok(handle) = tokio::runtime::Handle::try_current()
            && handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread
        {
            return tokio::task::block_in_place(operation);
        }
        std::thread::scope(|scope| {
            scope.spawn(operation).join().map_err(|_| {
                SocialServiceError::dependency_unavailable(
                    "target_user_privacy_lookup_failed",
                    "target user privacy lookup worker panicked",
                )
            })?
        })
    }
}

impl SocialUserDirectory for PostgresSocialUserDirectory {
    fn validate_friend_request_target(
        &self,
        tenant_id: &str,
        organization_id: &str,
        target_user_id: &str,
    ) -> Result<(), SocialServiceError> {
        let pool = self.pool.inner().clone();
        let tenant_id_owned = tenant_id.to_owned();
        let target_user_id_owned = target_user_id.to_owned();
        let closure_tenant_id = tenant_id_owned.clone();
        let closure_target_user_id = target_user_id_owned.clone();
        let operation = move || {
            let mut client = postgres_pool_client(&pool, "friend request target iam lookup")
                .map_err(|error| {
                    SocialServiceError::dependency_unavailable(
                        "target_user_lookup_failed",
                        format!("target user lookup failed: {error:?}"),
                    )
                })?;
            let row = client
                .query_opt(
                    IAM_USER_EXISTS_SQL,
                    &[&closure_tenant_id, &closure_target_user_id],
                )
                .map_err(|error| {
                    SocialServiceError::dependency_unavailable(
                        "target_user_lookup_failed",
                        format!("target user lookup failed: {error}"),
                    )
                })?;
            Ok::<bool, SocialServiceError>(row.is_some())
        };
        let exists = if let Ok(handle) = tokio::runtime::Handle::try_current() {
            if handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread {
                tokio::task::block_in_place(operation)
            } else {
                std::thread::scope(|scope| {
                    scope.spawn(operation).join().map_err(|_| {
                        SocialServiceError::dependency_unavailable(
                            "target_user_lookup_failed",
                            "target user lookup worker panicked",
                        )
                    })?
                })
            }
        } else {
            std::thread::scope(|scope| {
                scope.spawn(operation).join().map_err(|_| {
                    SocialServiceError::dependency_unavailable(
                        "target_user_lookup_failed",
                        "target user lookup worker panicked",
                    )
                })?
            })
        }?;
        if !exists {
            return Err(SocialServiceError::not_found(
                "friend_request_target_not_found",
                "target user was not found in the current tenant",
            ));
        }

        let privacy = Self::lookup_privacy(
            &self.pool,
            tenant_id_owned.as_str(),
            organization_id,
            target_user_id_owned.as_str(),
        )?;
        if !privacy.allow_friend_requests {
            return Err(SocialServiceError::forbidden(
                "friend_requests_disabled",
                "target user does not accept friend requests",
            ));
        }
        Ok(())
    }
}

pub(crate) fn resolve_social_user_directory_from_pool(
    pool: Option<SocialPostgresPool>,
) -> Arc<dyn SocialUserDirectory> {
    pool.map(|pool| {
        Arc::new(PostgresSocialUserDirectory::new(pool)) as Arc<dyn SocialUserDirectory>
    })
    .unwrap_or_else(|| {
        if crate::friend_request_rate_limit::is_production_like_environment() {
            Arc::new(FailClosedSocialUserDirectory)
        } else {
            Arc::new(PermissiveSocialUserDirectory)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::FriendRequestPrivacySettings;
    use serde_json::json;

    #[test]
    fn privacy_settings_default_allow_friend_requests() {
        assert!(FriendRequestPrivacySettings::default().allow_friend_requests);
    }

    #[test]
    fn privacy_settings_honors_allow_friend_requests_flag() {
        let settings = FriendRequestPrivacySettings::from_json(&json!({
            "allowFriendRequests": false
        }));
        assert!(!settings.allow_friend_requests);
    }
}
