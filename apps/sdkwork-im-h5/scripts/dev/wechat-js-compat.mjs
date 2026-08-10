// WeChat DevTools / X5 内核（Chromium < 94）JS 语法兜底插件。
//
// 背景：若 vite 的 dev esbuild target 被改回 `esnext`（或 Vite 升级改变
// 默认行为），配合 `useDefineForClassFields: false`（Vite 对无 tsconfig 的
// SDK 源码自动注入 / tsconfig.json 显式设置），esbuild 会把 class static
// fields 编译为 ES2022 static blocks（`static {}`），微信开发者工具内核
// （< Chrome 94）会直接语法报错 `Unexpected token '{'`。
//
// 本插件在 vite:esbuild（内置）之后运行，兜底检测：只要模块代码里出现
// static blocks，就用 esbuild 按 es2020 目标二次降级为类外赋值
// （`_HttpClient.ACCESS_TOKEN_HEADER = "..."`），任何内核都可解析。
// 正常路径（vite.config 的 esbuild.target 已生效）下检测不到 static
// blocks，插件零开销直接跳过。
import { transformSync } from 'esbuild';

const JS_FILE_RE = /\.(m?[jt]s|[jt]sx)(\?|$)/;
// 只按 ES 版本降级（es2020 纯版本）：static blocks（ES2022）会降级为类外
// 赋值，而解构等 ES2015 语法保留，避免 esbuild 对旧浏览器 target 的
// "Transforming destructuring ... is not supported yet" 报错。
const COMPAT_TARGET = 'es2020';

export function wechatJsCompatPlugin() {
  return {
    name: 'sdkwork-im-h5:wechat-js-compat',
    transform(code, id) {
      if (!JS_FILE_RE.test(id)) return null;
      if (!code.includes('static {')) return null;
      const result = transformSync(code, {
        loader: 'js',
        target: COMPAT_TARGET,
        format: 'esm',
        sourcefile: id,
      });
      return { code: result.code };
    },
  };
}
