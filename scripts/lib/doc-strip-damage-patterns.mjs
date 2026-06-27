/**
 * Curated strip-damage signatures left by prior U+FFFD batch repairs.
 * Each entry: [RegExp, human-readable label]
 */
export const DOC_STRIP_DAMAGE_PATTERNS = [
  [/\u5DF2\u65B0`/u, '已新` (expected 已新增`)'],
  [/\u5DF2\u964D`/u, '已降` (expected 已降到/已降至`)'],
  [/\u5DF2\u8FC1`/u, '已迁` (expected 已迁移`)'],
  [/\u5B8C\u6574\u8FC1`/u, '完整迁` (expected 完整迁移`)'],
  [/\u5F00\u59CB\u8FC1`/u, '开始迁` (expected 开始迁移`)'],
  [/\u4E0D\u80FD\u8BC1`/u, '不能证` (expected 不能证明`)'],
  [/\u786C\u7F16`/u, '硬编` (expected 硬编码`)'],
  [/\u4F18\u5148`/u, '优先` (expected 优先在/优先由`)'],
  [/\u6536\u53E3\u5F84`/u, '收口径` (expected 收口到`)'],
  [/\u4ECD\u4F4D`Wave/u, '仍位`Wave (expected 仍位于 `Wave)'],
  [/\u6D88`runtime/u, '消`runtime (expected 消费 `runtime)'],
  [/\u8FD9\u8BC1`/u, '这证` (expected 这证明`)'],
  [/\u8FD9\u8FDB\u4E00\u6B65\u8BC1`/u, '这进一步证` (expected 这进一步证明`)'],
  [/\u9002\u7528\u9014/u, '适用途 (expected 适用)'],
  [/\u4E0D\u5B8C\u5168\u4E00$/mu, '不完全一 (expected 不完全一致)'],
  [/\u5DF2\u53D1`/u, '已发` (expected 已发布`)'],
  [/\u5DF2\u843D`/u, '已落` (expected 已落地`)'],
  [/\u53EA\u53D1`/u, '只发` (expected 只发布`)'],
  [/\u4E0D\u5BA3`/u, '不宣` (expected 不宣称`)'],
  [/\u6700owner/u, '最owner (expected 最早 owner)'],
  [/\u8FD9\u8BF4[A-Za-z]/u, '这说X (expected 这说明)'],
  [/\u53EAstep/u, '只step (expected 只在 step)'],
  [/\u5F53slice/u, '当slice (expected 当前 slice)'],
  [/\u8BC1\u660E\u786E$/mu, '证明确 (expected 证明确认)'],
  [/\u663E\u5F0F\u66B4[^露]/u, '显式暴 (expected 显式暴露)'],
  [/\u4E0B\u4E00\u8F6E\u8F93(?![出入])/u, '下一轮输 (expected 下一轮输出)'],
  [/\u662F\u4EC0(?![么])/u, '是什 (expected 是什么)'],
  [/\u5DF2\u5151\u73B0\u80FD$/mu, '已兑现能 (expected 已兑现能力)'],
  [/\u672A\u5151\u73B0\u80FD$/mu, '未兑现能 (expected 未兑现能力)'],
  [/\u7EE7\u7EED`[a-z]/u, '继续`code (expected 继续由/从 `code)'],
  [/\u56DE\u5199snapshot/u, '回写snapshot (expected 回写 snapshot)'],
  [/\u8FFDjournal/u, '追journal (expected 追写 journal)'],
  [/\u6E05\u5355marker/u, '清单marker (expected 清理 marker)'],
];

export function findStripDamageHits(source, relativePath) {
  const hits = [];
  for (const [pattern, label] of DOC_STRIP_DAMAGE_PATTERNS) {
    const match = source.match(pattern);
    if (match) {
      hits.push({ relativePath, label, sample: match[0] });
    }
  }
  return hits;
}
