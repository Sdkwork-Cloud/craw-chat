import express from 'express';
import path from 'path';
import { createServer as createViteServer } from 'vite';

const HOST = process.env.SDKWORK_IM_H5_SERVER_HOST ?? '0.0.0.0';

function resolveServerPort(): number {
  const value = process.env.SDKWORK_IM_H5_SERVER_PORT?.trim() || '4178';
  const port = Number.parseInt(value, 10);
  if (!/^\d+$/u.test(value) || port < 1 || port > 65_535) {
    throw new Error(`SDKWORK_IM_H5_SERVER_PORT must be a TCP port, received: ${value}`);
  }
  return port;
}

async function startServer(): Promise<void> {
  const app = express();
  const port = resolveServerPort();

  app.use(express.json());

  // Vite middleware for development; static dist in production.
  if (process.env.NODE_ENV !== 'production') {
    const vite = await createViteServer({
      server: { middlewareMode: true },
      appType: 'spa',
    });
    app.use(vite.middlewares);
  } else {
    const distPath = path.resolve(process.cwd(), 'dist');
    app.use(express.static(distPath));
    app.get('*', (_req, res) => {
      res.sendFile(path.join(distPath, 'index.html'));
    });
  }

  app.listen(port, HOST, () => {
    console.log(`[sdkwork-im-h5] server running on http://${HOST}:${port}`);
  });
}

void startServer().catch((error: unknown) => {
  console.error('[sdkwork-im-h5] server startup failed', error);
  process.exit(1);
});
