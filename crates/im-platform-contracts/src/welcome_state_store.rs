// Welcome State Store Contract - 系统智能体 Welcome 消息去重状态契约

use sdkwork_im_contract_core::ContractError;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// 用户已收到 Welcome 系统消息的记录。
///
/// 持久化在 `im_user_settings` 表（setting_key = `welcome.sent`），是
/// 「不重复发送」的唯一权威标记；`im_conversations.message_count` 只用于
/// 判断「用户是否已经有过对话」（老用户跳过）。
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeSentRecord {
    pub tenant_id: String,
    pub organization_id: String,
    pub user_id: String,
    /// Welcome 所在的系统智能体 direct chat 会话 ID。
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    /// Welcome 文案版本，用于将来升级文案后按版本重发。
    pub welcome_version: String,
    /// RFC3339 发送时间。
    pub sent_at: String,
}

/// Welcome 发送去重状态存储。
pub trait WelcomeStateStore: Send + Sync {
    /// 读取用户的 `welcome.sent` 标记；从未发送过时返回 None。
    fn read_welcome_sent(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
    ) -> Result<Option<WelcomeSentRecord>, ContractError>;

    /// 幂等写入 `welcome.sent` 标记（upsert，ON CONFLICT DO UPDATE）。
    fn write_welcome_sent(&self, record: &WelcomeSentRecord) -> Result<(), ContractError>;

    /// 判断用户是否已经拥有任何「有消息」的会话（member 且 message_count > 0）。
    ///
    /// `exclude_conversation_id` 排除 Welcome 会话本身，保证消息已发送但
    /// 标记未落库（进程崩溃）的重试路径不会被误判为「已有对话」。
    fn user_has_conversations_with_messages(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
        exclude_conversation_id: &str,
    ) -> Result<bool, ContractError>;
}

/// 内存版 [`WelcomeStateStore`]，供开发/测试运行时与单元测试注入。
#[derive(Clone, Default)]
pub struct InMemoryWelcomeStateStore {
    sent: Arc<Mutex<Option<WelcomeSentRecord>>>,
    engaged_user_ids: Arc<Mutex<Vec<String>>>,
}

impl InMemoryWelcomeStateStore {
    /// 标记某用户已拥有有消息的会话（模拟「已有对话」判定）。
    pub fn mark_engaged(&self, user_id: &str) {
        let mut engaged = self.engaged_user_ids.lock().unwrap();
        if !engaged.iter().any(|value| value == user_id) {
            engaged.push(user_id.to_owned());
        }
    }
}

impl WelcomeStateStore for InMemoryWelcomeStateStore {
    fn read_welcome_sent(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        _user_id: &str,
    ) -> Result<Option<WelcomeSentRecord>, ContractError> {
        Ok(self.sent.lock().unwrap().clone())
    }

    fn write_welcome_sent(&self, record: &WelcomeSentRecord) -> Result<(), ContractError> {
        *self.sent.lock().unwrap() = Some(record.clone());
        Ok(())
    }

    fn user_has_conversations_with_messages(
        &self,
        _tenant_id: &str,
        _organization_id: &str,
        user_id: &str,
        _exclude_conversation_id: &str,
    ) -> Result<bool, ContractError> {
        Ok(self
            .engaged_user_ids
            .lock()
            .unwrap()
            .iter()
            .any(|engaged| engaged == user_id))
    }
}
