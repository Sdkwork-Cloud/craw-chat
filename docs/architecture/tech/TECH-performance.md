> Migrated from `docs/部署/性能与灾备演练场景.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 性能与灾备演练场景

## 1. 文档定位

本文件用于冻结 `Step 11 / CP11-1` 的执行基线，避免后续压测、排空、恢复和升级演练各自使用不同口径。

- 机器可读清单：`tools/perf/step-11-scenario-catalog.json`
- 场景清单 schema：`tools/perf/schemas/step-11-scenario-catalog.schema.json`
- 当前高阶门禁 schema：`tools/perf/schemas/step-11-tier-gate.schema.json`
- 当前本地量化基线：`tools/perf/step-11-cp11-2-local-baseline.json`
- 当前本地演练基线：`tools/perf/step-11-cp11-3-local-drill-baseline.json`
- 当前 IM realtime core 商用门禁基线：`tools/perf/step-11-cp11-4-im-realtime-core-baseline.json`
- 当前 IM websocket E2E 商用门禁基线：`tools/perf/step-11-cp11-5-websocket-e2e-baseline.json`
- 当前 CP11-6 WebSocket E2E 预发布补充证据包：`artifacts/perf/step-11/pre-release/im-websocket-e2e/metrics.json`
- 当前预发布门禁模板：`tools/perf/step-11-pre-release-tier-gate.json`
- 当前容量门禁模板：`tools/perf/step-11-capacity-tier-gate.json`
- 当前高阶 gate 证据模板字段：`artifactRoot`、`collectionSummary`、`evidenceSlots`
- 当前 catalog 直接暴露的高阶证据根目录：
  - `artifacts/perf/step-11/pre-release`
  - `artifacts/perf/step-11/capacity`
- 当前冻结目标：先统一场景、层级、指标和仓库资产映射，再进入 `CP11-2` 与 `CP11-3`
- 当前不做的事：不在本轮新建独立压测框架，不把单次手工跑分冒充正式基线

## 2. 执行层级

| Tier | 用途 | 当前基线 | 何时算进入下一层 |
| --- | --- | --- | --- |
| `CI Smoke Tier` | 仓库内快速回归连接、消息、流、排空、恢复语义 | `standalone.split-services.development` 与定向 `cargo test` / smoke 脚本 | 形成稳定可重复的定量命令后进入预发布层 |
| `Pre-Release Tier` | 发布前验证中等规模并发、吞吐和故障演练 | `standalone.split-services.development` 或受控预发布拓扑 | 形成至少一轮定量结果与故障复盘后进入容量层 |
| `Capacity Tier` | 专用环境验证连接密度、吞吐上限和尾延迟 | 独立容量环境或后续 cell/region 演练环境 | 输出容量报告与恢复报告 |

## 3. 场景族

| 场景族 | 目标 | 主要指标 | 当前种子资产 |
| --- | --- | --- | --- |
| `connection` | 冻结握手、连接建立、多客户端命令流程与 resume 基线 | 握手成功率、活跃连接数、resume 成功率、连接 p95 | `services/session-gateway/tests/websocket_smoke_test.rs`、`services/session-gateway/tests/cluster_routing_test.rs`、`tools/smoke/local_stack_smoke.ps1`、`tools/smoke/local_stack_smoke.sh` |
| `message` | 冻结路由消息投递与多客户端会话扇出吞吐基线 | 消息 TPS、投递成功率、扇出 p95、route transfer 成功率 | `services/session-gateway/tests/cluster_routing_test.rs`、`services/session-gateway/tests/performance_realtime_core_baseline_test.rs` |
| `stream` | 冻结流帧持久化、checkpoint 与 replay-safe 吞吐基线 | 帧吞吐、checkpoint 成功率、stream replay 成功率、帧 p95 | `services/streaming-service/tests/stream_lifecycle_test.rs` |
| `im-realtime-core` | 冻结 IM 核心实时链路 fanout、窗口裁剪、checkpoint、补偿回滚与集群 handoff 商用门禁 | fanout success permille、publish p95/p99、publish TPS、ack p95、restore duration、compensation rollback、cluster handoff、capacity trimmed events | `services/session-gateway/tests/performance_realtime_core_baseline_test.rs`、`services/session-gateway/tests/commercial_realtime_acceptance_test.rs`、`tools/perf/step-11-cp11-4-im-realtime-core-baseline.json`；执行后输出 `STEP11_REALTIME_CORE` |
| `im-websocket-e2e` | 冻结真实 TCP WebSocket 长连接、订阅、live push、断线恢复、backlog restore、checkpoint 与 route handoff 商用门禁 | connect success permille、live fanout success permille、connect p95、subscribe p95、live push p95、ack p95、disconnect recovery、backlog restore、cluster handoff、capacity trimmed events | `services/session-gateway/tests/performance_websocket_e2e_baseline_test.rs`、`services/session-gateway/tests/websocket_smoke_test.rs`、`tools/perf/step-11-cp11-5-websocket-e2e-baseline.json`；执行后输出 `STEP11_WEBSOCKET_E2E` |
| `drain-rebalance` | 冻结排空、路由迁移和 rebalance 演练基线 | 排空完成时间、route 迁移成功率、rebalance p95、route loss | `services/session-gateway/tests/cluster_routing_test.rs`、`services/session-gateway/tests/performance_ha_dr_drill_test.rs`、`services/governance-service/tests/governance_loop_test.rs` |
| `restore-recovery` | 冻结备份预演、恢复和运行目录恢复时间基线 | restore 成功率、RTO、RPO、preview diff 准确性 | `services/session-gateway/tests/performance_ha_dr_drill_test.rs`、`artifacts/perf/step-11/pre-release/restore-recovery/drill.json` |
| `failover` | 冻结跨节点 `resume takeover` 与 stale session 拒绝演练基线 | takeover 时延、owner 切换准确率、stale session 拒绝率、resume takeover 成功率 | `services/session-gateway/tests/performance_ha_dr_drill_test.rs`、`services/session-gateway/tests/cluster_routing_test.rs`、`tools/perf/step-11-cp11-3-local-drill-baseline.json` |
| `upgrade-rollback` | 冻结协议兼容、kill switch 和回滚演练基线 | compatibility 通过率、rollback 激活时间、kill switch 成功率、回滚后协议错误率 | `crates/sdkwork-im-ccp-registry/tests/compatibility_matrix_test.rs`、`services/governance-service/tests/protocol_registry_test.rs`、`services/governance-service/tests/protocol_governance_test.rs`、`services/session-gateway/tests/performance_ha_dr_drill_test.rs` |

## 4. 当前推荐执行顺序

1. 在 `CI Smoke Tier` 冻结六类场景的命令入口和结果字段。
2. 在 `Pre-Release Tier` 至少完成一轮 `connection`、`message`、`stream` 的定量结果。
3. 在同一部署基线上完成 `drain-rebalance`、`restore-recovery` 与 `failover`。
4. 最后执行 `upgrade-rollback`，避免兼容策略变动影响前两轮结果口径。
5. 如果还没有真实结果，先使用 `tools/perf/step-11-pre-release-tier-gate.json` 与 `tools/perf/step-11-capacity-tier-gate.json` 冻结所需证据字段，不把模板误写成结果。
6. 高阶 gate 的证据口径以 tier evidence index 为准，不再使用 `template_only_pending_execution` 作为当前状态描述：
   - Pre-Release：`artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json` → `evidence_collected_gate_blocked`
   - Capacity：`artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json` → `evidence_collected_gate_passed`
   - gate template（`step-11-pre-release-tier-gate.json` / `step-11-capacity-tier-gate.json`）仍用于字段冻结；真实专用拓扑补采前不得把 doc-captured 回填误写成 sign-off
7. 如果只读取 `tools/perf/step-11-scenario-catalog.json`，当前也必须能直接拿到高等 tier 的目录入口：
   - `Pre-Release Tier -> artifacts/perf/step-11/pre-release`
   - `Capacity Tier -> artifacts/perf/step-11/capacity`

## 5. 与后续检查点的关系

- `CP11-1`
  - 完成本文件与 `tools/perf/step-11-scenario-catalog.json` 的冻结
  - 明确层级、场景族、指标与仓库资产映射
- `CP11-2`
  - 基于本文件执行 `connection`、`message`、`stream` 三类至少一轮定量结果
  - 当前最小可重复执行入口为：
    - `tools/perf/step-11-cp11-2-local-baseline.json`
    - `services/session-gateway/tests/cluster_routing_test.rs`
    - `services/session-gateway/tests/performance_realtime_core_baseline_test.rs`
- `CP11-3`
  - 基于同一部署基线执行 `drain-rebalance`、`restore-recovery`、`failover`、`upgrade-rollback`
  - 当前最小可重复执行入口为：
    - `tools/perf/step-11-cp11-3-local-drill-baseline.json`
    - `services/session-gateway/tests/performance_ha_dr_drill_test.rs`
    - `node scripts/dev/run-step11-ha-dr-drill.mjs`
  - 当前 `failover` 先以跨节点 `resume takeover` 路径验证 owner 切换与 stale owner 拒绝语义
  - 当前 `upgrade-rollback` 先以 control-plane governance snapshot 与 runtime hello 协商验证 kill switch rollback，
    覆盖 MQTT binding 回滚拒绝、`payload.cbor` capability 剥离与 safe binding 零协议错误率
- `CP11-4`
  - 基于 `tools/perf/step-11-cp11-4-im-realtime-core-baseline.json` 执行 `im-realtime-core` 商用门禁
  - 当前最小可重复执行入口为：
    - `services/session-gateway/tests/performance_realtime_core_baseline_test.rs`
    - `services/session-gateway/tests/commercial_realtime_acceptance_test.rs`
  - 执行 `cargo test -p session-gateway --test performance_realtime_core_baseline_test --offline -- --nocapture` 时输出 `STEP11_REALTIME_CORE`，用于回填 metrics artifact
- `CP11-5`
  - 基于 `tools/perf/step-11-cp11-5-websocket-e2e-baseline.json` 执行 `im-websocket-e2e` 商用门禁
  - 当前最小可重复执行入口为：
    - `services/session-gateway/tests/performance_websocket_e2e_baseline_test.rs`
    - `services/session-gateway/tests/websocket_smoke_test.rs`
  - 执行 `cargo test -p session-gateway --test performance_websocket_e2e_baseline_test --offline -- --nocapture` 时输出 `STEP11_WEBSOCKET_E2E`，用于回填 metrics artifact
  - 把定量结果、异常点、改进建议和是否回写架构的结论写入 `docs/review/`
- `CP11-6`
  - 把 CP11-5 `STEP11_WEBSOCKET_E2E` 输出标准化为预发布补充证据路径：`artifacts/perf/step-11/pre-release/im-websocket-e2e/metrics.json`
  - 状态固定为 `supplement_collected_gate_blocked_pending_real_pre_release_run`
  - 覆盖真实 TCP WebSocket 长连接、订阅 fanout、断线恢复、消息窗口裁剪、跨客户端路由 checkpoint、一致性补偿和 cluster route handoff
  - 该证据包仅用于补充预发布追踪，not full Pre-Release Tier sign-off；真实预发布拓扑压测仍需替换为 CI Smoke 来源 artifact

## 6. 当前边界

- 本文件只冻结执行基线，不声称已经完成任何容量结论。
- 当前 `CP11-2` 的本地量化基线只覆盖 `CI Smoke Tier`，用于先产出一轮连接、消息、流的真实数量化结果。
- 当前 `CP11-3` 的本地演练基线只覆盖 `CI Smoke Tier`，用于先产出一轮 `drain-rebalance / restore-recovery / failover / upgrade-rollback` 的真实演练结果。
- 当前 `Pre-Release Tier` 与 `Capacity Tier` 已进入“索引已回填、专用拓扑待补采”状态：
  - Capacity Tier：`evidence_collected_gate_passed`（见 `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json`）
  - Pre-Release Tier：`evidence_collected_gate_blocked`（见 `artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json`）
  - `standalone.split-services.development` 与 `self-hosted.split-services.development` 作为当前本地/预发布 profile
  - `capacity-dedicated` 作为容量门禁模板的目标环境名；专用容量环境压测仍待执行
  - 当前 doc-captured 证据来自 CI Smoke / 本地 drill 回填，不代表 multi-region 或多 cell 自动化已落地
  - `step-11-scenario-catalog.json` 现在也直接暴露：
    - `tools/perf/step-11-pre-release-tier-gate.json`
    - `tools/perf/step-11-capacity-tier-gate.json`
    - `artifacts/perf/step-11/pre-release`
    - `artifacts/perf/step-11/capacity`
- 当前 `upgrade-rollback` 的本地 drill 只验证最小可信路径：
  - control-plane registry 与 governance snapshot 一致
  - runtime hello 协商遵从 effective snapshot
  - kill switch 能阻断高风险 binding / capability，同时保留安全降级路径
- 当前 `standalone.split-services.development` 仍沿用 `Step 10` 冻结的兼容运行合同，不额外引入新的本地拓扑。
- Git Bash 独立命令帮助在当前宿主环境可能存在兼容问题，因此 Step 11 的主要放行证据仍以仓库内测试与 PowerShell/CMD 入口为主。

