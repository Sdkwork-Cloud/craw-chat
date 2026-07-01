# Release 文档规范

## 1. 目录定位

`docs/release` 用于沉淀版本、changelog、release notes、go/no-go、商业化交付记录。

## 2. 最小文件集

每次有效迭代至少维护：

1. `docs/release/CHANGELOG.md`
2. `docs/release/YYYY-MM-DD-vX.Y.Z-loop-XX.md`

必要时补：

- `docs/release/YYYY-MM-DD-vX.Y.Z-release-notes.md`
- `docs/release/YYYY-MM-DD-vX.Y.Z-go-no-go.md`

## 3. 版本规则

- 商业化正式发布前默认 `0.y.z`
- `minor`：新增能力、完成 step/wave 闭环、外部可感知增强
- `patch`：修复、对齐、文档/测试/脚本补强、非破坏性优化
- `major`：破坏性变更或正式商业版本发布
- 仅当 `S00-S14` 全部闭环且 `release_closure` 达成，才允许 `1.0.0`

## 4. 每条 changelog 必填

- 日期
- 版本
- loop 编号
- 影响 step
- 变更摘要
- 专业影响描述
- 数据模型 / API / 协议 / 行为变化
- 迁移 / 降级 / 回退
- 测试与证据
- 文档回写
- 剩余风险

## 5. 更新规则

只要代码、行为、契约、测试结论、step 闭环状态、架构回写结论发生变化，就必须更新 `CHANGELOG.md` 与本轮 loop 文档。

## 6. 商业化交付统一验证入口

- 进入 release / go-no-go 评估前，建议先单独执行 `pnpm test:appbase-iam-standard`
- 该命令会先执行共享 `sdkwork-appbase` 用户中心标准合同，再执行 `sdkwork-im-portal` 的用户中心桥接合同，用于冻结 AuthToken / AccessToken / secret-handshake 与独立 validation plugin 的统一标准
- 进入 release / go-no-go 评估前，必须先执行 `node scripts/release/commercial-readiness.mjs`
- 该命令按固定顺序运行 PC install/lint/build、Playwright production shell + authenticated chat e2e（默认端口 `4173`）、SDK 契约、Step 11 HA/DR drill、commercial deployment contract、Rust gateway/session gates，以及 Step 11 tier evidence 评估
- 共享 SDK 依赖默认通过 sibling git materialization 提供；见 `sdks/README.md` 的 Shared SDK Materialization Modes
- 若任一验证命令执行失败、依赖缺失、或 Step 11 tier evidence index 无法读取 / 解析，则命令必须以 `exit code 1` 失败退出，并视为验证链路异常
- 若代码验证全部通过，但 `artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json` 或 `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json` 未完成真实采集，则命令必须以 `exit code 2` 阻断发布结论
- Pre-Release Tier 与 Capacity Tier 均须达到 `evidence_collected_gate_passed`；doc-captured 回填指标须在 index `boundary` 字段中如实声明
- 只有当统一验证命令返回 `0`，且两级 evidence index 均通过上述诚实状态检查，才允许在 release / commercial readiness 文档中声明“可商用交付”
