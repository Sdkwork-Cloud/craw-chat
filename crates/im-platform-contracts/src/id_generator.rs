// ID Generator Contract - ID 生成器契约
// 统一 Snowflake ID 生成策略，废弃字符串拼接

use sdkwork_im_contract_core::ContractError;

/// ID 生成器契约
///
/// 所有持久化实体的主键 ID 统一由 Snowflake 生成：
/// - message_id: 消息唯一标识
/// - member_id: 成员唯一标识  
/// - event_id: 事件唯一标识
/// - outbox_id: Outbox 事件唯一标识
///
/// Snowflake 布局：41 timestamp + 10 node + 12 sequence
/// 单节点单毫秒可生成 4096 个 ID
pub trait IdGenerator: Send + Sync {
    /// 生成下一个唯一 ID
    fn next_id(&self) -> Result<i64, ContractError>;

    /// 获取当前节点 ID
    fn node_id(&self) -> u16;

    /// 生成指定时间戳的 ID（用于测试或回填）
    fn next_id_at(&self, timestamp_millis: u64) -> Result<i64, ContractError>;
}

/// ID 生成器配置
#[derive(Clone, Debug, Default)]
pub struct IdGeneratorConfig {
    /// 节点 ID（0-1023）
    pub node_id: u16,
    /// 自定义 epoch（可选，默认 2024-01-01 00:00:00 UTC）
    pub epoch_millis: Option<u64>,
}

impl IdGeneratorConfig {
    /// 从环境变量读取配置
    ///
    /// SDKWORK_IM_ID_NODE_ID: 节点 ID（必须唯一，0-1023）
    pub fn from_env() -> Self {
        let node_id = std::env::var("SDKWORK_IM_ID_NODE_ID")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(0);
        Self {
            node_id,
            epoch_millis: None,
        }
    }
}
