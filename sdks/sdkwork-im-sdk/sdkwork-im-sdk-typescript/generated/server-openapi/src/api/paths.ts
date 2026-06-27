export const IM_API_PREFIX = '/im/v3/api';

export function imApiPath(path: string): string {
  if (!path) {
    return IM_API_PREFIX;
  }
  if (/^https?:\/\//i.test(path)) {
    return path;
  }
  const normalizedPrefixRaw = (IM_API_PREFIX || '').trim();
  const normalizedPrefix = normalizedPrefixRaw
    ? `/${normalizedPrefixRaw.replace(/^\/+|\/+$/g, '')}`
    : '';
  const normalizedPath = path.startsWith('/') ? path : `/${path}`;

  if (!normalizedPrefix || normalizedPrefix === '/') {
    return normalizedPath;
  }
  if (normalizedPath === normalizedPrefix || normalizedPath.startsWith(`${normalizedPrefix}/`)) {
    return normalizedPath;
  }
  return `${normalizedPrefix}${normalizedPath}`;
}
