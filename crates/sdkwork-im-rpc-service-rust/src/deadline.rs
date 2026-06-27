use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RpcDeadline {
    millis: u64,
}

impl RpcDeadline {
    pub const fn from_millis(millis: u64) -> Self {
        Self { millis }
    }

    pub const fn as_millis(self) -> u64 {
        self.millis
    }

    pub fn as_duration(self) -> Duration {
        Duration::from_millis(self.millis)
    }
}

impl Default for RpcDeadline {
    fn default() -> Self {
        Self::from_millis(30_000)
    }
}
