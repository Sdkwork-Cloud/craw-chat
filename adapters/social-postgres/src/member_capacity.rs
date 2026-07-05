//! Atomic member capacity outcomes shared by space and group member stores.

/// Result of an insert that enforces a maximum member count in SQL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberInsertOutcome {
    /// Row inserted successfully.
    Inserted,
    /// Member already exists (idempotent conflict).
    AlreadyExists,
    /// Capacity limit would be exceeded.
    CapacityFull,
}
