> Migrated from `docs/review/step-12-cp12-3-兼容矩阵文档测试与控制面映射收口-执行卡-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 12 / CP12-3 兼容矩阵文档测试与控制面映射收口 执行- 2026-04-08

## 当前上下
- 当前波次：`Wave D`
- 当前 step：`Step 12`
- 当前子任务：`CP12-3`
- 前置状态：
  - `CP12-1` 已闭环，CLI / realtime / public app 主链路可
  - `CP12-2` 已闭环，SDK workspace 入口facade 边界可信
  - `compatibility matrix` 仍缺少“文档、测试、control-plane 映射”三者统一的收口门

## 本轮为什么做这个增量
- `docs/step/12-SDK-CLI与兼容矩阵收口md` `CP12-3` 的要求是
  - 有文
  - 有测
  - 有控制面映射
- 当前仓库虽然已经有：
  - `crates/sdkwork-im-ccp-registry/tests/compatibility_matrix_test.rs`
  - `services/control-plane-api/tests/protocol_registry_test.rs`
  - `services/control-plane-api/tests/protocol_governance_test.rs`
  但这些证据是分散的，缺少一条能SDK facade、client row、registry governance 对齐到同一口径的闭环
- 因此本轮先补最小可信控制面映射，再把文档与契约测试追平

## 本轮实际完成

### 1. 已先用红灯把真实缺口钉死
- 更新测试
  - `tools/chat-cli/tests/chat_cli_contract_test.rs`
    - `test_step12_compatibility_matrix_doc_freezes_control_plane_mapping_and_client_rows`
  - `services/control-plane-api/tests/protocol_governance_test.rs`
    - `test_control_plane_exposes_protocol_governance_snapshot_to_control_readers`
- 初始失败分别证明
  - Step 12 文档里缺`desktop / mobile / backend` client row control-plane 映射
  - control-plane governance 响应缺少 `sdkCompatibilityBaseline`

### 2. 已把 control-plane governance 补成 SDK 可消费的 matrix baseline
- 更新：`services/control-plane-api/src/lib.rs`
- 当前新增长
  - `sdkCompatibilityBaseline`
    - `appSdkFacade = sdkwork-im-sdk`
    - `adminSdkFacade = sdkwork-control-plane-sdk`
    - `matrixClientTypes = ["backend", "desktop", "mobile", "web"]`
    - `protocolRegistryPath = /backend/v3/api/control/protocol-registry`
    - `protocolGovernancePath = /backend/v3/api/control/protocol-governance`
- 这让 control-plane 不再只返runtime 用的 `effectiveSnapshot`，也开始返回面Step 12 SDK facade matrix baseline

### 3. 已把 Step 12 兼容矩阵文档升级为显client row control-plane field mapping
- 更新：`docs/部署/CLI聊天验证与兼容矩阵md`
- 当前文档已明确：
  - `web / desktop / mobile / backend` 四类 client row
  - `ccp/http/1` / `ccp/ws/1` / `ccp/sse/1` / `ccp/mqtt/1`
  - `json` / `cbor`
  - `payload.json` / `payload.cbor`
  - raw registry row governance `effectiveSnapshot` 的差
  - `sdkCompatibilityBaseline` 如何把两SDK facade 对齐matrix client type

### 4. 已把 SDK 总览的当前状态追平到 CP12-3
- 更新：`sdks/README.md`
- 当前 SDK 总览不再停留在“后CP12-3 再做”的旧状态，而是明确当前 matrix 闭环证据来自
  - 文档
  - registry test
  - protocol registry test
  - protocol governance test

## TDD / Red-Green 证据

### Red
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_step12_compatibility_matrix_doc_freezes_control_plane_mapping_and_client_rows -- --nocapture`
  - 初始失败：`Step 12 compatibility doc must contain desktop`
- `cargo test -p control-plane-api --offline --test protocol_governance_test test_control_plane_exposes_protocol_governance_snapshot_to_control_readers -- --nocapture`
  - 初始失败：`protocol governance should expose sdk compatibility baseline`

### Green
- 在补齐文档与 control-plane response 后，上述两条测试均已保持通过
- 本轮没有compatibility matrix 继续停留在分散注释里，而是client row、control-plane endpoint、SDK facade baseline 一起补成了可回归资产

## Fresh 验证
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`
- `cargo test -p sdkwork-im-ccp-registry --offline --test compatibility_matrix_test -- --nocapture`
- `cargo test -p control-plane-api --offline -- --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture`
- `cargo fmt --all --check`

## 当前判断
- `CP12-3`：闭环
- 已兑现：
  - compatibility matrix 已同时具备文档、测试、control-plane 映射
  - SDK facade 已能在控制面响应里找到自己的 matrix baseline
  - raw registry row governance `effectiveSnapshot` 的边界已被写
- 当前仍未兑现但不阻塞 `CP12-3` 关闭环
  - `CP12-4` 的多终端聊天与流式验证脚本整步收口
  - tenant / client segment 级治
  - 多语言 SDK 真实生成与发布链

## 下一轮继续做什
1. 进入 `Wave D / Step 12 / CP12-4`
2. 盘点 `bin/open-chat-test*`、`bin/chat-window*` 与终端流式验证脚本的重复执行缺口
3. 保持 `CP12-1 / CP12-2 / CP12-3` 已冻结的主链路、SDK 边界：matrix 映射不漂

