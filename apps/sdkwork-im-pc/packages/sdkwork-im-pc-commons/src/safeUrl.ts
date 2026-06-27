const BLOCKED_URL_PROTOCOLS = new Set(['javascript:', 'data:', 'vbscript:']);
const ALLOWED_RESOURCE_PROTOCOLS = new Set(['http:', 'https:', 'blob:']);

function parseUrl(raw: string): URL | null {
  const trimmed = raw.trim();
  if (!trimmed) {
    return null;
  }

  try {
    if (trimmed.startsWith('/') && !trimmed.startsWith('//')) {
      if (typeof window === 'undefined') {
        return new URL(trimmed, 'https://localhost');
      }
      return new URL(trimmed, window.location.origin);
    }
    return new URL(trimmed);
  } catch {
    return null;
  }
}

export function sanitizeMessageUrl(raw: string | null | undefined): string | null {
  if (typeof raw !== 'string') {
    return null;
  }

  const url = parseUrl(raw);
  if (!url) {
    return null;
  }

  const protocol = url.protocol.toLowerCase();
  if (BLOCKED_URL_PROTOCOLS.has(protocol)) {
    return null;
  }

  if (raw.trim().startsWith('/') && !raw.trim().startsWith('//')) {
    return raw.trim();
  }

  if (!ALLOWED_RESOURCE_PROTOCOLS.has(protocol)) {
    return null;
  }

  return url.href;
}

export function sanitizeMessageLinkHref(raw: string | null | undefined): string | null {
  if (typeof raw !== 'string') {
    return null;
  }

  const trimmed = raw.trim();
  if (!trimmed || trimmed.startsWith('/') || trimmed.startsWith('//')) {
    return null;
  }

  return sanitizeMessageUrl(trimmed);
}

export function sanitizeEnterpriseWebsiteUrl(raw: string | null | undefined): string | null {
  if (typeof raw !== 'string') {
    return null;
  }

  const trimmed = raw.trim();
  if (!trimmed) {
    return null;
  }

  const candidate = trimmed.includes('://') ? trimmed : `https://${trimmed}`;
  return sanitizeMessageUrl(candidate);
}
