import { lowerLogicalAndTransforms } from './wechat-css-compat.mjs';

const cases = [
  {
    name: 'margin-inline 单值',
    input: '.a { margin-inline: calc(var(--spacing) * -1); color: red; }',
  },
  {
    name: 'padding-inline 双值',
    input: '.b { padding-inline: 1rem 2rem; }',
  },
  {
    name: 'translate 独立属性',
    input: '.c { --tw-translate-x: calc(var(--spacing) * 4); translate: var(--tw-translate-x) var(--tw-translate-y); }',
  },
  {
    name: 'rotate + scale + translate 组合',
    input: '.d { rotate: 45deg; scale: 105%; translate: 10px 20px; }',
  },
  {
    name: '已有 transform 时跳过',
    input: '.e { transform: translateX(1px); translate: 10px 0; }',
  },
  {
    name: '@media 嵌套',
    input: '@media (min-width: 48rem) { .f { padding-inline: 2rem; } }',
  },
  {
    name: 'divide-x 的 border-inline',
    input: '.g { border-inline-style: var(--tw-border-style); border-inline-start-width: 1px; border-inline-end-width: 1px; }',
  },
  {
    name: 'translate3d 三值',
    input: '.h { translate: 1px 2px 3px; }',
  },
  {
    name: 'rotate3d 方向向量',
    input: '.i { rotate: 1 0 0 90deg; }',
  },
  {
    name: 'scale3d 三值',
    input: '.j { scale: 2 2 2; }',
  },
  {
    name: '值含引号/括号',
    input: '.k { background: url("data:image/svg+xml;utf8,<svg></svg>"); margin-inline: 4px; }',
  },
  {
    name: 'margin-block',
    input: '.l { margin-block: 1rem 2rem; padding-block: 0.5rem; }',
  },
  {
    name: 'backdrop-filter 补标准版',
    input: '.m { -webkit-backdrop-filter: saturate(180%) blur(20px); background: var(--color-glass-bg); }',
  },
  {
    name: 'backdrop-filter 已有标准版不重复',
    input: '.n { -webkit-backdrop-filter: blur(8px); backdrop-filter: blur(8px); }',
  },
];

let pass = 0;
let fail = 0;
for (const c of cases) {
  try {
    const out = lowerLogicalAndTransforms(c.input);
    console.log(`=== ${c.name} ===`);
    console.log(out);
    pass += 1;
  } catch (e) {
    console.log(`=== ${c.name} === FAIL: ${e.message}`);
    fail += 1;
  }
}
console.log(`\n通过 ${pass}/${cases.length}, 失败 ${fail}`);
