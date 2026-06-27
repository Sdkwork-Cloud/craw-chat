> Migrated from `docs/review/step-08-cp08-2-质量审计与复盘-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 08 / CP08-2 质量审计与复盘 - 2026-04-07

## 审计范围
- `crates/im-domain-core/src/automation.rs`
- `services/automation-service/src/lib.rs`
- `services/automation-service/tests/agent_response_lifecycle_test.rs`

## 审计结论
- 本轮未发现阻塞 `CP08-2` 交付的剩余缺陷。
- Agent 响应流与 tool call 生命周期已经具备可重复执行的运行时验证，不再停留在 execution 完成即返回结果 JSON 的单点行为。
- 实现仍然遵守 Step 08 的边界：先收口 Agent 路径，不把设备接入、MQTT、telemetry 一次堆进本轮。

## 正向结果
- Agent 响应流直接复用 `StreamSession / StreamFrame`，没有创建与 stream 主链路平行的私有模型。
- delta frame 的 `sender` 来自 `AgentSubject::sender(...)`，保持 `CP08-1` 刚建立的统一主体语义。
- tool call 生命周期有了结构化 `AgentToolCall` 记录，后续接设备或更多能力时不必回退到字符串拼装。
- commit journal 已能记录 execution 之后的 Agent 行为事件，为后续 projection / audit 提供最小证据。

## 本轮发现并修正的问题
- `automation-service` 原先无法表达 Agent 响应流开始、delta、完成这三个生命周期节点。
- `automation-service` 原先无法表达 tool call request / complete 生命周期。
- 原先 journal ordering 只覆盖 execution request / complete 两个事件，新增 Agent 生命周期后容易出现排序歧义，已通过 `event_orders` 收口。

## 剩余风险
- 当前 Agent 生命周期只在 `automation-service` 运行时可用，还没有外露到 HTTP 或 `sdkwork-im-server` 主链路。
- tool call 目前只有 `requested / completed / failed` 结构，没有补齐更复杂的 timeout / cancel / retry 语义。
- 设备路径仍未进入统一实时主链路，`Step 08` 仍然不完整。

## 验证证据
- `cargo test -p automation-service --test agent_response_lifecycle_test --offline --target-dir target-step08-cp082-red-agent-lifecycle`
- `cargo test -p automation-service --test agent_response_lifecycle_test --offline --target-dir target-step08-cp082-green-agent-lifecycle`
- `cargo fmt -p im-domain-core -p automation-service -- --check`
- `cargo test -p im-domain-core --offline --target-dir target-step08-cp082-im-domain-core`
- `cargo test -p automation-service --offline --target-dir target-step08-cp082-automation-service`
- `cargo test -p automation-service --test agent_response_lifecycle_test --offline --target-dir target-step08-cp082-green-agent-lifecycle-postfmt`
- `cargo test -p im-domain-core --offline --target-dir target-step08-cp082-im-domain-core-postfmt`
- `cargo test -p automation-service --offline --target-dir target-step08-cp082-automation-service-postfmt`

## 复盘结论
- `CP08-2` 的实现继续保持“小步快跑但不跳过验证”的节奏：先最小运行时闭环，再保留 HTTP / node 集成为下一轮收敛对象。
- 这一轮真正加快迭代速度的关键，不是减少测试，而是把测试压到最小行为闭环，并限制写入范围。
- 下一轮应当继续用同样方式推进 `CP08-3`：设备接入只做主路径最小闭环，不提前扩成完整 IoT 平台。

