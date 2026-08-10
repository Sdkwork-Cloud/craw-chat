import { createServer } from 'vite';
import path from 'path';
import { fileURLToPath } from 'url';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, '../..');
const server = await createServer({
  configFile: path.join(root, 'vite.config.ts'),
  server: { host: '127.0.0.1', port: 4180, strictPort: true, fs: { allow: ['E:/sdkwork-space'] } },
  optimizeDeps: { disabled: true },
  logLevel: 'error',
});
await server.listen();
const res = await fetch('http://127.0.0.1:4180/src/index.css');
const css = await res.text();
const checks = {
  'html,body background': /html,body,#root[^}]*background-color:\s*var\(--color-bg-color\)/.test(css),
  ':root --color-bg-color': /:root[^}]*--color-bg-color:\s*var\(--sdkwork-im-h5-bg-color\)/.test(css),
  ':root --sdkwork-im-h5-bg-color': /:root[^}]*--sdkwork-im-h5-bg-color:\s*#[0-9a-f]+/i.test(css),
  '.glass-tab-bar': /\.glass-tab-bar\s*\{[^}]*background:\s*var\(--color-glass-bg\)/.test(css),
  '.glass-header': /\.glass-header\s*\{[^}]*background:\s*var\(--color-glass-bg\)/.test(css),
  'bg-bg-color 类': /\.bg-bg-color\b/.test(css),
  'bg-glass-bg 类': /\.bg-glass-bg\b/.test(css),
  'bg-bg-color/opacity': /\.bg-bg-color\\\/\d+/.test(css),
};
console.log(JSON.stringify(checks, null, 1));
// 输出 glass-tab-bar 完整定义
const g = css.indexOf('.glass-tab-bar');
console.log('--- .glass-tab-bar 定义 ---');
console.log(css.slice(g, g + 200));
// 输出 :root 中用户主题变量块
const u = css.indexOf('--sdkwork-im-h5-primary-blue:');
console.log('--- 用户主题变量上下文 ---');
console.log(css.slice(Math.max(0, u - 40), u + 200));
await server.close();
