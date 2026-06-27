# 150W - SDK leaf README release boundary 设计

## 设计目标

具体语言目录是消费方最容易直接进入的入口，因此叶子 README 不能只描述职责，还必须公开当前 bundle 级发布边界。

## 约束原则

- 叶子 README 不维护独立发布真源
- bundle 级 `sdk-release-catalog.json` 才是发布状态唯一真源
- 叶子 README 只负责把该真源向消费方公开

## 当前最小公开字段

每个叶子 README 至少应公开：

- `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
- `template_only_pending_generation`
- `not_published`

## 为什么必须下沉到叶子 README

- 总入口 `sdks/README.md` 只能解决 workspace 导航问题
- 真实消费方很可能直接打开：
  - `sdkwork-im-sdk-typescript`
  - `sdkwork-im-sdk-flutter`
  - `sdkwork-control-plane-sdk-typescript`
  - `sdkwork-control-plane-sdk-flutter`
- 如果叶子 README 不带发布边界，消费方仍会误以为这些目录已经具备独立的真实生成或发布状态

## 非目标

- 不把叶子 README 扩写成完整 release note
- 不在叶子 README 中新增独立版本号维护
- 不让叶子 README 脱离 bundle catalog 自行声明发布结论
