//! Contact Service smoke tests.
//!
//! `contact-service` is a deprecated compatibility shim that re-exports
//! `social-service` supplemental handlers and route builders from
//! `sdkwork-routes-im-social-open-api`. These tests verify the shim stays
//! wired correctly at compile time — re-exports resolve with the expected
//! signatures — without depending on a live PostgreSQL pool. Runtime
//! behavior (infra routes, business routes) is covered by `social-service`
//! integration tests that have access to a real pool.

use axum::Router;

/// The deprecated shim must continue to re-export `AppState` so downstream
/// callers that still reference `contact_service::AppState` keep compiling.
#[test]
fn shim_re_exports_app_state_alias() {
    // Compile-time assertion: the type alias exists and is reachable.
    let _ = std::marker::PhantomData::<contact_service::AppState>;
}

/// The shim must re-export `build_supplemental_app` with the canonical
/// signature `(PostgresAppState) -> Router` from
/// `sdkwork-routes-im-social-open-api`. Verifying the function pointer type
/// at compile time is sufficient — it proves the re-export resolves and the
/// signature matches without constructing a real Postgres pool.
#[test]
fn shim_re_exports_build_supplemental_app() {
    let _ = std::marker::PhantomData::<fn(contact_service::AppState) -> Router>;
    // Compile-time assertion: the re-exported symbol is callable with the
    // expected signature. We do not invoke it (that requires a live pool),
    // but referencing it proves the re-export path resolves.
    let _: fn(contact_service::AppState) -> Router = contact_service::build_supplemental_app;
}

/// The shim must re-export `build_supplemental_public_app` with the canonical
/// signature `(PostgresAppState) -> Router` from
/// `sdkwork-routes-im-social-open-api`.
#[test]
fn shim_re_exports_build_supplemental_public_app() {
    let _: fn(contact_service::AppState) -> Router = contact_service::build_supplemental_public_app;
}

/// The shim must re-export the pool/state constructor helpers so callers
/// wiring the deprecated service can still build a `PostgresAppState` from a
/// database URL environment variable or an existing pool.
#[test]
fn shim_re_exports_pool_helpers() {
    // Reference the helpers to prove they resolve through the shim.
    let _ = contact_service::app_state_from_postgres_pool;
    let _ = contact_service::try_postgres_app_state_from_database_url_env;
}
