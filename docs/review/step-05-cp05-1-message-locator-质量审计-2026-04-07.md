# Step 05 CP05-1 MessageLocatorIndex 质量审计 - 2026-04-07

## 1. 审计范围

- `crates/im-domain-core/src/message.rs`
- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/recovery.rs`
- `services/conversation-runtime/src/runtime/support.rs`
- `crates/im-domain-core/tests/conversation_domain_builder_test.rs`
- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`

## 2. 审计结论

- 通过
  - `conversation-runtime` 不再直接持有 `message_index: HashMap<String, String>`
  - `MessageLocatorIndex` 已进入 domain crate，并被 runtime live/replay 路径统一消费
  - 本轮没有留下新增 warning
- 未通过项不在本轮范围
  - `CP05-2`
  - `CP05-3`
  - `CP05-4`

## 3. 关键检查点

- TDD 检查
  - 先写失败测试，再做实现，最后跑完整包验证
- 结构边界检查
  - `RuntimeState` 只保留 `message_locator: MessageLocatorIndex`
  - replay 路径不再写 `.message_index`
- 行为回归检查
  - edit / recall / replay / projection / local node 相关包测试全部通过

## 4. 风险与复盘

- `CP05-1` 之前无法闭环，并不是 message aggregate owner 不足，而是 runtime 里仍残留一个跨会话定位表。
- 将这个表纳入 domain crate 之后，message 主链路 owner 才真正闭环。
- 当前剩余风险已经被收敛到 `CP05-2 / CP05-3 / CP05-4`，下一轮不应继续停留在 `CP05-1`。
