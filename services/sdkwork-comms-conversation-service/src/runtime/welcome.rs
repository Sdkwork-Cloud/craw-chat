//! 系统智能体 Welcome 消息投递（幂等去重）。
//!
//! 规则（与产品约定一致）：
//! - 用户已收到过 Welcome（`im_user_settings.welcome.sent`）→ 跳过；
//! - 用户已与其他人有过对话（存在 `message_count > 0` 的会话）→ 跳过；
//! - 否则确保 系统智能体↔用户 direct chat 存在，并以
//!   `MessageType::System` 投递 Welcome 消息后写入标记。
//!
//! 并发/崩溃安全依赖两个幂等锚点：
//! - 会话 ID 由 `canonical_direct_chat_*` 确定性推导，绑定请求按
//!   request-key 重放；
//! - 消息使用确定性 `client_msg_id = "welcome:{user_id}"`，投递路径
//!   按 request-key / client-id 重放，任何并发或重试都不会产生第二条
//!   Welcome 消息。
//! - 「已有对话」判定排除 Welcome 会话本身，保证消息已落库而标记未写
//!   （进程崩溃）时，重试路径不会被误判为「已有对话」。

use super::*;
use im_domain_core::message::MessageAttributes;

use super::support::{
    canonical_direct_chat_business_id, canonical_direct_chat_conversation_id,
};

/// 系统智能体 actor 身份（与 `im_domain_events::EventActor` 的
/// `actor_id: "system" / actor_kind: "system"` 约定一致）。
pub const SYSTEM_AGENT_ACTOR_ID: &str = "system";
pub const SYSTEM_AGENT_ACTOR_KIND: &str = "system";

const WELCOME_MESSAGE_TEXT_ENV: &str = "SDKWORK_IM_CONVERSATION_WELCOME_MESSAGE_TEXT";
const WELCOME_MESSAGE_VERSION_ENV: &str = "SDKWORK_IM_CONVERSATION_WELCOME_MESSAGE_VERSION";
const WELCOME_DEFAULT_VERSION: &str = "v1";
const WELCOME_DEFAULT_TEXT: &str =
    "欢迎使用 SDKWork 即时通讯！我是系统智能体，随时为你提供帮助。如有任何问题，可以直接在对话中向我提问。";

/// Welcome 投递结果。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WelcomeDeliveryOutcome {
    /// 本次成功投递（或完成投递后落标记）。
    Sent {
        conversation_id: String,
        message_id: String,
        message_seq: u64,
    },
    /// 该用户此前已收到过 Welcome，未重复投递。
    AlreadySent,
    /// 该用户已与其他参与者有过对话，跳过 Welcome。
    AlreadyEngaged,
}

impl WelcomeDeliveryOutcome {
    pub fn status_label(&self) -> &'static str {
        match self {
            Self::Sent { .. } => "sent",
            Self::AlreadySent => "already_sent",
            Self::AlreadyEngaged => "already_engaged",
        }
    }
}

/// HTTP 对外返回的 Welcome 检查/投递结果视图。
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WelcomeEnsureView {
    pub status: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
}

impl From<&WelcomeDeliveryOutcome> for WelcomeEnsureView {
    fn from(outcome: &WelcomeDeliveryOutcome) -> Self {
        match outcome {
            WelcomeDeliveryOutcome::Sent {
                conversation_id,
                message_id,
                message_seq,
            } => Self {
                status: outcome.status_label().to_owned(),
                conversation_id: conversation_id.clone(),
                message_id: message_id.clone(),
                message_seq: *message_seq,
            },
            _ => Self {
                status: outcome.status_label().to_owned(),
                conversation_id: String::new(),
                message_id: String::new(),
                message_seq: 0,
            },
        }
    }
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    /// 确保用户收到系统智能体的 Welcome 系统消息（幂等）。
    ///
    /// `text_override` 为 Some 且非空时覆盖服务端配置文案
    /// （`SDKWORK_IM_CONVERSATION_WELCOME_MESSAGE_TEXT`）。
    pub fn ensure_user_welcome(
        &self,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
        text_override: Option<&str>,
    ) -> Result<WelcomeDeliveryOutcome, RuntimeError> {
        let store = self.welcome_state_store.as_ref().ok_or_else(|| {
            RuntimeError::Contract(ContractError::Unavailable(
                "welcome state store is not wired; ensure_user_welcome requires a WelcomeStateStore"
                    .into(),
            ))
        })?;

        let direct_chat_id = canonical_direct_chat_business_id(
            SYSTEM_AGENT_ACTOR_KIND,
            SYSTEM_AGENT_ACTOR_ID,
            "user",
            user_id,
        )?;
        let conversation_id = canonical_direct_chat_conversation_id(
            tenant_id,
            organization_id,
            direct_chat_id.as_str(),
        );

        // 1. 已发送过 → 跳过。
        if store
            .read_welcome_sent(tenant_id, organization_id, user_id)?
            .is_some()
        {
            return Ok(WelcomeDeliveryOutcome::AlreadySent);
        }

        // 2. 已有对话 → 跳过（排除 Welcome 会话本身）。
        if store.user_has_conversations_with_messages(
            tenant_id,
            organization_id,
            user_id,
            conversation_id.as_str(),
        )? {
            return Ok(WelcomeDeliveryOutcome::AlreadyEngaged);
        }

        // 3. 确保 系统智能体↔用户 direct chat 存在（canonical ID 幂等，
        //    系统 actor 允许绑定任意用户）。
        let auth = system_agent_context(tenant_id, organization_id);
        self.bind_direct_chat_conversation_from_auth_context(
            &auth,
            conversation_id.clone(),
            direct_chat_id,
            SYSTEM_AGENT_ACTOR_ID.to_owned(),
            SYSTEM_AGENT_ACTOR_KIND.to_owned(),
            user_id.to_owned(),
            "user".to_owned(),
        )?;

        // 4. 投递 Welcome 系统消息。确定性 client_msg_id 使并发或崩溃后
        //    重试只会重放同一条消息。
        let welcome_text = resolve_welcome_message_text(text_override);
        let result = self.post_message(PostMessageCommand {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            conversation_id: conversation_id.clone(),
            sender: Sender {
                id: SYSTEM_AGENT_ACTOR_ID.to_owned(),
                kind: SYSTEM_AGENT_ACTOR_KIND.to_owned(),
                member_id: None,
                device_id: None,
                session_id: None,
                metadata: MessageAttributes::default(),
            },
            client_msg_id: Some(format!("welcome:{user_id}")),
            message_type: MessageType::System,
            body: MessageBody {
                summary: None,
                parts: vec![ContentPart::text(welcome_text)],
                render_hints: MessageAttributes::default(),
                reply_to: None,
            },
        })?;

        // 5. 落标记（upsert 幂等）。即使第 4 步实际是重放（崩溃恢复），
        //    也补写标记，避免后续重复评估。
        store.write_welcome_sent(&WelcomeSentRecord {
            tenant_id: tenant_id.to_owned(),
            organization_id: organization_id.to_owned(),
            user_id: user_id.to_owned(),
            conversation_id: conversation_id.clone(),
            message_id: result.message_id.clone(),
            message_seq: result.message_seq,
            welcome_version: resolve_welcome_version(),
            sent_at: utc_now_rfc3339_millis(),
        })?;

        Ok(WelcomeDeliveryOutcome::Sent {
            conversation_id,
            message_id: result.message_id,
            message_seq: result.message_seq,
        })
    }
}

fn resolve_welcome_message_text(text_override: Option<&str>) -> String {
    text_override
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| {
            std::env::var(WELCOME_MESSAGE_TEXT_ENV)
                .ok()
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| WELCOME_DEFAULT_TEXT.to_owned())
        })
}

fn resolve_welcome_version() -> String {
    std::env::var(WELCOME_MESSAGE_VERSION_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| WELCOME_DEFAULT_VERSION.to_owned())
}

fn system_agent_context(tenant_id: &str, organization_id: &str) -> AppContext {
    AppContext {
        tenant_id: tenant_id.to_owned(),
        organization_id: organization_id.to_owned(),
        user_id: SYSTEM_AGENT_ACTOR_ID.to_owned(),
        session_id: None,
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: Default::default(),
        permission_scope: Default::default(),
        actor_id: SYSTEM_AGENT_ACTOR_ID.to_owned(),
        actor_kind: SYSTEM_AGENT_ACTOR_KIND.to_owned(),
        device_id: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    struct TestWelcomeStateStore {
        sent: Mutex<Option<WelcomeSentRecord>>,
        conversations_with_messages: Mutex<Vec<String>>,
    }

    struct AllowAllDirectMessageAccessGate;

    impl DirectMessageAccessGate for AllowAllDirectMessageAccessGate {
        fn ensure_direct_message_allowed(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _sender_user_id: &str,
            _peer_user_id: &str,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    impl WelcomeStateStore for TestWelcomeStateStore {
        fn read_welcome_sent(
            &self,
            _tenant_id: &str,
            _organization_id: &str,
            _user_id: &str,
        ) -> Result<Option<WelcomeSentRecord>, ContractError> {
            Ok(self.sent.lock().unwrap().clone())
        }

        fn write_welcome_sent(
            &self,
            record: &WelcomeSentRecord,
        ) -> Result<(), ContractError> {
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
                .conversations_with_messages
                .lock()
                .unwrap()
                .iter()
                .any(|engaged| engaged == user_id))
        }
    }

    fn runtime_with_welcome_store(
        store: Arc<TestWelcomeStateStore>,
    ) -> ConversationRuntime<InMemoryJournal> {
        ConversationRuntime::new(InMemoryJournal::default())
            .with_welcome_state_store(store)
            .with_direct_message_access_gate(Arc::new(AllowAllDirectMessageAccessGate))
    }

    #[test]
    fn new_user_receives_welcome_system_message_once() {
        let store = Arc::new(TestWelcomeStateStore::default());
        let runtime = runtime_with_welcome_store(store.clone());

        let first = runtime
            .ensure_user_welcome("t_1", "0", "u_alice", None)
            .expect("welcome should be delivered");
        let WelcomeDeliveryOutcome::Sent {
            conversation_id,
            message_id,
            ..
        } = &first
        else {
            panic!("expected sent outcome, got {first:?}");
        };

        // 会话是系统智能体↔用户的 direct chat。
        let expected_conversation_id = canonical_direct_chat_conversation_id(
            "t_1",
            "0",
            &canonical_direct_chat_business_id("system", "system", "user", "u_alice")
                .expect("canonical direct chat id"),
        );
        assert_eq!(conversation_id, &expected_conversation_id);

        // 消息已按 MessageType::System 投递。
        let state_guard = runtime.state.read().unwrap();
        let stored = state_guard
            .conversations
            .get(&conversation_scope_key("t_1", "0", &expected_conversation_id))
            .expect("welcome conversation should exist");
        let message = stored
            .message_log
            .message(message_id.as_str())
            .expect("welcome message should be logged");
        assert_eq!(message.message.message_type, MessageType::System);
        assert_eq!(message.message.sender.kind, SYSTEM_AGENT_ACTOR_KIND);
        assert_eq!(message.message.sender.id, SYSTEM_AGENT_ACTOR_ID);

        // 标记已写入。
        assert!(store.sent.lock().unwrap().is_some());

        // 再次调用 → 不再重复发送。
        let second = runtime
            .ensure_user_welcome("t_1", "0", "u_alice", None)
            .expect("re-evaluation should succeed");
        assert_eq!(second, WelcomeDeliveryOutcome::AlreadySent);
    }

    #[test]
    fn engaged_user_skips_welcome() {
        let store = Arc::new(TestWelcomeStateStore::default());
        store
            .conversations_with_messages
            .lock()
            .unwrap()
            .push("u_bob".to_owned());
        let runtime = runtime_with_welcome_store(store.clone());

        let outcome = runtime
            .ensure_user_welcome("t_1", "0", "u_bob", None)
            .expect("evaluation should succeed");
        assert_eq!(outcome, WelcomeDeliveryOutcome::AlreadyEngaged);
        assert!(store.sent.lock().unwrap().is_none());
    }

    #[test]
    fn text_override_and_default_text_are_resolved() {
        assert_eq!(
            resolve_welcome_message_text(Some("  hello  ")),
            "hello",
            "override 应去除首尾空白"
        );
        assert_eq!(
            resolve_welcome_message_text(Some("  ")),
            resolve_welcome_message_text(None),
            "空白 override 回落到服务端配置/默认文案"
        );
        let default = resolve_welcome_message_text(None);
        assert!(!default.is_empty());
        assert_eq!(resolve_welcome_version(), "v1");
    }

    #[test]
    fn welcome_requires_wired_state_store() {
        let runtime = ConversationRuntime::new(InMemoryJournal::default());
        let error = runtime
            .ensure_user_welcome("t_1", "0", "u_alice", None)
            .expect_err("missing welcome state store must fail");
        assert!(error.to_string().contains("welcome state store"));
    }
}
