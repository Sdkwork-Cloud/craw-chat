use std::sync::Arc;

use sdkwork_web_bootstrap::{ReadinessCheck, ReadinessFuture};
use session_gateway::ServiceReadiness;

#[derive(Clone)]
struct RealtimePlaneReadinessCheck {
    readiness: ServiceReadiness,
}

impl RealtimePlaneReadinessCheck {
    fn new(readiness: ServiceReadiness) -> Self {
        Self { readiness }
    }
}

impl ReadinessCheck for RealtimePlaneReadinessCheck {
    fn check(&self) -> ReadinessFuture<'_> {
        let readiness = self.readiness.clone();
        Box::pin(async move {
            if readiness.is_ready().await {
                Ok(())
            } else {
                Err("embedded realtime plane is unavailable or draining".to_owned())
            }
        })
    }
}

fn gateway_runtime_readiness_checks(
    realtime_readiness: ServiceReadiness,
    agents_readiness: Option<Arc<dyn ReadinessCheck>>,
) -> Vec<Arc<dyn ReadinessCheck>> {
    let mut checks: Vec<Arc<dyn ReadinessCheck>> = vec![Arc::new(
        RealtimePlaneReadinessCheck::new(realtime_readiness),
    )];
    if let Some(check) = agents_readiness {
        checks.push(check);
    }
    checks
}

pub async fn resolve_required_gateway_readiness_check(
    realtime_readiness: ServiceReadiness,
    agents_readiness: Option<Arc<dyn ReadinessCheck>>,
) -> Arc<dyn ReadinessCheck> {
    sdkwork_im_service_readiness::resolve_gateway_readiness_check_with_required_checks(
        gateway_runtime_readiness_checks(realtime_readiness, agents_readiness),
    )
    .await
}

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use sdkwork_im_service_readiness::compose_im_required_readiness_checks;
    use sdkwork_web_bootstrap::{READINESS_DEPENDENCY_UNAVAILABLE, readyz_handler};

    use super::*;

    #[derive(Clone)]
    struct SecretBearingFailure;

    impl ReadinessCheck for SecretBearingFailure {
        fn check(&self) -> ReadinessFuture<'_> {
            Box::pin(async {
                Err("postgres://operator:top-secret@internal-db.example/im".to_owned())
            })
        }
    }

    #[tokio::test]
    async fn realtime_draining_fails_gateway_readiness() {
        let readiness = ServiceReadiness::default();
        let check = RealtimePlaneReadinessCheck::new(readiness.clone());
        check
            .check()
            .await
            .expect("active realtime plane should be ready");

        readiness.mark_draining();

        check
            .check()
            .await
            .expect_err("draining realtime plane must fail readiness");
    }

    #[tokio::test]
    async fn readyz_returns_sanitized_503_for_required_dependency_failure() {
        let check = compose_im_required_readiness_checks(vec![Arc::new(SecretBearingFailure)]);
        let response = readyz_handler(Some(check)).await;

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = to_bytes(response.into_body(), 4 * 1024)
            .await
            .expect("readiness response body");
        let body = String::from_utf8(body.to_vec()).expect("readiness response utf-8");
        assert!(body.contains(READINESS_DEPENDENCY_UNAVAILABLE));
        assert!(!body.contains("top-secret"));
        assert!(!body.contains("internal-db.example"));
        assert!(!body.contains("postgres://"));
    }
}
