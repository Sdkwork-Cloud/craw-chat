export function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

export function readOverridePath(source, packageName) {
  const match = source.match(
    new RegExp(`^\\s{2}${escapeRegExp(packageName)}:\\s*\\r?\\n\\s{4}path:\\s*(.+)$`, 'm'),
  );
  return match ? match[1].trim().replace(/^['"]|['"]$/g, '') : '';
}
