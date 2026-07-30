const MOBILE_USER_AGENT_PATTERN = /(?:android|blackberry|iemobile|iphone|ipad|ipod|mobile|opera mini|webos)/iu;

export const IM_WEB_CLIENTS = Object.freeze({
  H5: 'h5',
  PC: 'pc',
});

const CANONICAL_API_PATH_PATTERNS = Object.freeze([
  /^\/api(?:\/|$)/u,
  /^\/app\/v\d+\/api(?:\/|$)/u,
  /^\/backend\/v\d+\/api(?:\/|$)/u,
  /^\/im\/v\d+\/api(?:\/|$)/u,
  /^\/open\/v\d+\/api(?:\/|$)/u,
  /^\/(?:healthz|livez|metrics|openapi\.json|readyz)$/u,
]);

export function preferredImWebClient(userAgent) {
  return MOBILE_USER_AGENT_PATTERN.test(String(userAgent ?? ''))
    ? IM_WEB_CLIENTS.H5
    : IM_WEB_CLIENTS.PC;
}

export function imWebClientFallbackOrder(userAgent) {
  const preferred = preferredImWebClient(userAgent);
  return preferred === IM_WEB_CLIENTS.H5
    ? [IM_WEB_CLIENTS.H5, IM_WEB_CLIENTS.PC]
    : [IM_WEB_CLIENTS.PC, IM_WEB_CLIENTS.H5];
}

export function resolveAvailableImWebClient({
  availableClients = [],
  userAgent,
} = {}) {
  const available = new Set(availableClients);
  return imWebClientFallbackOrder(userAgent).find((client) => available.has(client));
}

export function isCanonicalImApiPath(requestUrl) {
  const pathname = String(requestUrl ?? '').split(/[?#]/u, 1)[0] || '/';
  return CANONICAL_API_PATH_PATTERNS.some((pattern) => pattern.test(pathname));
}
