import express from 'express';
import path from 'path';
import { createServer as createViteServer } from 'vite';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PORT = Number(process.env.SDKWORK_IM_H5_SERVER_PORT ?? 4178);
const HOST = process.env.SDKWORK_IM_H5_SERVER_HOST ?? '0.0.0.0';

async function startServer(): Promise<void> {
  const app = express();

  app.use(express.json());

  // Vite middleware for development; static dist in production.
  if (process.env.NODE_ENV !== 'production') {
    const vite = await createViteServer({
      server: { middlewareMode: true },
      appType: 'spa',
    });
    app.use(vite.middlewares);
  } else {
    const distPath = path.join(__dirname, 'dist');
    app.use(express.static(distPath));
    app.get('*', (_req, res) => {
      res.sendFile(path.join(distPath, 'index.html'));
    });
  }

  app.listen(PORT, HOST, () => {
    console.log(`[sdkwork-im-h5] server running on http://${HOST}:${PORT}`);
  });
}

void startServer().catch((error: unknown) => {
  console.error('[sdkwork-im-h5] server startup failed', error);
  process.exit(1);
});
