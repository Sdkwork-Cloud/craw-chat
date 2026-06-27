const DEFAULT_PERMISSIONS_POLICY = [
  'accelerometer=()',
  'camera=()',
  'geolocation=()',
  'gyroscope=()',
  'magnetometer=()',
  'microphone=()',
  'payment=()',
  'usb=()',
].join(', ');

function createContentSecurityPolicy({
  allowInlineScripts = false,
  allowUnsafeEval = false,
} = {}) {
  const scriptSrc = ["'self'"];

  if (allowInlineScripts) {
    scriptSrc.push("'unsafe-inline'");
  }

  if (allowUnsafeEval) {
    scriptSrc.push("'unsafe-eval'");
  }

  return [
    "default-src 'self'",
    "base-uri 'self'",
    "connect-src 'self' http: https: ws: wss:",
    "font-src 'self' data:",
    "frame-ancestors 'none'",
    "img-src 'self' data: blob:",
    "object-src 'none'",
    `script-src ${scriptSrc.join(' ')}`,
    "style-src 'self' 'unsafe-inline'",
  ].join('; ');
}

export const WEB_SECURITY_HEADERS = Object.freeze({
  'Content-Security-Policy': createContentSecurityPolicy(),
  'Permissions-Policy': DEFAULT_PERMISSIONS_POLICY,
  'Referrer-Policy': 'strict-origin-when-cross-origin',
  'X-Content-Type-Options': 'nosniff',
  'X-Frame-Options': 'DENY',
});

export function createWebSecurityHeaders(options = {}) {
  return {
    ...WEB_SECURITY_HEADERS,
    'Content-Security-Policy': createContentSecurityPolicy(options),
  };
}

export function applyWebSecurityHeaders(target, options = {}) {
  const headers = createWebSecurityHeaders(options);

  for (const [headerName, headerValue] of Object.entries(headers)) {
    target.setHeader(headerName, headerValue);
  }

  return headers;
}
