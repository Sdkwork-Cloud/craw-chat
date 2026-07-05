import express from 'express';
import type { IncomingMessage, ServerResponse } from 'http';
import path from 'path';
import { createServer as createViteServer } from 'vite';
import { handleSdkworkChatLocalApiRequest } from './local-api';

/**
 * Compose the Content-Security-Policy header value.
 *
 * Default policy is strict same-origin:
 * - `default-src 'self'`: deny by default for any unspecified directive.
 * - `script-src 'self'`: only allow scripts from the same origin. Generated
 *   Vite bundles are hashed assets served from `/assets/*`, so no inline
 *   scripts or `eval()` are required in production.
 * - `style-src 'self' 'unsafe-inline'`: React inline styles and Tailwind
 *   utilities inject inline styles, so `'unsafe-inline'` is required.
 * - `img-src 'self' data: blob: https:`: avatar and image attachments may be
 *   served from any HTTPS origin or as data/blob URIs.
 * - `font-src 'self' data:`: fonts are self-hosted or embedded as data URIs.
 * - `media-src 'self' data: blob:`: audio/video attachments.
 * - `connect-src 'self' <extra>`: XHR/fetch/WebSocket. Default to same
 *   origin (production typically fronts the API through the same origin
 *   via nginx). Operators may extend this with `SDKWORK_CSP_CONNECT_SRC`
 *   (comma-separated list of additional origins, e.g.
 *   `https://api.example.com wss://rtm.example.com`).
 * - `object-src 'none'`: ban Flash/Java plugins.
 * - `base-uri 'self'`: prevent `<base>` injection.
 * - `form-action 'self'`: limit form submissions to same origin.
 * - `frame-ancestors 'none'`: equivalent to `X-Frame-Options: DENY`,
 *   prevents clickjacking.
 * - `frame-src 'self'`: only same-origin iframes.
 * - `worker-src 'self' blob:`: allow blob workers (used by some SDKs).
 */
function composeContentSecurityPolicy(): string {
  const extraConnectSrc = (process.env.SDKWORK_CSP_CONNECT_SRC ?? '').trim();
  const connectSrc = extraConnectSrc
    ? `'self' ${extraConnectSrc}`
    : "'self'";
  return [
    "default-src 'self'",
    "script-src 'self'",
    "style-src 'self' 'unsafe-inline'",
    "img-src 'self' data: blob: https:",
    "font-src 'self' data:",
    "media-src 'self' data: blob:",
    `connect-src ${connectSrc}`,
    "object-src 'none'",
    "base-uri 'self'",
    "form-action 'self'",
    "frame-ancestors 'none'",
    "frame-src 'self'",
    "worker-src 'self' blob:",
  ].join('; ');
}

/**
 * Security headers middleware applied to every response in production mode.
 * In development, Vite's middleware injects its own HMR scripts and React
 * Refresh runtime which require `'unsafe-inline'` and `'unsafe-eval'` for
 * scripts, so we skip CSP there to avoid breaking HMR.
 */
function applySecurityHeaders(_req: IncomingMessage, res: ServerResponse, next: () => void): void {
  res.setHeader('Content-Security-Policy', composeContentSecurityPolicy());
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('X-Frame-Options', 'DENY');
  res.setHeader('Referrer-Policy', 'strict-origin-when-cross-origin');
  res.setHeader('Strict-Transport-Security', 'max-age=31536000; includeSubDomains');
  res.setHeader('Permissions-Policy', 'geolocation=(), microphone=(), camera=()');
  next();
}

async function startServer() {
  const app = express();
  const PORT = Number(process.env.PORT ?? 3000);

  app.use(express.json());
  app.use((req: IncomingMessage, res: ServerResponse, next: () => void) => {
    handleSdkworkChatLocalApiRequest(req, res, (req as IncomingMessage & { path?: string }).path ?? '/')
      .then((handled) => {
        if (!handled) {
          next();
        }
      })
      .catch(next);
  });

  if (process.env.NODE_ENV !== 'production') {
    const vite = await createViteServer({
      server: { middlewareMode: true },
      appType: 'spa',
    });
    app.use(vite.middlewares);
  } else {
    app.use(applySecurityHeaders);
    const distPath = path.join(process.cwd(), 'dist');
    app.use(express.static(distPath));
    app.get('*', (_req, res) => {
      res.sendFile(path.join(distPath, 'index.html'));
    });
  }

  app.listen(PORT, '0.0.0.0', () => {
    console.log(`Server running on http://localhost:${PORT}`);
  });
}

startServer();
