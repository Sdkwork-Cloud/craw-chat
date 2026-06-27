# Step 13 Next-Wave Backlog - 2026-04-08

## 用途
- 本清单作为 `00-13` 全部闭环后的持续优化输入。
- 本清单中的条目当前都不是 `Step 13` 的阻塞项，但都值得在下一轮持续推进。

## P0：持续优化首批输入
- 建立多语言 SDK 的真实生成、版本冻结与发布归档链路，避免当前 README / facade 边界长期停留在占位与契约层。
- 把 `Pre-Release Tier` 与 `Capacity Tier` 纳入可重复执行的量化门禁，避免 `Step 11` 的证据长期只停留在 `CI Smoke Tier / standalone.split-services.development`。
- 把 `open-chat-test` 的 scripted validation 向 Bash 路径补齐与固化，形成跨脚本入口的对称 E2E 证据。
- 梳理 close / error registry 的客户端恢复策略，让 CLI / SDK / operator 文档对 session disconnect、overload、goaway、resume fallback 的处理基线更完整。

## P1：发布与运维深化
- 形成正式 release bundle 产物目录与版本归档约定，把本轮 `review` 结论提升为可归档、可审计、可回滚的发布物。
- 把 `status / inspect / repair / archive / prune / preview / restore` 的帮助面核对升级为最小 smoke 行为核对。
- 为 `standalone.split-services.development` 追加与 `standalone.split-services.development` 对称的关键发布后验证样本，避免 profile 只在帮助面和模板层对齐。

## P1：架构与治理深化
- 把 Wave D 的发布基线回写到更细粒度的部署、HA/DR、观测与协议治理架构文档中，形成“发布边界”和“下一轮不做什么”的显式约束。
- 建立从 `compatibility matrix` 到 `SDK facade` 再到 CLI / operator 验证入口的单一索引页，降低多处 review 证据的查找成本。

## P2：长期项
- 推进多 cell / 多 region 的正式 rollout orchestration 与灾备切换演练。
- 补齐跨环境发布、灰度、kill switch、生效观测的联动剧本。
- 评估把重复的 public-auth 测试守卫抽成共享测试辅助，减少未来发布前 lint 收口成本。

## 当前建议
1. 先完成 `Wave D / 93` 总验收。
2. 进入持续优化模式后，优先从 P0 开始。
3. 每完成一个持续优化条目，都补 `docs/review/` 与必要的 `docs/架构/` 回写。
