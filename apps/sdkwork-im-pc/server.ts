import express from 'express';
import type { IncomingMessage, ServerResponse } from 'http';
import path from 'path';
import { createServer as createViteServer } from 'vite';
import { handleSdkworkChatLocalApiRequest } from './local-api';

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
