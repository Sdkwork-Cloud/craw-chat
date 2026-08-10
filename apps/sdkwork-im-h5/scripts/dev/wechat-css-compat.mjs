// WeChat DevTools / X5 内核（Chromium < 94）CSS 兼容降级插件。
//
// 背景：Tailwind CSS v4 产物大量使用现代 CSS（@layer、:where()、dvh 等），
// Vite 内置的 vite:css（lightningcss）在 tailwind 展开之前运行，且
// lightningcss 本身不对 @layer / :where() / dvh 做降级，老内核会直接忽略
// 这些规则，导致 tailwind 全部工具类失效、布局全乱。
//
// 该插件注册在 tailwindcss() 之后，负责：
//   1. 展开 @layer 块（删除 layer 语义，规则按源码顺序保留优先级）
//   2. 展开 :where() / :is() 伪类（等价选择器，仅优先级不同）
//   3. 将 dvh/svh/lvh 声明值降级为 vh（保留 @supports/@media 参数不替换）
//   4. logical properties -> physical（margin-inline -> margin-left/right 等，
//      WeChat X5 Chromium 86 不支持 logical properties）
//   5. 独立 transform 属性 translate:/rotate:/scale: -> transform:（Chromium
//      104+ 才支持独立属性，86 内核会整条声明失效）
//   6. 用 lightningcss 兜底降级 oklch / :is / inset 等（tailwind 产物未经过
//      vite:css 的 lightningcss）
import { transform as lightningTransform } from 'lightningcss';

const CSS_FILE_RE = /\.css(\?|$)/;
const LAYER_STATEMENT_RE = /@layer\s+[a-zA-Z_][\w-]*(\s*,\s*[a-zA-Z_][\w-]*)*\s*;/g;
const VIEWPORT_UNIT_RE = /(\d+(?:\.\d+)?)(dvh|svh|lvh)\b/g;
const AT_RULE_PARAMS_RE = /@(?:supports|media)\s*\([^)]*\)/g;

// 展开 `@layer name { ... }` 块；`@layer a, b;` 语句直接删除。
// 块内容提升到原位置，@media/@supports 等外层规则原样保留。
function expandLayers(css) {
  const src = css.replace(LAYER_STATEMENT_RE, '');
  const out = [];
  const n = src.length;
  let i = 0;
  while (i < n) {
    const m = /@layer\s+[a-zA-Z_][\w-]*\s*\{/.exec(src.slice(i));
    if (!m) {
      out.push(src.slice(i));
      break;
    }
    const start = i + m.index;
    out.push(src.slice(i, start));
    const bodyStart = start + m[0].length - 1; // index of '{'
    let depth = 1;
    let j = bodyStart + 1;
    let inStr = null;
    let esc = false;
    while (j < n && depth > 0) {
      const ch = src[j];
      if (esc) {
        esc = false;
      } else if (ch === '\\') {
        esc = true;
      } else if (inStr) {
        if (ch === inStr) inStr = null;
      } else if (ch === '"' || ch === "'") {
        inStr = ch;
      } else if (ch === '{') {
        depth += 1;
      } else if (ch === '}') {
        depth -= 1;
      }
      j += 1;
    }
    out.push(src.slice(bodyStart + 1, Math.max(bodyStart + 1, j - 1)));
    i = j;
  }
  return out.join('');
}

// 按顶层（跳过引号、转义与嵌套括号）拆分。
function splitTopLevel(str, separator) {
  const parts = [];
  let depth = 0;
  let inStr = null;
  let esc = false;
  let cur = '';
  for (const ch of str) {
    if (esc) {
      esc = false;
      cur += ch;
      continue;
    }
    if (ch === '\\') {
      esc = true;
      cur += ch;
      continue;
    }
    if (inStr) {
      if (ch === inStr) inStr = null;
      cur += ch;
      continue;
    }
    if (ch === '"' || ch === "'") {
      inStr = ch;
      cur += ch;
      continue;
    }
    if (ch === '(') depth += 1;
    else if (ch === ')') depth -= 1;
    if (ch === separator && depth === 0) {
      parts.push(cur);
      cur = '';
      continue;
    }
    cur += ch;
  }
  parts.push(cur);
  return parts;
}

// 展开选择器中的 :where(...) / :is(...)（等价选择器，仅优先级不同）。
function expandWhereInSelector(sel) {
  const m = /:(?:where|is)\(/.exec(sel);
  if (!m) return sel;
  const idx = m.index;
  const prefix = sel.slice(0, idx);
  const open = idx + m[0].length;
  // 匹配 '(' ... ')'
  let depth = 0;
  let close = -1;
  let inStr = null;
  let esc = false;
  const n = sel.length;
  for (let k = open; k < n; k += 1) {
    const ch = sel[k];
    if (esc) {
      esc = false;
    } else if (ch === '\\') {
      esc = true;
    } else if (inStr) {
      if (ch === inStr) inStr = null;
    } else if (ch === '"' || ch === "'") {
      inStr = ch;
    } else if (ch === '(') {
      depth += 1;
    } else if (ch === ')') {
      if (depth === 0) { close = k; break; }
      depth -= 1;
    }
  }
  if (close === -1) return sel; // 结构异常，保持原样
  const inner = sel.slice(open, close);
  const parts = splitTopLevel(inner, ',');
  const suffix = sel.slice(close + 1);
  return parts
    .map((part) => expandWhereInSelector(prefix + part + suffix))
    .join(',');
}

// 扫描所有规则块，对选择器执行 :where 展开。
function expandWhere(css) {
  const out = [];
  const n = css.length;
  let segStart = 0;
  let depth = 0;
  let inStr = null;
  let esc = false;
  let i = 0;
  while (i < n) {
    const ch = css[i];
    if (esc) {
      esc = false;
    } else if (ch === '\\') {
      esc = true;
    } else if (inStr) {
      if (ch === inStr) inStr = null;
    } else if (ch === '"' || ch === "'") {
      inStr = ch;
    } else if (ch === '{') {
      const seg = css.slice(segStart, i);
      out.push(expandWhereInSelector(seg));
      out.push('{');
      depth += 1;
      segStart = i + 1;
    } else if (ch === '}') {
      depth -= 1;
      out.push(css.slice(segStart, i + 1));
      segStart = i + 1;
    } else if (ch === ';' && depth === 0) {
      out.push(css.slice(segStart, i + 1));
      segStart = i + 1;
    }
    i += 1;
  }
  out.push(css.slice(segStart));
  return out.join('');
}

// dvh/svh/lvh -> vh，跳过 @supports/@media 参数（保留特性探测语义）。
function lowerViewportUnits(css) {
  const skip = [];
  let m;
  AT_RULE_PARAMS_RE.lastIndex = 0;
  while ((m = AT_RULE_PARAMS_RE.exec(css))) skip.push([m.index, m.index + m[0].length]);
  const out = [];
  let last = 0;
  VIEWPORT_UNIT_RE.lastIndex = 0;
  let um;
  while ((um = VIEWPORT_UNIT_RE.exec(css))) {
    if (skip.some(([s, e]) => um.index >= s && um.index < e)) continue;
    out.push(css.slice(last, um.index), um[1], 'vh');
    last = um.index + um[0].length;
  }
  out.push(css.slice(last));
  return out.join('');
}

// logical properties -> physical（Chromium < 87 不支持 logical properties）。
// kind: 'pair' 双值（inline/block 本体）拆成左右/上下两条；'target' 单目标直转。
const LOGICAL_RULES = {
  'margin-inline': { kind: 'pair', a: 'margin-left', b: 'margin-right' },
  'margin-block': { kind: 'pair', a: 'margin-top', b: 'margin-bottom' },
  'padding-inline': { kind: 'pair', a: 'padding-left', b: 'padding-right' },
  'padding-block': { kind: 'pair', a: 'padding-top', b: 'padding-bottom' },
  'inset-inline': { kind: 'pair', a: 'left', b: 'right' },
  'inset-block': { kind: 'pair', a: 'top', b: 'bottom' },
  'border-inline-style': { kind: 'pair', a: 'border-left-style', b: 'border-right-style' },
  'border-block-style': { kind: 'pair', a: 'border-top-style', b: 'border-bottom-style' },
  'border-inline-width': { kind: 'pair', a: 'border-left-width', b: 'border-right-width' },
  'border-block-width': { kind: 'pair', a: 'border-top-width', b: 'border-bottom-width' },
  'border-inline-color': { kind: 'pair', a: 'border-left-color', b: 'border-right-color' },
  'border-block-color': { kind: 'pair', a: 'border-top-color', b: 'border-bottom-color' },
  'scroll-margin-inline': { kind: 'pair', a: 'scroll-margin-left', b: 'scroll-margin-right' },
  'scroll-margin-block': { kind: 'pair', a: 'scroll-margin-top', b: 'scroll-margin-bottom' },
  'scroll-padding-inline': { kind: 'pair', a: 'scroll-padding-left', b: 'scroll-padding-right' },
  'scroll-padding-block': { kind: 'pair', a: 'scroll-padding-top', b: 'scroll-padding-bottom' },
  'margin-inline-start': { kind: 'target', target: 'margin-left' },
  'margin-inline-end': { kind: 'target', target: 'margin-right' },
  'margin-block-start': { kind: 'target', target: 'margin-top' },
  'margin-block-end': { kind: 'target', target: 'margin-bottom' },
  'padding-inline-start': { kind: 'target', target: 'padding-left' },
  'padding-inline-end': { kind: 'target', target: 'padding-right' },
  'padding-block-start': { kind: 'target', target: 'padding-top' },
  'padding-block-end': { kind: 'target', target: 'padding-bottom' },
  'inset-inline-start': { kind: 'target', target: 'left' },
  'inset-inline-end': { kind: 'target', target: 'right' },
  'inset-block-start': { kind: 'target', target: 'top' },
  'inset-block-end': { kind: 'target', target: 'bottom' },
  'border-inline-start-width': { kind: 'target', target: 'border-left-width' },
  'border-inline-end-width': { kind: 'target', target: 'border-right-width' },
  'border-block-start-width': { kind: 'target', target: 'border-top-width' },
  'border-block-end-width': { kind: 'target', target: 'border-bottom-width' },
  'border-inline-start-style': { kind: 'target', target: 'border-left-style' },
  'border-inline-end-style': { kind: 'target', target: 'border-right-style' },
  'border-block-start-style': { kind: 'target', target: 'border-top-style' },
  'border-block-end-style': { kind: 'target', target: 'border-bottom-style' },
};

// 独立 transform 属性转 transform() 函数（Chromium < 104 不支持 translate/rotate/scale
// 独立属性）。顺序按规范 translate -> rotate -> scale。
function toTransformFn(prop, value) {
  const v = value.trim();
  if (v === 'none' || !v) return null;
  if (prop === 'translate') {
    const parts = splitTopLevel(v, ' ').filter(Boolean);
    if (parts.length === 3) return `translate3d(${parts.join(', ')})`;
    return `translate(${parts.join(', ')})`;
  }
  if (prop === 'rotate') {
    const parts = splitTopLevel(v, ' ').filter(Boolean);
    if (parts.length >= 4) {
      return `rotate3d(${parts.slice(0, 3).join(', ')}, ${parts.slice(3).join(' ')})`;
    }
    return `rotate(${v})`;
  }
  if (prop === 'scale') {
    const parts = splitTopLevel(v, ' ').filter(Boolean);
    if (parts.length === 3) return `scale3d(${parts.join(', ')})`;
    return `scale(${parts.join(', ')})`;
  }
  return null;
}

// 处理单个声明块（叶子规则）：logical -> physical、独立 transform -> transform。
function processDeclarationBlock(body) {
  const decls = splitTopLevel(body, ';');
  const parts = [];
  const transforms = [];
  let hasTransform = false;
  let webkitBackdrop = null;
  let hasBackdrop = false;
  for (const decl of decls) {
    const trimmed = decl.trim();
    if (!trimmed) {
      parts.push(decl);
      continue;
    }
    const colon = trimmed.indexOf(':');
    if (colon === -1) {
      parts.push(decl);
      continue;
    }
    const prop = trimmed.slice(0, colon).trim().toLowerCase();
    const value = trimmed.slice(colon + 1).trim();
    if (prop === 'translate' || prop === 'rotate' || prop === 'scale') {
      transforms.push({ prop, value });
      continue;
    }
    if (prop === 'transform') {
      hasTransform = true;
      parts.push(decl.endsWith(';') ? decl : `${decl};`);
      continue;
    }
    if (prop === '-webkit-backdrop-filter') {
      // 记录 webkit 前缀版本；若块内缺少标准版本则块尾补充，让 Blink 内核
      // （微信开发者工具 X5/Chromium）走标准实现，避免 webkit 路径的半透明
      // 背景渲染差异导致"背景透明"。
      webkitBackdrop = value;
      parts.push(decl.endsWith(';') ? decl : `${decl};`);
      continue;
    }
    if (prop === 'backdrop-filter') {
      hasBackdrop = true;
      parts.push(decl.endsWith(';') ? decl : `${decl};`);
      continue;
    }
    const logical = LOGICAL_RULES[prop];
    if (logical) {
      if (logical.kind === 'pair') {
        const vals = splitTopLevel(value, ' ').filter(Boolean);
        const a = vals[0] ?? '';
        const b = vals[1] ?? vals[0] ?? '';
        if (a) parts.push(`${logical.a}: ${a};`);
        if (b) parts.push(`${logical.b}: ${b};`);
      } else {
        parts.push(`${logical.target}: ${value};`);
      }
      continue;
    }
    parts.push(decl.endsWith(';') ? decl : `${decl};`);
  }
  if (webkitBackdrop !== null && !hasBackdrop) {
    parts.push(`backdrop-filter: ${webkitBackdrop};`);
  }
  // 已有 transform: 的规则不做合并（独立属性在老内核被忽略、transform 生效）。
  if (transforms.length > 0 && !hasTransform) {
    // 规范顺序：translate -> rotate -> scale
    const order = { translate: 0, rotate: 1, scale: 2 };
    const fns = transforms
      .sort((a, b) => order[a.prop] - order[b.prop])
      .map(({ prop, value }) => toTransformFn(prop, value))
      .filter(Boolean);
    if (fns.length > 0) parts.push(`transform: ${fns.join(' ')};`);
  }
  return parts.join('');
}

// 块内容是否含嵌套 `{`（引号感知）。
function hasNestedBrace(body) {
  let inStr = null;
  let esc = false;
  for (const ch of body) {
    if (esc) esc = false;
    else if (ch === '\\') esc = true;
    else if (inStr) { if (ch === inStr) inStr = null; }
    else if (ch === '"' || ch === "'") inStr = ch;
    else if (ch === '{') return true;
  }
  return false;
}

// 扫描规则块：叶子块做声明级降级，嵌套块（@media/@supports 内）递归处理。
function lowerLogicalAndTransforms(css) {
  const out = [];
  const n = css.length;
  let i = 0;
  let segStart = 0;
  let inStr = null;
  let esc = false;
  let depth = 0;
  let blockStart = -1;
  while (i < n) {
    const ch = css[i];
    if (esc) {
      esc = false;
    } else if (ch === '\\') {
      esc = true;
    } else if (inStr) {
      if (ch === inStr) inStr = null;
    } else if (ch === '"' || ch === "'") {
      inStr = ch;
    } else if (ch === '{') {
      if (depth === 0) {
        out.push(css.slice(segStart, i + 1));
        blockStart = i + 1;
      }
      depth += 1;
    } else if (ch === '}') {
      depth -= 1;
      if (depth === 0) {
        const body = css.slice(blockStart, i);
        out.push(hasNestedBrace(body) ? lowerLogicalAndTransforms(body) : processDeclarationBlock(body));
        out.push('}');
        segStart = i + 1;
      }
    } else if (ch === ';' && depth === 0) {
      out.push(css.slice(segStart, i + 1));
      segStart = i + 1;
    }
    i += 1;
  }
  out.push(css.slice(segStart));
  return out.join('');
}

export { lowerLogicalAndTransforms, lowerViewportUnits, expandLayers, expandWhere };

export function wechatCssCompatPlugin() {
  return {
    name: 'sdkwork-im-h5:wechat-css-compat',
    transform(code, id) {
      if (!CSS_FILE_RE.test(id)) return null;
      if (id.includes('node_modules')) return null;
      if (!/@layer|:where\(|dvh|svh|lvh/.test(code)) return null;
      let out = expandLayers(code);
      out = expandWhere(out);
      out = lowerViewportUnits(out);
      out = lowerLogicalAndTransforms(out);
      const result = lightningTransform({
        filename: id,
        code: Buffer.from(out),
        minify: false,
        targets: {
          chrome: 86 << 16,
          android: 86 << 16,
          ios_saf: 13 << 16,
          safari: 13 << 16,
        },
      });
      return { code: result.code.toString() };
    },
  };
}
