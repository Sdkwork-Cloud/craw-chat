//! Fail-closed policy for supplemental Postgres social routes.

use sdkwork_routes_web_framework_backend_api::response::ApiProblem;

pub(crate) fn supplemental_social_mutation_forbidden() -> ApiProblem {
    ApiProblem::forbidden(
        "supplemental postgres social routes are read-only; use event-sourced /im/v3/api/social or control-plane /backend/v3/api/control/social mutations",
    )
}
