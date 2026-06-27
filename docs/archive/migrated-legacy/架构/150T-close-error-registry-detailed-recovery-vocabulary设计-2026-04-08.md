# 150T - close / error registry 细粒度恢复词汇设计

## 设计目标

恢复策略不能只停留在“需要重连”这类宽泛描述，必须让公开消费面共享同一组细粒度恢复输入。

## 词汇范围

本轮冻结以下最小细粒度恢复词汇：

- `4001`
- `session.disconnect`
- `reconnect_required`
- `pull-only`
- `events.pull`

## 语义定义

### 1. `4001`

- 表示显式 `session.disconnect` 的 websocket close code
- 这是 close / error registry 的正式组成部分，不是 SDK 本地可替换常量

### 2. `session.disconnect`

- 表示当前 session 已被显式作废
- 既出现在 `goaway` message，也出现在 close reason

### 3. `reconnect_required`

- 表示旧 session 在 fresh resume 之前不得继续发送设备级请求
- 它不是普通“重试失败”，而是 stale session 的明确拒绝信号

### 4. `pull-only`

- 表示 live push 暂时降级，但链路不一定已经断开
- 客户端必须进入拉取恢复路径，而不是直接断言消息丢失

### 5. `events.pull`

- 表示 pull-only 降级下继续追平窗口的正式恢复入口
- 该词汇必须在 operator / SDK 文档里公开，避免消费方把恢复动作写成私有旁路

## 文档落点

以下公开面必须保持一致：

- `docs/部署/CLI聊天验证与兼容矩阵.md`
- `sdks/sdkwork-im-sdk/README.md`
- `sdks/sdkwork-control-plane-sdk/README.md`
- `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`

## 非目标

- 不引入新的 close code
- 不扩展新的 runtime 恢复状态机
- 不提前宣称多语言 SDK 已把这些词汇消费成正式 API
