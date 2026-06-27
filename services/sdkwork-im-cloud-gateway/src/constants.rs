//! Gateway-wide compile-time constants: env var names, route group tables,
//! websocket/HTTP body and timeout limits, and reserved header allow-lists.

pub(crate) const BROWSER_ORIGINS_ENV: &str = "SDKWORK_IM_BROWSER_ORIGINS";

pub(crate) const COMMERCE_T1_ROUTE_GROUPS: &[(&str, &[&str])] = &[
    (
        "sdkwork-account-app-api",
        &["accounts", "addresses", "billing", "wallet"],
    ),
    ("sdkwork-catalog-app-api", &["catalog"]),
    (
        "sdkwork-order-app-api",
        &[
            "cart",
            "checkout",
            "orders",
            "after_sales",
            "fulfillments",
            "shipments",
            "refunds",
        ],
    ),
    (
        "sdkwork-payment-app-api",
        &["payments", "recharges"],
    ),
    ("sdkwork-shop-app-api", &["shops"]),
    ("sdkwork-membership-app-api", &["memberships"]),
    ("sdkwork-promotion-app-api", &["promotions"]),
    ("sdkwork-invoice-app-api", &["invoices"]),
];

pub(crate) const COURSE_APP_API_SEGMENTS: &[&str] = &[
    "course_categories",
    "courses",
    "course_offerings",
    "course_enrollments",
    "course_lessons",
    "course_live_sessions",
    "course_comments",
    "course_reactions",
    "course_applications",
];

pub(crate) const WEBSOCKET_UPSTREAM_CONNECT_TIMEOUT_SECONDS: u64 = 5;
pub(crate) const GATEWAY_MAX_WEBSOCKET_MESSAGE_BYTES: usize = 512 * 1024;
pub(crate) const GATEWAY_MAX_WEBSOCKET_FRAME_BYTES: usize = 256 * 1024;

pub(crate) const GATEWAY_MAX_REQUEST_BODY_BYTES_ENV: &str = "SDKWORK_IM_GATEWAY_MAX_REQUEST_BODY_BYTES";
pub(crate) const GATEWAY_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
pub(crate) const GATEWAY_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;

pub(crate) const GATEWAY_UPSTREAM_TIMEOUT_SECONDS_ENV: &str = "SDKWORK_IM_GATEWAY_UPSTREAM_TIMEOUT_SECONDS";
pub(crate) const GATEWAY_UPSTREAM_TIMEOUT_SECONDS_DEFAULT: u64 = 60;
pub(crate) const GATEWAY_UPSTREAM_TIMEOUT_SECONDS_MAX: u64 = 300;

pub(crate) const GATEWAY_MAX_UPSTREAM_RESPONSE_BODY_BYTES_ENV: &str =
    "SDKWORK_IM_GATEWAY_MAX_UPSTREAM_RESPONSE_BODY_BYTES";
pub(crate) const GATEWAY_MAX_UPSTREAM_RESPONSE_BODY_BYTES_DEFAULT: usize = 20 * 1024 * 1024;
pub(crate) const GATEWAY_MAX_UPSTREAM_RESPONSE_BODY_BYTES_MAX: usize = 100 * 1024 * 1024;

pub(crate) const SDKWORK_CONTEXT_PROJECTION_HEADERS: &[&str] = &[
    "x-sdkwork-app-id",
    "x-sdkwork-tenant-id",
    "x-sdkwork-organization-id",
    "x-sdkwork-user-id",
    "x-sdkwork-session-id",
    "x-sdkwork-environment",
    "x-sdkwork-deployment-mode",
    "x-sdkwork-auth-level",
    "x-sdkwork-data-scope",
    "x-sdkwork-permission-scope",
    "x-sdkwork-actor-id",
    "x-sdkwork-actor-kind",
    "x-sdkwork-device-id",
    "x-sdkwork-context-signature",
];

pub(crate) const GATEWAY_WEBSOCKET_ALLOW_QUERY_TOKENS_ENV: &str =
    "SDKWORK_IM_GATEWAY_ALLOW_WEBSOCKET_QUERY_TOKENS";

pub(crate) const SDKWORK_INTERNAL_HEADER_PREFIX: &str = concat!("x-sdk", "work-");
