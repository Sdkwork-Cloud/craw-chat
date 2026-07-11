/**
 * Node ESM 加载器注册入口：为无扩展名的相对导入补全 .js 后缀。
 *
 * dist/ 编译产物使用 moduleResolution: "bundler"，import 路径不带 .js，
 * Node 原生 ESM 解析器要求显式扩展名。此钩子仅对相对路径且无扩展名的
 * 说明符补全 .js，不影响其他导入。
 *
 * 用法：
 *   node --import ./tests/extension-resolver.mjs --test tests/transport.test.mjs
 */
import { register } from 'node:module';

const hookCode = `
export async function resolve(specifier, context, nextResolve) {
  if (
    (specifier.startsWith('./') || specifier.startsWith('../')) &&
    !specifier.includes('.', 2)
  ) {
    const patched = specifier + '.js';
    try {
      return await nextResolve(patched, context);
    } catch {
      return nextResolve(specifier, context);
    }
  }
  return nextResolve(specifier, context);
}
`;

const dataUrl = 'data:text/javascript;base64,' + Buffer.from(hookCode).toString('base64');
register(dataUrl);
