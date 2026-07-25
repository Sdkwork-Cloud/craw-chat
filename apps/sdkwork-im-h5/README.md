# Sdkwork IM H5

Sdkwork IM H5 是 SDKWork IM 的移动端 H5 应用，提供手机优先的移动 Web 通讯与协同体验。

## 运行

**前置条件：** Node.js、pnpm

1. 安装依赖：
   `pnpm install`
2. 启动开发服务：
   `pnpm dev`

## 部署配置

- `etc/sdkwork.deployment.config.json`：部署配置入口
- `etc/browser.runtime.json`：H5 渲染器运行时绑定
- `sdkwork.app.config.json`：应用清单与发布元数据

## 验证

```bash
node ../../../sdkwork-specs/tools/check-app-manifest-standard.mjs --root .
node ../../../sdkwork-specs/tools/check-source-config-standard.mjs --root .
node ../../../sdkwork-specs/tools/check-pnpm-script-standard.mjs --root . --product-prefix im,chat
node ../../scripts/dev/sdkwork-im-h5-architecture-standard.test.mjs
```
