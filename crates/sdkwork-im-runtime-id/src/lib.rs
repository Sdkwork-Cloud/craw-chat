use sdkwork_id::{
    NodeAllocatorConfig, NodeAllocatorError, NodeLease, SnowflakeIdError, SnowflakeIdGenerator,
    SnowflakeNodeAllocator,
};

pub const SDKWORK_IM_ID_NODE_ID_ENV: &str = "SDKWORK_IM_ID_NODE_ID";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeIdStrategy {
    pub id_type: &'static str,
    pub clock_rollback: &'static str,
    pub node_conflict: &'static str,
    pub sequence_overflow: &'static str,
    pub restart_recovery: &'static str,
    pub failure_handling: &'static str,
    pub public_id: &'static str,
}

pub fn runtime_id_strategy() -> RuntimeIdStrategy {
    RuntimeIdStrategy {
        id_type: "snowflake",
        clock_rollback: "reject_and_alert",
        node_conflict: "database_backed_auto_allocation",
        sequence_overflow: "fail_closed",
        restart_recovery: "idempotent_lease_reclaim",
        failure_handling: "database_first_then_env_fallback",
        public_id: "uuid_or_business_id",
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeIdConfig {
    pub node_id: u16,
}

impl RuntimeIdConfig {
    pub fn from_env() -> Result<Self, RuntimeIdError> {
        Self::from_env_pairs(std::env::vars())
    }

    pub fn from_env_pairs<I, K, V>(pairs: I) -> Result<Self, RuntimeIdError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let Some(raw_node_id) = pairs.into_iter().find_map(|(name, value)| {
            (name.as_ref() == SDKWORK_IM_ID_NODE_ID_ENV).then(|| value.as_ref().trim().to_owned())
        }) else {
            return Err(RuntimeIdError::MissingNodeId);
        };

        if raw_node_id.is_empty() {
            return Err(RuntimeIdError::MissingNodeId);
        }

        let node_id =
            raw_node_id
                .parse::<u16>()
                .map_err(|error| RuntimeIdError::InvalidNodeIdConfig {
                    env_name: SDKWORK_IM_ID_NODE_ID_ENV,
                    value: raw_node_id.clone(),
                    message: error.to_string(),
                })?;

        SnowflakeIdGenerator::new(node_id).map_err(RuntimeIdError::Snowflake)?;

        Ok(Self { node_id })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeIdError {
    MissingNodeId,
    InvalidNodeIdConfig {
        env_name: &'static str,
        value: String,
        message: String,
    },
    Snowflake(SnowflakeIdError),
    NodeAllocation(String),
}

impl std::fmt::Display for RuntimeIdError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingNodeId => write!(
                formatter,
                "{SDKWORK_IM_ID_NODE_ID_ENV} is required for runtime Snowflake ID generation"
            ),
            Self::InvalidNodeIdConfig {
                env_name,
                value,
                message,
            } => write!(
                formatter,
                "{env_name} must be an unsigned 16-bit integer Snowflake node id, got `{value}`: {message}"
            ),
            Self::Snowflake(error) => {
                write!(formatter, "Snowflake ID generation failed: {error:?}")
            }
            Self::NodeAllocation(msg) => {
                write!(formatter, "Snowflake node_id allocation failed: {msg}")
            }
        }
    }
}

impl std::error::Error for RuntimeIdError {}

impl From<SnowflakeIdError> for RuntimeIdError {
    fn from(error: SnowflakeIdError) -> Self {
        Self::Snowflake(error)
    }
}

impl From<NodeAllocatorError> for RuntimeIdError {
    fn from(error: NodeAllocatorError) -> Self {
        Self::NodeAllocation(error.to_string())
    }
}

#[derive(Debug)]
pub struct RuntimeSnowflakeIdGenerator {
    inner: SnowflakeIdGenerator,
    /// Keeps the database heartbeat alive while the generator is in use.
    /// `None` when constructed from a static env-based node_id (legacy path).
    _lease: Option<NodeLease>,
}

impl RuntimeSnowflakeIdGenerator {
    pub fn from_env() -> Result<Self, RuntimeIdError> {
        Self::from_config(RuntimeIdConfig::from_env()?)
    }

    pub fn from_config(config: RuntimeIdConfig) -> Result<Self, RuntimeIdError> {
        Self::with_node_id(config.node_id)
    }

    pub fn with_node_id(node_id: u16) -> Result<Self, RuntimeIdError> {
        Ok(Self {
            inner: SnowflakeIdGenerator::new(node_id)?,
            _lease: None,
        })
    }

    /// Allocate a node_id from the IM database and create a generator.
    ///
    /// This is the recommended constructor for production: it automatically
    /// discovers a unique, stable `node_id` from the `sdkwork_node_registry`
    /// table, eliminating manual `SDKWORK_IM_ID_NODE_ID` configuration.
    ///
    /// The `service_name` parameter identifies the logical service (e.g.
    /// `"social-service"`, `"space-service"`) for the node registry.
    /// The database pool is created from `SDKWORK_IM_DATABASE_*` env vars.
    ///
    /// # Idempotency
    ///
    /// On restart, the same `service_name` + hostname will reclaim its
    /// previous `node_id`, ensuring stable ID sequences.
    pub async fn from_database_env(service_name: &str) -> Result<Self, RuntimeIdError> {
        let (generator, lease) =
            SnowflakeNodeAllocator::allocate_generator_from_env(service_name, "IM").await?;
        Ok(Self {
            inner: generator,
            _lease: Some(lease),
        })
    }

    /// Allocate a node_id from an existing database pool.
    ///
    /// Use this when the service already has a [`sdkwork_database_sqlx::DatabasePool`]
    /// and wants to avoid creating a second pool.
    pub async fn from_database_pool(
        pool: &sdkwork_database_sqlx::DatabasePool,
        service_name: &str,
    ) -> Result<Self, RuntimeIdError> {
        let config = NodeAllocatorConfig::from_service_name(service_name);
        let (generator, lease) =
            SnowflakeNodeAllocator::allocate_generator(pool, &config).await?;
        Ok(Self {
            inner: generator,
            _lease: Some(lease),
        })
    }

    pub fn next_id(&self) -> Result<i64, RuntimeIdError> {
        self.inner.generate().map_err(RuntimeIdError::Snowflake)
    }

    pub fn next_id_at(&self, now_millis: u64) -> Result<i64, RuntimeIdError> {
        self.inner
            .generate_at(now_millis)
            .map_err(RuntimeIdError::Snowflake)
    }

    pub fn node_id(&self) -> u16 {
        self.inner.node_id()
    }
}

// ---------------------------------------------------------------------------
// IdGenerator trait implementation
// ---------------------------------------------------------------------------

use im_platform_contracts::{ContractError, IdGenerator};

impl IdGenerator for RuntimeSnowflakeIdGenerator {
    fn next_id(&self) -> Result<i64, ContractError> {
        self.inner.generate().map_err(|error| {
            ContractError::Unavailable(format!("snowflake id generation failed: {error:?}"))
        })
    }

    fn node_id(&self) -> u16 {
        self.inner.node_id()
    }

    fn next_id_at(&self, timestamp_millis: u64) -> Result<i64, ContractError> {
        self.inner.generate_at(timestamp_millis).map_err(|error| {
            ContractError::Unavailable(format!("snowflake id generation failed: {error:?}"))
        })
    }
}
