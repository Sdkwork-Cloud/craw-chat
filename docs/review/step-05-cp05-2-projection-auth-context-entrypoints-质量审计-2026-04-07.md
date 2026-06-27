# Step 05 CP05-2 Projection Auth Context Entrypoints 质量审计 - 2026-04-07

## 1. 审计范围

- `services/projection-service/src/access.rs`
- `services/projection-service/src/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node/projection.rs`
- `services/sdkwork-im-cloud-gateway/src/node/session.rs`
- `services/sdkwork-im-cloud-gateway/src/node.rs`
- `services/projection-service/tests/lib_structure_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 2. 审计结论

- 本轮改动与 `Wave B / Step 05 / CP05-2` 对齐，没有跨 step 乱序推进。
- auth-context authority owner 已真实下沉到 projection-service，而不是停留在 HTTP 或 sdkwork-im-server adapter。
- `projection-service` 与 `sdkwork-im-server` 的结构测试、整包测试均通过。
- `rustfmt --check` 通过，且本轮新增 warning 已清零。

## 3. 本轮消除的风险

- 消除了 projection-service HTTP 自己维护 device / inbox / timeline / summary / read-cursor authority 校验分支的风险。
- 消除了 sdkwork-im-server projection/session 继续重复线程化 `auth.tenant_id / auth.actor_id` 的风险。
- 消除了 `projection-service` 与 `sdkwork-im-server` 在 device scope / active member guard 上继续分叉的风险。

## 4. 仍然存在的剩余风险

- `services/sdkwork-im-cloud-gateway/src/node/effects.rs` 仍有 runtime read raw authority threading。
- 这说明 `CP05-2` 尚未整体闭环，`Step 05` 也不能进入 `91 / 95 / 97` 通过态。

## 5. 质量证据

- Red
  - `target-step05-cp05-2e-red-projection-structure`
  - `target-step05-cp05-2e-red-local-node-structure`
  - `target-step05-cp05-2e-red-local-node-session`
- Green / fresh verification
  - `target-step05-cp05-2e-green-projection-structure`
  - `target-step05-cp05-2e-green-local-node-structure`
  - `target-step05-cp05-2e-projection-full`
  - `target-step05-cp05-2e-local-node-full`

## 6. 审计判断

- 本轮可以判定为 `CP05-2` 继续推进。
- 本轮不能判定：
  - `CP05-2` 完成
  - `Step 05` 完成
  - `91 / 95 / 97` 对 `Step 05` 通过
  - `Wave B / 93` 可启动
