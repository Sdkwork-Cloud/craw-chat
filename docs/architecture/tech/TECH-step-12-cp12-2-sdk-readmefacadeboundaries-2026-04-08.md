> Migrated from `docs/review/step-12-cp12-2-sdk目录README与facade边界收口-执行卡-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 12 / CP12-2 SDK 目录 README facade 边界收口 执行- 2026-04-08

## 当前上下
- 当前波次：`Wave D`
- 当前 step：`Step 12`
- 当前子任务：`CP12-2`
- 前置状态：
  - `CP12-1` 已闭环，CLI / realtime / public app 主链路已具备可信验证基线
  - `sdks/` 目录已经存在，但此前缺少能冻结边界的 README workspace 入口说明
  - `Step 12` 仍未闭环，因SDK facade、compatibility matrix、终端验证脚本仍未收口

## 本轮为什么做这个增量
- `docs/step/12-SDK-CLI与兼容矩阵收口md` 明确要求两套 SDK 目录具备清晰 facade 边界：README，不能保留为“空目录 + 未来再说”的幽灵路径
- 如果 `sdks/` 没有稳定入口与职责声明，后续 `CP12-3` compatibility matrix control-plane 映射就会缺少明确消费者边界：
- 因此本轮先冻结：
  - workspace README `sdks/README.md` 的入
  - 应用/ 管理SDK facade 边界
  - `TypeScript` / `Flutter` 子目录的稳定占位路径

## 本轮实际完成

### 1. 已先用契约测试把 README 边界冻结下来
- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增测试
  - `test_step12_sdk_readmes_freeze_facade_boundaries_and_workspace_entry_points`
- 该测试当前冻结了以下事实。
  - `sdks/README.md` 必须公开两套 SDK 入口、`compatibility matrix`、`TypeScript`、`Flutter`
  - `sdks/sdkwork-im-sdk/README.md` 必须声明 `payload.json`、`ccp/ws/1`、`bearer` app-facing 边界
  - `sdks/sdkwork-control-plane-sdk/README.md` 必须声明 `control-plane`、`protocol governance` admin-facing 边界
  - `README.md` 必须链接 `sdks/README.md`

### 2. 已把 workspace 根入口补齐SDK 总览
- 更新：`README.md`
- 当前仓库README 已显式公开
  - `sdks/README.md`
- 这消除了“SDK 目录存在但没有仓库级入口”的灰色状态

### 3. 已冻`sdks/` 总览与两facade 容器边界
- 更新
  - `sdks/README.md`
  - `sdks/sdkwork-im-sdk/README.md`
  - `sdks/sdkwork-control-plane-sdk/README.md`
- 当前已明确：
  - `sdkwork-im-sdk`
    - 只承载app-facing conversation / message / timeline / realtime facade
    - 遵守 `payload.json`、`ccp/ws/1`、bearer `auth_bind`
    - capability 是否启用`compatibility matrix` 与协商结果决议
  - `sdkwork-control-plane-sdk`
    - 只承载`control-plane` / `protocol governance` / `compatibility matrix`
    - 不冒充聊天产SDK
    - 协议能力决策control-plane snapshot 为准

### 4. 已把 `TypeScript` / `Flutter` 子目录从空壳路径升级为稳定占位入
- 新增长
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
  - `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/README.md`
  - `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/README.md`
- 当前这些目录不再只是空文件夹，而是有了最小可信职责声明：
  - 谁消app-facing facade
  - 谁消admin-facing facade
  - 当前不提前宣称代码生成或发布链已经完

## TDD / Red-Green 证据

### Red
- 延续上一轮已完成的失败验证：
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_step12_sdk_readmes_freeze_facade_boundaries_and_workspace_entry_points -- --nocapture`
  - 初始失败原因：SDK README 为空或缺Step 12 需要冻结的边界文本

### Green
- README 与占位入口补齐后，同一条契约测试已保持通过
- 本轮没有跳过 README 约束，也没有把边界说明留到后续再补，而是workspace 入口、facade 角色与语言路径一次性写成了可回归资产

## Fresh 验证
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture`
- `cargo fmt --all --check`

## 当前判断
- `CP12-2`：闭环
- 已兑现：
  - `sdks/` 目录不再是无边界的占位空
  - app-facing / admin-facing facade 边界已冻
  - `TypeScript` / `Flutter` 目录已具备稳定入
  - README SDK 总览workspace 入口已建
- 当前仍未兑现但不阻塞 `CP12-2` 关闭环
  - `CP12-3` compatibility matrix 文档 / 测试 / 控制面映射整
  - `CP12-4` 的多终端聊天与流式验证脚本收口
  - 多语言 SDK 的真实生成、发布与示例链路

## 下一轮继续做什
1. 进入 `Wave D / Step 12 / CP12-3`
2. 盘点 `compatibility matrix` 当前文档、测试、control-plane snapshot 的真实差
3. 继续保持 `CP12-2` 已冻结的 SDK facade 边界不漂

