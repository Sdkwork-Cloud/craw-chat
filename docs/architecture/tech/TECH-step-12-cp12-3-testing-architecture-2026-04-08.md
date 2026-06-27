> Migrated from `docs/review/step-12-cp12-3-兼容矩阵文档测试与控制面映射收口-架构兑现与回写决议-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 12 / CP12-3 兼容矩阵文档测试与控制面映射收口 架构兑现与回写决议- 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md`
- `docs/架构/148-CCP控制面注册表与协议发布治理设计2026-04-06.md`
- `docs/架构/149-多Cell多Region协议升级与灾备兼容设计2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave D / Step 12` 已具compatibility matrix 的三类证据：
    - 文档
    - 测试
    - control-plane 映射
- `146`
  - client compatibility matrix 不再只是 registry 内部结构，而是已显式映射到
    - `sdkwork-im-sdk`
    - `sdkwork-control-plane-sdk`
  - app-facing admin-facing SDK 现在都能指向自己`matrixClientTypes`
- `148`
  - control-plane governance 响应新增 `sdkCompatibilityBaseline`
  - governance 现在同时公开
    - raw matrix 对应client baseline
    - `effectiveSnapshot.allowedBindings`
    - `effectiveSnapshot.allowedCodecs`
    - `effectiveSnapshot.enabledCapabilities`
  - 这让 registry row runtime 生效结果之间的映射首次对 SDK facade 可见
- `149`
  - 当前已把升级兼容中的关键边界进一步冻结为：
    - raw compatibility matrix 可以声明 `ccp/mqtt/1`、`cbor`
    - governance `effectiveSnapshot` 可以rollback / kill switch 下收口`ccp/mqtt/1`、`cbor`、`payload.cbor`
  - 这为cell / region 升级中的“声明能力”和“当前允许能力”提供了真实 consumer-side 区分

## 本轮未兑现能力力力力力
- `146`
  - 多语言 SDK 的真实生成与发布
  - close / error registry 的客户端恢复语义
- `148`
  - tenant / client segment 级治
  - release bundle 与发布编
  - 更高 tier 的治理演
- `149`
  - cell / region 动rollout orchestration
  - 发布后门禁观
  - region 灾备切换自动

## 是否偏离架构
- 本轮修正的是一个证据链断裂，而不是单bug
  - 偏离现象
    - compatibility matrix 的测试与 control-plane 已存在，SDK / CLI 文档没有把它们连成统一 baseline
    - governance 响应里也没有可被 SDK facade 直接消费`sdkCompatibilityBaseline`
  - 修正结果：
    - 文档、registry test、control-plane governance response 已形成同一条映射链
- 因此当前实现回到`146 / 148 / 149` 共同定义的“兼容矩阵必须既可治理、又可消费、还能安全降级”的主路径

## 回写决议
- `docs/架构/09-实施计划.md`
  - 追加 `As-Built 108`
- `docs/架构/146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md`
  - 追加 `As-Built 6`
- `docs/架构/148-CCP控制面注册表与协议发布治理设计2026-04-06.md`
  - 追加 `As-Built 7`
- `docs/架构/149-多Cell多Region协议升级与灾备兼容设计2026-04-06.md`
  - 追加 `As-Built 8`

## 证据
- 代码
  - `services/control-plane-api/src/lib.rs`
- 测试
  - `tools/chat-cli/tests/chat_cli_contract_test.rs`
  - `crates/sdkwork-im-ccp-registry/tests/compatibility_matrix_test.rs`
  - `services/control-plane-api/tests/protocol_registry_test.rs`
  - `services/control-plane-api/tests/protocol_governance_test.rs`
- 文档
  - `docs/部署/CLI聊天验证与兼容矩阵md`
  - `sdks/README.md`
- 验证
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`
  - `cargo test -p sdkwork-im-ccp-registry --offline --test compatibility_matrix_test -- --nocapture`
  - `cargo test -p control-plane-api --offline -- --nocapture`
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture`
  - `cargo fmt --all --check`

## 当前判断
- `CP12-3`：通过
- `Step 12`：继续进行中
- 下一步：进入 `CP12-4`

