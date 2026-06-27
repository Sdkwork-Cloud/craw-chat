# Step 05 CP05-4 projection realtime fanout target owner 质量审计

## 1. 审计范围

- `services/projection-service/src/model.rs`
- `services/projection-service/src/lib.rs`
- `services/projection-service/tests/lib_structure_test.rs`
- `services/projection-service/tests/timeline_projection_test.rs`
- `services/sdkwork-im-cloud-gateway/src/node/effects.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/cluster_realtime_routing_e2e_test.rs`

## 2. 审计结论

- 审计通过项
  - principal -> device realtime fanout target 的解析 owner 已收口到 `projection-service`
  - `sdkwork-im-server` side-effect 路径不再直接把 raw `registered_devices(...)` 二次拼装成 fanout target
  - cross-node realtime routing 行为保持不变，远端 owner node 仍能收到 fanout 事件
- 当前裁定
  - 本轮推进了 `CP05-4`
  - 但这仍不构成 `CP05-4` 的整体闭环
  - `Step 05` 仍不可判定通过

## 3. 证据

- 结构红绿测试
  - `projection-service` owner seam
  - `sdkwork-im-server` consumer seam
- 行为验证
  - `projection-service` principal -> device pair 输出测试
- 回归验证
  - `projection-service` registered client route / latest sync seq 查询测试
  - `sdkwork-im-server` cross-node realtime routing e2e
- 额外验证
  - `cargo fmt --all`
  - `cargo fmt --all --check`

## 4. 风险与后续

- 本轮未发现新的架构偏离
- 剩余风险仍集中在 `CP05-4` 的其他 owner seam：
  - notification side-effect 与 projection / sync 的剩余衔接点
  - multi-client-route sync 最终收口
- 只有这些剩余 seam 也完成后，`CP05-4` 才能进入通过态
