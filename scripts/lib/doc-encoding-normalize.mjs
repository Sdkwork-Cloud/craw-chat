/**
 * Detect GBK/GB18030 markdown saved without UTF-8 conversion and normalize to UTF-8.
 */
export function normalizeMarkdownEncoding(buffer) {
  const utf8 = buffer.toString('utf8');
  if (!/\uFFFD/u.test(utf8)) {
    return utf8;
  }

  const gb18030 = new TextDecoder('gb18030').decode(buffer);
  if (!/\uFFFD/u.test(gb18030) && /[\u4e00-\u9fff]/u.test(gb18030)) {
    return gb18030;
  }

  return utf8;
}
