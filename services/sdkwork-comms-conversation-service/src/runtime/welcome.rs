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
//!
//! 并发/历史兼容：同 ID 会话已存在但 bind 无法重放（历史流程创建、
//! 冷加载后绑定记录缺失、或绑定记录参与方顺序不一致）时，降级为
//! 形状校验而非 409——会话是 active 的 direct 且系统智能体与用户均为
//! active 成员则继续投递（消息按确定性 client_msg_id 幂等），否则按
//! 「已有对话」幂等跳过。该端点契约是「登录后可安全重复调用」，
//! 绝不因既有会话形态返回冲突。

use super::*;
use im_domain_core::message::MessageAttributes;

use super::support::{
    canonical_direct_chat_business_id, canonical_direct_chat_conversation_id,
    conversation_scope_key,
};

/// 系统智能体 actor 身份（与 `im_domain_events::EventActor` 的
/// `actor_id: "system" / actor_kind: "system"` 约定一致）。
pub const SYSTEM_AGENT_ACTOR_ID: &str = "system";
pub const SYSTEM_AGENT_ACTOR_KIND: &str = "system";

const WELCOME_MESSAGE_TEXT_ENV: &str = "SDKWORK_IM_CONVERSATION_WELCOME_MESSAGE_TEXT";
const WELCOME_MESSAGE_VERSION_ENV: &str = "SDKWORK_IM_CONVERSATION_WELCOME_MESSAGE_VERSION";
const WELCOME_DEFAULT_VERSION: &str = "v1";
const WELCOME_DEFAULT_TEXT: &str = "欢迎使用 SDKWork 即时通讯！我是系统智能体，随时为你提供帮助。如有任何问题，可以直接在对话中向我提问。";

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
        //    系统 actor 允许绑定任意用户）。同 ID 会话已存在但 bind 无法
        //    重放时降级形状校验；会话不可投递则按「已有对话」幂等跳过，
        //    避免对既有会话返回 409 冲突。
        let auth = system_agent_context(tenant_id, organization_id);
        let bound = self.ensure_welcome_direct_chat_bound(
            &auth,
            tenant_id,
            organization_id,
            conversation_id.as_str(),
            direct_chat_id.as_str(),
            user_id,
        )?;
        if !bound {
            return Ok(WelcomeDeliveryOutcome::AlreadyEngaged);
        }

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

    /// 确保 系统智能体↔用户 direct chat 已存在（Welcome 专用）。
    ///
    /// 常规路径走 canonical bind（不存在时创建，已存在且可重放时重放）。
    /// 当同 ID 会话已存在但 bind 判定冲突——历史流程创建（无绑定记录）、
    /// 冷加载后绑定记录缺失、或绑定记录参与方顺序与本次调用不一致——
    /// 时，降级为形状校验：会话仍可投递则视为绑定已成立（返回 `true`），
    /// 否则返回 `false` 由调用方按「已有对话」跳过。其他错误原样上抛。
    fn ensure_welcome_direct_chat_bound(
        &self,
        auth: &AppContext,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        direct_chat_id: &str,
        user_id: &str,
    ) -> Result<bool, RuntimeError> {
        let bound = self.bind_direct_chat_conversation_from_auth_context(
            auth,
            conversation_id.to_owned(),
            direct_chat_id.to_owned(),
            SYSTEM_AGENT_ACTOR_ID.to_owned(),
            SYSTEM_AGENT_ACTOR_KIND.to_owned(),
            user_id.to_owned(),
            "user".to_owned(),
        );
        match bound {
            Ok(_) => Ok(true),
            Err(RuntimeError::Conflict(_)) => self.welcome_direct_chat_shape_servable(
                tenant_id,
                organization_id,
                conversation_id,
                user_id,
            ),
            Err(error) => Err(error),
        }
    }

    /// Welcome 冲突降级：校验既有同 ID 会话是否仍可作为系统智能体会话
    /// 投递 Welcome。仅当会话存在、类型为 `direct`、生命周期 `active`、
    /// 且系统智能体与用户均为 active 成员时返回 `true`；其他形态
    /// （归档、成员缺失等）视为不可投递，返回 `false`。
    fn welcome_direct_chat_shape_servable(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        user_id: &str,
    ) -> Result<bool, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, organization_id, conversation_id);
        match self.ensure_conversation_loaded(tenant_id, organization_id, conversation_id) {
            Ok(()) => {}
            Err(RuntimeError::ConversationNotFound(_)) => return Ok(false),
            Err(error) => return Err(error),
        }
        {
            let state = read_runtime_state(&self.state, "welcome.direct_chat_shape");
            let Some(conversation) = state.conversations.get(scope_key.as_str()) else {
                return Ok(false);
            };
            if conversation.aggregate.conversation_type() != "direct"
                || conversation.aggregate.lifecycle_state() != ConversationLifecycleState::Active
            {
                return Ok(false);
            }
        }
        // 权威加载双方成员；任一非 active 成员即不可投递。
        for (kind, id) in [("system", SYSTEM_AGENT_ACTOR_ID), ("user", user_id)] {
            match self.ensure_member_loaded(tenant_id, organization_id, conversation_id, kind, id) {
                Ok(()) => {}
                Err(RuntimeError::PermissionDenied(_)) => return Ok(false),
                Err(error) => return Err(error),
            }
        }
        // 无持久化聚合存储的运行时（内存/开发）中 ensure_member_loaded
        // 是空操作，需直接校验热 roster 上的成员资格。
        let state = read_runtime_state(&self.state, "welcome.direct_chat_shape.roster");
        let Some(conversation) = state.conversations.get(scope_key.as_str()) else {
            return Ok(false);
        };
        Ok(conversation
            .roster
            .resolve_active_member_with_kind(SYSTEM_AGENT_ACTOR_ID, SYSTEM_AGENT_ACTOR_KIND)
            .is_some()
            && conversation
                .roster
                .resolve_active_member_with_kind(user_id, "user")
                .is_some())
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
            .get(&conversation_scope_key(
                "t_1",
                "0",
                &expected_conversation_id,
            ))
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

    #[test]
    fn welcome_binding_keeps_actor_kinds_aligned_when_user_id_sorts_before_system() {
        let store = Arc::new(TestWelcomeStateStore::default());
        let runtime = runtime_with_welcome_store(store.clone());

        // "ce7543e8fe13850499743e08" 字典序小于 "system"，归一化 pair 会把
        // 用户放在 anchor 位。actor kind 必须跟随归一化 pair 交换，否则成员
        // 为错配配对（("system", <user_id>) 与 ("user", "system")），Welcome
        // 消息（sender system:system）无法投递且标记不落，后续重试在冷加载后
        // 判定冲突（40901）。
        let user_id = "ce7543e8fe13850499743e08";
        let outcome = runtime
            .ensure_user_welcome("t_1", "0", user_id, None)
            .expect("welcome should be delivered");
        let WelcomeDeliveryOutcome::Sent {
            conversation_id, ..
        } = outcome
        else {
            panic!("expected sent outcome, got {outcome:?}");
        };

        let state_guard = runtime.state.read().unwrap();
        let stored = state_guard
            .conversations
            .get(&conversation_scope_key("t_1", "0", &conversation_id))
            .expect("welcome conversation should exist");
        assert!(
            stored
                .roster
                .resolve_active_member_with_kind(SYSTEM_AGENT_ACTOR_ID, SYSTEM_AGENT_ACTOR_KIND)
                .is_some(),
            "system actor member must be system:system"
        );
        assert!(
            stored
                .roster
                .resolve_active_member_with_kind(user_id, "user")
                .is_some(),
            "user member must be user:{user_id}"
        );

        // 标记已写入 → 再次调用直接短路为 AlreadySent。
        let second = runtime
            .ensure_user_welcome("t_1", "0", user_id, None)
            .expect("re-evaluation should succeed");
        assert_eq!(second, WelcomeDeliveryOutcome::AlreadySent);
    }

    /// 以系统 actor 身份创建 系统智能体↔用户 canonical direct chat，返回会话 ID。
    fn seed_system_agent_conversation(
        runtime: &ConversationRuntime<InMemoryJournal>,
        tenant_id: &str,
        organization_id: &str,
        user_id: &str,
    ) -> String {
        let auth = system_agent_context(tenant_id, organization_id);
        let direct_chat_id = canonical_direct_chat_business_id(
            SYSTEM_AGENT_ACTOR_KIND,
            SYSTEM_AGENT_ACTOR_ID,
            "user",
            user_id,
        )
        .expect("canonical direct chat id");
        let conversation_id = canonical_direct_chat_conversation_id(
            tenant_id,
            organization_id,
            direct_chat_id.as_str(),
        );
        runtime
            .bind_direct_chat_conversation_from_auth_context(
                &auth,
                conversation_id.clone(),
                direct_chat_id,
                SYSTEM_AGENT_ACTOR_ID.to_owned(),
                SYSTEM_AGENT_ACTOR_KIND.to_owned(),
                user_id.to_owned(),
                "user".to_owned(),
            )
            .expect("seed bind should succeed");
        conversation_id
    }

    #[test]
    fn preexisting_conversation_without_binding_record_is_tolerated() {
        let store = Arc::new(TestWelcomeStateStore::default());
        let runtime = runtime_with_welcome_store(store.clone());
        let user_id = "u_carol";
        let conversation_id = seed_system_agent_conversation(&runtime, "t_1", "0", user_id);

        // 模拟历史流程/冷加载形态：同 ID 会话无 direct_chat_binding_request，
        // 且 business binding 与当前 canonical 不一致 → bind 无法重放。
        let scope_key = conversation_scope_key("t_1", "0", &conversation_id);
        {
            let mut state = runtime.state.write().unwrap();
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .expect("seeded conversation should be hot");
            conversation.direct_chat_binding_request = None;
            conversation.aggregate.replace_business_binding(None);
        }

        // 降级形状校验通过 → 继续投递 Welcome，而非 409。
        let outcome = runtime
            .ensure_user_welcome("t_1", "0", user_id, None)
            .expect("welcome should tolerate a preexisting conversation without a binding record");
        assert_eq!(outcome.status_label(), "sent");
        assert!(
            store.sent.lock().unwrap().is_some(),
            "welcome marker should be written"
        );
    }

    #[test]
    fn preexisting_conversation_with_reversed_binding_record_is_replayed() {
        let store = Arc::new(TestWelcomeStateStore::default());
        let runtime = runtime_with_welcome_store(store.clone());
        let user_id = "u_dana";

        // 历史调用以 (user, system) 反序绑定：canonical 会话 ID 不变，
        // anchor/peer 顺序与 Welcome 调用相反。actor kind 必须跟随归一化
        // pair 交换，成员仍为 system:system + user:u_dana，重放后继续投递。
        let auth = system_agent_context("t_1", "0");
        let direct_chat_id = canonical_direct_chat_business_id(
            SYSTEM_AGENT_ACTOR_KIND,
            SYSTEM_AGENT_ACTOR_ID,
            "user",
            user_id,
        )
        .expect("canonical direct chat id");
        let conversation_id =
            canonical_direct_chat_conversation_id("t_1", "0", direct_chat_id.as_str());
        runtime
            .bind_direct_chat_conversation_from_auth_context(
                &auth,
                conversation_id.clone(),
                direct_chat_id,
                user_id.to_owned(),
                "user".to_owned(),
                SYSTEM_AGENT_ACTOR_ID.to_owned(),
                SYSTEM_AGENT_ACTOR_KIND.to_owned(),
            )
            .expect("seed bind should succeed");

        let outcome = runtime
            .ensure_user_welcome("t_1", "0", user_id, None)
            .expect("welcome should replay a preexisting reversed binding record");
        assert_eq!(outcome.status_label(), "sent");
        assert!(
            store.sent.lock().unwrap().is_some(),
            "welcome marker should be written"
        );
    }

    #[test]
    fn preexisting_unservable_conversation_skips_welcome_without_conflict() {
        let store = Arc::new(TestWelcomeStateStore::default());
        let runtime = runtime_with_welcome_store(store.clone());
        let user_id = "u_eve";
        let conversation_id = seed_system_agent_conversation(&runtime, "t_1", "0", user_id);

        // 会话已归档（不可投递）且无绑定记录 → 跳过而非 409。
        let scope_key = conversation_scope_key("t_1", "0", &conversation_id);
        {
            let mut state = runtime.state.write().unwrap();
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .expect("seeded conversation should be hot");
            conversation.direct_chat_binding_request = None;
            conversation
                .aggregate
                .synchronize_normalized_current_state("direct", "archived", u64::MAX, u64::MAX)
                .expect("archive sync should apply");
        }

        let outcome = runtime
            .ensure_user_welcome("t_1", "0", user_id, None)
            .expect("welcome should skip unservable conversations without conflict");
        assert_eq!(outcome, WelcomeDeliveryOutcome::AlreadyEngaged);
        assert!(
            store.sent.lock().unwrap().is_none(),
            "skipped welcome must not write the marker"
        );
    }

    #[test]
    fn preexisting_conversation_without_system_member_skips_welcome() {
        let store = Arc::new(TestWelcomeStateStore::default());
        let runtime = runtime_with_welcome_store(store.clone());
        let user_id = "u_frank";
        let conversation_id = seed_system_agent_conversation(&runtime, "t_1", "0", user_id);

        // 系统智能体不再是 active 成员（无绑定记录）→ 不可投递 → 跳过。
        let scope_key = conversation_scope_key("t_1", "0", &conversation_id);
        {
            let mut state = runtime.state.write().unwrap();
            let conversation = state
                .conversations
                .get_mut(scope_key.as_str())
                .expect("seeded conversation should be hot");
            conversation.direct_chat_binding_request = None;
            let system_member = conversation
                .roster
                .resolve_active_member(SYSTEM_AGENT_ACTOR_ID)
                .expect("system member should exist after seed");
            conversation.roster.deactivate_member(system_member);
        }

        let outcome = runtime
            .ensure_user_welcome("t_1", "0", user_id, None)
            .expect("welcome should skip conversations without the system member");
        assert_eq!(outcome, WelcomeDeliveryOutcome::AlreadyEngaged);
        assert!(store.sent.lock().unwrap().is_none());
    }
}
