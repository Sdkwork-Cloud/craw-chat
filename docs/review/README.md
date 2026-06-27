# Review 索引

> **Topology v2 权威入口：** [docs/topology-greenfield.md](../topology-greenfield.md) · [docs/部署/README.md](../部署/README.md)
> 本目录归档执行卡与复盘；正文已迁移到 Topology v2。文件名中的历史词仅用于链接定位。

`docs/review` 用于归档执行卡、质量审计、架构兑现、回写决议与阶段/波次总验收证据。

## 执行卡命名

- 历史产物继续沿用 `step-XX-*` 命名，不回改旧文件。
- 自 `2026-04-10` 起，主步骤执行卡允许直接使用 `Sxx-执行卡-YYYY-MM-DD.md`。
- `Sxx` 编号以 `docs/step/100-*` 与 `docs/step/101-*` 为准；旧 `step-XX` 产物和新 `Sxx` 产物可并存。

## 当前持续优化样例

- [continuous-optimization-pre-release-capacity-tier-gates-2026-04-09](./continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md)
  - 对应 `Step 11`
  - 覆盖 `Pre-Release Tier` / `Capacity Tier` 模板门禁的公开索引与回写收口
  - 当前状态：`evidence_collected_gate_passed`（Capacity Tier）与 `evidence_collected_gate_blocked`（Pre-Release Tier）；权威索引见 `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json` 与 `artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json`
- [continuous-optimization-chat-cli-token-only-contract-2026-04-09](./continuous-optimization-chat-cli-token-only-contract-2026-04-09.md)
  - 对应 `Step 12`
  - 收敛 `sdkwork-im-cli token --token-only` 的裸 token 边界与 `generatedBearerToken / providedBearerToken` 来源语义
  - 对应 step 回写：`docs/step/continuous-optimization-chat-cli-token-only-contract-2026-04-09.md`
  - 对应架构回写：`docs/架构/09AS-chat-cli-token-only-contract-implementation-plan-2026-04-09.md`
  - 对应架构设计：`docs/架构/150AS-chat-cli-token-only-contract-design-2026-04-09.md`
- [continuous-optimization-chat-cli-lowercase-bearer-normalization-contract-2026-04-09](./continuous-optimization-chat-cli-lowercase-bearer-normalization-contract-2026-04-09.md)
  - 对应 `Step 12`
  - 收敛 `--bearer-token "bearer <token>"` 的大小写无关前缀归一化，冻结默认 header 形态与 `--token-only` 裸 token 形态
  - 对应 step 回写：`docs/step/continuous-optimization-chat-cli-lowercase-bearer-normalization-contract-2026-04-09.md`
  - 对应架构回写：`docs/架构/09AT-chat-cli-lowercase-bearer-normalization-contract-implementation-plan-2026-04-09.md`
  - 对应架构设计：`docs/架构/150AT-chat-cli-lowercase-bearer-normalization-contract-design-2026-04-09.md`
- [continuous-optimization-chat-cli-provided-token-claims-boundary-2026-04-09](./continuous-optimization-chat-cli-provided-token-claims-boundary-2026-04-09.md)
  - 对应 `Step 12`
  - 收敛 `providedBearerToken` 的 `claims` 边界，禁止把本地 CLI 输入伪装成外部 token 已解码 claims
  - 对应 step 回写：`docs/step/continuous-optimization-chat-cli-provided-token-claims-boundary-2026-04-09.md`
  - 对应架构回写：`docs/架构/09AU-chat-cli-provided-token-claims-boundary-implementation-plan-2026-04-09.md`
  - 对应架构设计：`docs/架构/150AU-chat-cli-provided-token-claims-boundary-design-2026-04-09.md`
- [continuous-optimization-chat-cli-cmd-help-pass-through-contract-2026-04-09](./continuous-optimization-chat-cli-cmd-help-pass-through-contract-2026-04-09.md)
- [continuous-optimization-open-chat-test-cmd-gnu-flag-contract-2026-04-09](./continuous-optimization-open-chat-test-cmd-gnu-flag-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/open-chat-test.cmd` GNU-style named flags for scripted validation on Windows
  - step: `docs/step/continuous-optimization-open-chat-test-cmd-gnu-flag-contract-2026-04-09.md`
  - impl: `docs/架构/09AW-open-chat-test-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150AW-open-chat-test-cmd-gnu-flag-contract-design-2026-04-09.md`
- [continuous-optimization-chat-window-cmd-gnu-flag-contract-2026-04-09](./continuous-optimization-chat-window-cmd-gnu-flag-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/chat-window.cmd` GNU-style named flags for interactive launches on Windows
  - step: `docs/step/continuous-optimization-chat-window-cmd-gnu-flag-contract-2026-04-09.md`
  - impl: `docs/架构/09AX-chat-window-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150AX-chat-window-cmd-gnu-flag-contract-design-2026-04-09.md`
- [continuous-optimization-open-chat-test-cmd-validation-message-special-char-contract-2026-04-09](./continuous-optimization-open-chat-test-cmd-validation-message-special-char-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/open-chat-test.cmd` literal preservation for `--validation-message` on Windows scripted validation
  - step: `docs/step/continuous-optimization-open-chat-test-cmd-validation-message-special-char-contract-2026-04-09.md`
  - impl: `docs/架构/09AY-open-chat-test-cmd-validation-message-special-char-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150AY-open-chat-test-cmd-validation-message-special-char-contract-design-2026-04-09.md`
- [continuous-optimization-chat-window-cmd-message-prefix-special-char-contract-2026-04-09](./continuous-optimization-chat-window-cmd-message-prefix-special-char-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/chat-window.cmd` literal preservation for `--message-prefix` on Windows interactive launches
  - step: `docs/step/continuous-optimization-chat-window-cmd-message-prefix-special-char-contract-2026-04-09.md`
  - impl: `docs/架构/09AZ-chat-window-cmd-message-prefix-special-char-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150AZ-chat-window-cmd-message-prefix-special-char-contract-design-2026-04-09.md`
- [continuous-optimization-chat-window-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-chat-window-cmd-help-gnu-surface-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/chat-window.cmd --help` GNU-style discoverability on Windows
  - step: `docs/step/continuous-optimization-chat-window-cmd-help-gnu-surface-contract-2026-04-09.md`
  - impl: `docs/架构/09BA-chat-window-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BA-chat-window-cmd-help-gnu-surface-contract-design-2026-04-09.md`
- [continuous-optimization-open-chat-test-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-open-chat-test-cmd-help-gnu-surface-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/open-chat-test.cmd --help` GNU-style discoverability on Windows
  - step: `docs/step/continuous-optimization-open-chat-test-cmd-help-gnu-surface-contract-2026-04-09.md`
  - impl: `docs/架构/09BB-open-chat-test-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BB-open-chat-test-cmd-help-gnu-surface-contract-design-2026-04-09.md`
- [continuous-optimization-chat-window-gui-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-chat-window-gui-cmd-help-gnu-surface-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/chat-window-gui.cmd --help` GNU-style discoverability on Windows
  - step: `docs/step/continuous-optimization-chat-window-gui-cmd-help-gnu-surface-contract-2026-04-09.md`
  - impl: `docs/架构/09BC-chat-window-gui-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BC-chat-window-gui-cmd-help-gnu-surface-contract-design-2026-04-09.md`
- [continuous-optimization-chat-window-gui-cmd-label-special-char-contract-2026-04-09](./continuous-optimization-chat-window-gui-cmd-label-special-char-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/chat-window-gui.cmd` literal preservation for `-Label` / `--label` on Windows
  - step: `docs/step/continuous-optimization-chat-window-gui-cmd-label-special-char-contract-2026-04-09.md`
  - impl: `docs/架构/09BD-chat-window-gui-cmd-label-special-char-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BD-chat-window-gui-cmd-label-special-char-contract-design-2026-04-09.md`
- [continuous-optimization-chat-window-gui-cmd-gnu-flag-contract-2026-04-09](./continuous-optimization-chat-window-gui-cmd-gnu-flag-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/chat-window-gui.cmd` GNU-style named flags for visible GUI launches on Windows
  - step: `docs/step/continuous-optimization-chat-window-gui-cmd-gnu-flag-contract-2026-04-09.md`
  - impl: `docs/架构/09BE-chat-window-gui-cmd-gnu-flag-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BE-chat-window-gui-cmd-gnu-flag-contract-design-2026-04-09.md`
  - 对应 `Step 12`
  - 收敛 `bin/chat-cli.cmd --help` 的 raw-arg pass-through 边界，禁止 wrapper 把 `--help` 改写成 `-Help`
  - 对应 step 回写：`docs/step/continuous-optimization-chat-cli-cmd-help-pass-through-contract-2026-04-09.md`
  - 对应架构回写：`docs/架构/09AV-chat-cli-cmd-help-pass-through-contract-implementation-plan-2026-04-09.md`
  - 对应架构设计：`docs/架构/150AV-chat-cli-cmd-help-pass-through-contract-design-2026-04-09.md`
  - 默认预发布 profile：`standalone.split-services.development`
  - 目标容量环境：`capacity-dedicated`
  - 对应部署文档：`docs/部署/性能与灾备演练场景.md`
  - 对应 gate 模板：`tools/perf/step-11-pre-release-tier-gate.json`、`tools/perf/step-11-capacity-tier-gate.json`
  - 对应 catalog：`tools/perf/step-11-scenario-catalog.json`；未来高阶 `artifactRoot` 归档根目录：`artifacts/perf/step-11/pre-release`、`artifacts/perf/step-11/capacity`
  - 对应 schema：`tools/perf/schemas/step-11-scenario-catalog.schema.json`、`tools/perf/schemas/step-11-tier-gate.schema.json`
  - 同时冻结 `collectionSummary`、`evidenceSlots`、`pending_collection`、`checksumSha256` 等 evidence-slot 契约字段，当前仍待真实采集
  - `collectionSummary` 公开 `totalSlots`、`requiredSlots`、`optionalSlots`、`collectedSlots`、`pendingSlots`、`skippedOptionalSlots` 六个统计字段
  - Capacity Tier 当前索引状态：`evidence_collected_gate_passed`（7/7 collected，见 `artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json`）
  - Pre-Release Tier 当前索引状态：`evidence_collected_gate_blocked`（7/7 collected，见 `artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json`）
  - evidence slot 元数据还包含 `artifactPath`、`suggestedRelativePath`、`collectedAt`、`sizeBytes` 等回填字段
  - evidence slot 语义字段还包含 `scenarioFamily`、`required`、`reportId`，用于区分场景槽位与报告槽位
  - 最小示例值统一冻结为 `scenarioFamily = connection` / `scenarioFamily = failover`、`required = true`、`reportId = capacity_report` / `reportId = recovery_report`
  - `reportId` 与 `artifactKind` 的对应关系包括 `capacity_report -> report_markdown`、`recovery_report -> report_markdown`
  - `reportId` 与建议路径的对应关系包括 `capacity_report -> reports/capacity-report.md`、`recovery_report -> reports/recovery-report.md`
  - `reportId` 与 `requiredSections` 的对应关系包括 `capacity_report -> input_scale / throughput_summary / tail_latency_summary`、`recovery_report -> recovery_window / rto_rpo_summary / operator_follow_up`
  - `suggestedRelativePath` 示例包括 `connection/metrics.json`、`failover/drill.json`、`reports/capacity-report.md`、`reports/recovery-report.md`
  - Capacity Tier 额外示例路径包括 `connection/capacity.json`、`restore-recovery/recovery.json`、`failover/recovery.json`
  - Capacity Tier 剩余 capacity 路径示例包括 `message/capacity.json`、`stream/capacity.json`
  - evidence slot 还冻结主键 `id`，代表值包括 `connection_metrics`、`connection_capacity`、`failover_recovery`
  - Capacity Tier 其余代表性 slot id 还包括 `message_capacity`、`stream_capacity`、`restore_recovery_recovery`
  - Pre-Release Tier collected slot examples now include `message_metrics` and `stream_metrics`
  - Pre-Release Tier collected path examples now include `message/metrics.json` and `stream/metrics.json`
  - Pre-Release Tier current state is now `evidence_collected_gate_blocked`
  - Capacity Tier current state is now `evidence_collected_gate_passed`
  - Both Step 11 tier artifact roots now carry truthful local evidence; dedicated topology runs still gate full commercial sign-off.
  - evidence slot 还公开 `artifactKind`，代表值包括 `metrics_json`、`drill_json`、`capacity_json`、`recovery_json`、`report_markdown`
  - 机器契约还冻结 `requiredFields` / `requiredSections`，示例值包括 `runId`、`connectP95Ms`、`input_scale`、`operator_follow_up`
  - 额外字段示例包括 `messageTps`、`frameP95Ms`、`recovery_window`、`rto_rpo_summary`
  - report section 代表值还包括 `throughput_summary`、`tail_latency_summary`、`recovery_window`、`operator_follow_up`
  - 更细一级字段示例还包括 `fanoutP95Ms`、`streamFramesPerSecond`、`previewDiffAccuracy`、`rollbackActivationSeconds`
  - drill / rollback 字段示例还包括 `drainCompletionSeconds`、`restoreRtoSeconds`、`compatibilityMatrixPassRate`、`postRollbackProtocolErrorRate`
  - `artifactKind` 与代表字段/section 的对应关系包括 `metrics_json -> connectP95Ms / messageTps / frameP95Ms`、`drill_json -> drainCompletionSeconds / rollbackActivationSeconds`、`capacity_json -> fanoutP95Ms / streamFramesPerSecond`、`recovery_json -> restoreRtoSeconds / previewDiffAccuracy`、`report_markdown -> throughput_summary / rto_rpo_summary`
  - `artifactKind` 与建议路径的对应关系包括 `metrics_json -> connection/metrics.json / message/metrics.json`、`drill_json -> failover/drill.json / restore-recovery/drill.json`、`capacity_json -> connection/capacity.json / message/capacity.json`、`recovery_json -> failover/recovery.json / restore-recovery/recovery.json`、`report_markdown -> reports/capacity-report.md / reports/recovery-report.md`
  - `artifactKind` 与代表性 `slot id` 的对应关系包括 `metrics_json -> connection_metrics / message_metrics`、`drill_json -> failover_drill / restore_recovery_drill`、`capacity_json -> connection_capacity / message_capacity`、`recovery_json -> failover_recovery / restore_recovery_recovery`、`report_markdown -> capacity_report / recovery_report`
  - `artifactKind` 与 `requiredFields / requiredSections` 的对应关系包括 `metrics_json -> runId / connectionCount / successCount`、`drill_json -> runId / drainCompletionSeconds / takeoverDurationMs`、`capacity_json -> runId / peakActiveConnections / messageTps`、`recovery_json -> runId / restoreRtoSeconds / staleSessionRejectionRate`、`report_markdown -> input_scale / throughput_summary / operator_follow_up`
  - `requiredScenarioFamilies = connection / message / stream / drain-rebalance / restore-recovery / failover / upgrade-rollback`
  - `requiredScenarioFamilies = connection / message / stream / restore-recovery / failover`
  - `requiredReports = capacity_report / recovery_report`
  - `requiredOutputs` 以 `scenarioFamily -> artifactKind -> requiredFields` tuple 冻结最小输出契约，代表项包括 `connection -> metrics_json -> runId / connectionCount / successCount`、`restore-recovery -> recovery_json -> runId / restoreRtoSeconds / dataLossRpoEvents / previewDiffAccuracy`
  - `operatorDocPath = docs/部署/性能与灾备演练场景.md`，`scenarioCatalogPath = tools/perf/step-11-scenario-catalog.json`
  - `profile = standalone.split-services.development / capacity-dedicated`
  - `reviewBackwrite = docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md / docs/review/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md / docs/架构/09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md / docs/架构/150AR-pre-release-capacity-tier-gates-design-2026-04-09.md`
  - `scenarioFamily` 与 `artifactKind` 的对应关系包括 `connection -> metrics_json / capacity_json`、`failover -> drill_json / recovery_json`、`restore-recovery -> drill_json / recovery_json`
  - `scenarioFamily` 与 `requiredFields / requiredSections` 的对应关系包括 `connection -> runId / connectP95Ms`、`failover -> runId / takeoverDurationMs`、`restore-recovery -> runId / restoreRtoSeconds / previewDiffAccuracy`
  - `scenarioFamily` 与 slot id 的对应关系包括 `connection -> connection_metrics / connection_capacity`、`failover -> failover_drill / failover_recovery`、`restore-recovery -> restore_recovery_drill / restore_recovery_recovery`
  - 代表性 `slot id` 与 `artifactKind` 的对应关系包括 `connection_metrics -> metrics_json`、`failover_drill -> drill_json`、`restore_recovery_recovery -> recovery_json`
  - 代表性 `slot id` 与 `requiredFields / requiredSections` 的对应关系包括 `connection_metrics -> runId / connectP95Ms`、`failover_drill -> runId / takeoverDurationMs`、`capacity_report -> input_scale / throughput_summary / tail_latency_summary`
  - `scenarioFamily` 与建议路径的对应关系包括 `connection -> connection/metrics.json / connection/capacity.json`、`failover -> failover/drill.json / failover/recovery.json`、`restore-recovery -> restore-recovery/drill.json / restore-recovery/recovery.json`
  - 代表性 `slot id` 与建议路径的对应关系包括 `connection_metrics -> connection/metrics.json`、`failover_drill -> failover/drill.json`、`restore_recovery_recovery -> restore-recovery/recovery.json`
  - 默认命名关系为 `artifactPath = artifactRoot + "/" + suggestedRelativePath`
  - 在真实采集前，`artifactPath`、`collectedAt`、`sizeBytes`、`checksumSha256` 可保持 `null`；当前索引已回填 doc-captured 证据，但 dedicated topology 运行仍待补采
  - 对应 step 回写：`docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md`
  - 对应架构回写：`docs/架构/09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md`
  - 对应架构设计：`docs/架构/150AR-pre-release-capacity-tier-gates-design-2026-04-09.md`

## 当前总体验收与 backlog 入口

- [step-13-next-wave-backlog-2026-04-08](./step-13-next-wave-backlog-2026-04-08.md)
- [wave-c-93-持续优化复核-2026-04-09](./wave-c-93-持续优化复核-2026-04-09.md)
- [wave-d-93-总验收-2026-04-08](./wave-d-93-总验收-2026-04-08.md)
# 2026-04-09 Step 12 Addendum

- [continuous-optimization-retired-lifecycle-start-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-retired-lifecycle-start-cmd-help-gnu-surface-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/retired-lifecycle-start.cmd --help` GNU-style discoverability on Windows
  - step: `docs/step/continuous-optimization-retired-lifecycle-start-cmd-help-gnu-surface-contract-2026-04-09.md`
  - impl: `docs/架构/09BF-retired-lifecycle-start-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BF-retired-lifecycle-start-cmd-help-gnu-surface-contract-design-2026-04-09.md`
- [continuous-optimization-retired-lifecycle-status-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-retired-lifecycle-status-cmd-help-gnu-surface-contract-2026-04-09.md)
  - `Step 12`
  - freeze `bin/retired-lifecycle-status.cmd --help` GNU-style discoverability on Windows
  - step: `docs/step/continuous-optimization-retired-lifecycle-status-cmd-help-gnu-surface-contract-2026-04-09.md`
  - impl: `docs/架构/09BG-retired-lifecycle-status-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BG-retired-lifecycle-status-cmd-help-gnu-surface-contract-design-2026-04-09.md`
- [continuous-optimization-principal-profile-runtime-provider-selection-2026-04-09](./continuous-optimization-principal-profile-runtime-provider-selection-2026-04-09.md)
  - `Step 12`
  - fix the runtime-selection drift where default app bootstrap could not choose `principal-profile-external-catalog`
  - step: `docs/step/continuous-optimization-principal-profile-runtime-provider-selection-2026-04-09.md`
  - impl: `docs/架构/09BH-principal-profile-runtime-provider-selection-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BH-principal-profile-runtime-provider-selection-design-2026-04-09.md`
- [continuous-optimization-install-deploy-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-install-deploy-cmd-help-gnu-surface-contract-2026-04-09.md)
  - `Step 12`
  - close the remaining Windows help-surface drift for `retired-lifecycle-install.cmd` and `retired-lifecycle-deploy.cmd`
  - step: `docs/step/continuous-optimization-install-deploy-cmd-help-gnu-surface-contract-2026-04-09.md`
  - impl: `docs/架构/09BI-install-deploy-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BI-install-deploy-cmd-help-gnu-surface-contract-design-2026-04-09.md`
- [continuous-optimization-principal-profile-external-catalog-missing-catalog-unavailable-contract-2026-04-09](./continuous-optimization-principal-profile-external-catalog-missing-catalog-unavailable-contract-2026-04-09.md)
  - `Step 12`
  - close the missing external principal-profile catalog-path drift where app assembly still panicked instead of returning structured unavailable semantics
  - step: `docs/step/continuous-optimization-principal-profile-external-catalog-missing-catalog-unavailable-contract-2026-04-09.md`
  - impl: `docs/架构/09BJ-principal-profile-external-catalog-missing-catalog-unavailable-contract-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BJ-principal-profile-external-catalog-missing-catalog-unavailable-contract-design-2026-04-09.md`
## 2026-04-09 Addendum

- [continuous-optimization-principal-profile-provider-health-http-surface-2026-04-09](./continuous-optimization-principal-profile-provider-health-http-surface-2026-04-09.md)
  - `Step 12`
  - close the missing operator-facing provider-health surface for `principal-profile`
  - step: `docs/step/continuous-optimization-principal-profile-provider-health-http-surface-2026-04-09.md`
  - impl: `docs/架构/09BK-principal-profile-provider-health-http-surface-implementation-plan-2026-04-09.md`
  - design: `docs/架构/150BK-principal-profile-provider-health-http-surface-design-2026-04-09.md`
## 2026-04-09 Addendum

- [continuous-optimization-local-minimal-ops-provider-bindings-runtime-visibility-2026-04-09](./continuous-optimization-local-minimal-ops-provider-bindings-runtime-visibility-2026-04-09.md)
- [continuous-optimization-local-minimal-ops-provider-bindings-http-surface-2026-04-09](./continuous-optimization-local-minimal-ops-provider-bindings-http-surface-2026-04-09.md)
## 2026-04-09 Addendum

- [continuous-optimization-step11-tier-gate-doc-state-alignment-2026-04-09](./continuous-optimization-step11-tier-gate-doc-state-alignment-2026-04-09.md)
## 2026-04-09 Addendum

- [continuous-optimization-step11-tier-artifact-root-materialization-2026-04-09](./continuous-optimization-step11-tier-artifact-root-materialization-2026-04-09.md)
## 2026-04-09 Addendum

- [continuous-optimization-step11-tier-machine-readable-evidence-index-2026-04-09](./continuous-optimization-step11-tier-machine-readable-evidence-index-2026-04-09.md)
## 2026-04-09 Addendum

- [continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09.md)
  - close the next real Step 11 gap by materializing one collected `Pre-Release Tier` failover artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_failover_collected_evidence`
- [continuous-optimization-step11-pre-release-restore-recovery-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-restore-recovery-collected-evidence-2026-04-09.md)
  - close the next real Step 11 gap by materializing one collected `Pre-Release Tier` restore-recovery artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_restore_recovery_collected_evidence`
- [continuous-optimization-step11-pre-release-drain-rebalance-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-drain-rebalance-collected-evidence-2026-04-09.md)
  - close the next real Step 11 gap by materializing one collected `Pre-Release Tier` drain-rebalance artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_drain_rebalance_collected_evidence`
- [continuous-optimization-step11-pre-release-upgrade-rollback-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-upgrade-rollback-collected-evidence-2026-04-09.md)
  - close the next real Step 11 gap by materializing one collected `Pre-Release Tier` upgrade-rollback artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_upgrade_rollback_collected_evidence`
- [continuous-optimization-step11-pre-release-connection-metrics-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-connection-metrics-collected-evidence-2026-04-09.md)
  - close the next real Step 11 gap by materializing one collected `Pre-Release Tier` connection metric artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_connection_metrics_collected_evidence`
- [continuous-optimization-step11-pre-release-message-metrics-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-message-metrics-collected-evidence-2026-04-09.md)
  - close the next real Step 11 gap by materializing one collected `Pre-Release Tier` message metric artifact
  - verification anchored in `test_continuous_optimization_materializes_pre_release_message_metrics_collected_evidence`
- [continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09](./continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09.md)
  - close the final truthful Step 11 metric gap by materializing one collected `Pre-Release Tier` stream artifact and moving the tier into `evidence_collected_gate_blocked`
  - verification anchored in `test_continuous_optimization_materializes_pre_release_stream_metrics_collected_evidence`
- [continuous-optimization-step11-closure-claim-supersession-2026-04-09](./continuous-optimization-step11-closure-claim-supersession-2026-04-09.md)
  - correct stale Step 11 “fully closed” readings that conflict with the current high-tier evidence state
  - verification anchored in `test_continuous_optimization_supersedes_stale_step11_closure_claims_in_historical_docs`
## 2026-04-09 Addendum

- [continuous-optimization-shell-process-identity-portability-2026-04-09](./continuous-optimization-shell-process-identity-portability-2026-04-09.md)
  - `Step 10`
  - close the Bash lifecycle drift where `ps -o comm=` can truncate `sdkwork-im-server` on BSD/macOS
  - verification anchored in `test_shell_lifecycle_scripts_use_args_based_process_identity_for_portability`
- [continuous-optimization-runtime-lifecycle-profile-selection-2026-04-09](./continuous-optimization-runtime-lifecycle-profile-selection-2026-04-09.md)
  - `Step 10`
  - close the lifecycle/runtime-ops split where `standalone.split-services.development` worked in status/runtime scripts but not in init/install/start/stop/restart
  - verification anchored in `test_init_config_local_ps1_uses_local_default_profile_when_requested` and `test_restart_local_ps1_forwards_profile_name_to_stop_and_start_scripts`
- [continuous-optimization-lifecycle-profile-doc-contract-alignment-2026-04-09](./continuous-optimization-lifecycle-profile-doc-contract-alignment-2026-04-09.md)
  - `Step 10`
  - close the operator-doc drift where lifecycle profile support existed in scripts but not in README and quick-start examples
  - verification anchored in `test_quick_start_doc_surfaces_local_default_profile_examples_across_lifecycle_commands`
- [continuous-optimization-retired-lifecycle-start-ps1-health-timeout-test-stability-2026-04-09](./continuous-optimization-retired-lifecycle-start-ps1-health-timeout-test-stability-2026-04-09.md)
  - `Step 10`
  - close the flaky health-timeout rollback regression caused by an over-compressed test scheduling window
  - verification anchored in `test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out`
## 2026-04-09 Addendum

- [continuous-optimization-restore-runtime-cmd-expected-preview-fingerprint-2026-04-09](./continuous-optimization-restore-runtime-cmd-expected-preview-fingerprint-2026-04-09.md)
  - `Step 10`
  - close the Windows restore-wrapper drift where `.cmd` dropped the documented preview fingerprint confirmation flag
  - verification anchored in `test_restore_runtime_local_cmd_normalizes_expected_preview_fingerprint_switch`
## 2026-04-09 Addendum

- [continuous-optimization-inspect-runtime-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-inspect-runtime-cmd-help-gnu-surface-contract-2026-04-09.md)
  - `Step 10`
  - close the Windows inspect-runtime help drift where `.cmd --help` only surfaced PowerShell usage
  - verification anchored in `test_inspect_runtime_local_cmd_help_surfaces_gnu_style_named_flags`
## 2026-04-09 Addendum

- [continuous-optimization-retired-lifecycle-start-ps1-health-timeout-window-recalibration-2026-04-09](./continuous-optimization-retired-lifecycle-start-ps1-health-timeout-window-recalibration-2026-04-09.md)
  - `Step 10`
  - re-stabilize the Windows health-timeout rollback test after the prior `5 x 100ms` window remained too small
  - verification anchored in `test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out`
## 2026-04-09 Addendum

- [continuous-optimization-repair-runtime-cmd-help-gnu-surface-contract-2026-04-09](./continuous-optimization-repair-runtime-cmd-help-gnu-surface-contract-2026-04-09.md)
  - `Step 10`
  - close the Windows repair-runtime help drift where `.cmd --help` only surfaced PowerShell usage
  - verification anchored in `test_repair_runtime_local_cmd_help_surfaces_gnu_style_named_flags`
## 2026-04-09 Addendum

- [continuous-optimization-open-chat-test-detached-gui-start-process-fallback-2026-04-09](./continuous-optimization-open-chat-test-detached-gui-start-process-fallback-2026-04-09.md)
  - `Step 10`
  - close the Windows popup-launch gap where `open-chat-test.ps1` default GUI mode had no stable middle fallback between `Win32_Process.Create` and VBS
  - verification anchored in `test_open_chat_test_ps1_uses_detached_gui_launcher_for_default_windows_mode`
