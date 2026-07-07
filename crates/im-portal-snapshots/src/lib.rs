//! Portal snapshot builders backed by ops diagnostics and audit records.
//!
//! Console and workspace clients consume these snapshots through `/app/v3/api/portal/*`.

mod modules;
mod snapshots;

pub use modules::COMMERCIAL_RUNTIME_MODULES;
pub use snapshots::{
    PortalSnapshot, PortalWorkspaceView, build_portal_access_snapshot,
    build_portal_automation_snapshot, build_portal_conversations_snapshot,
    build_portal_dashboard_snapshot, build_portal_governance_snapshot, build_portal_home_snapshot,
    build_portal_media_snapshot, build_portal_realtime_snapshot, build_portal_snapshot_for_section,
    build_portal_workspace_view,
};
