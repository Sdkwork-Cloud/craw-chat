import type { IncomingMessage, ServerResponse } from 'http';
import { listCommercialRuntimeModules } from './packages/sdkwork-im-pc-shell/src/moduleRegistry';

/** Dev-shell module catalog; production uses app SDK portal.home.retrieve(). */
const LOCAL_APP_MODULES = listCommercialRuntimeModules();

const LOCAL_SHELL_AGENT_RETIRED = {
  error:
    'Local dev-shell agent endpoints are retired. Promote document/icon agent flows to im-app-api and consume them through @sdkwork/im-app-sdk.',
  code: 'local_shell_agent_retired',
};

function sendJson(res: ServerResponse, statusCode: number, body: unknown): void {
  res.statusCode = statusCode;
  res.setHeader('Content-Type', 'application/json; charset=utf-8');
  res.end(JSON.stringify(body));
}

async function readJsonBody(req: IncomingMessage): Promise<Record<string, unknown>> {
  return new Promise((resolve, reject) => {
    const chunks: Buffer[] = [];
    req.on('data', (chunk) => chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk)));
    req.on('error', reject);
    req.on('end', () => {
      const rawBody = Buffer.concat(chunks).toString('utf8').trim();
      if (!rawBody) {
        resolve({});
        return;
      }
      try {
        const parsed = JSON.parse(rawBody);
        resolve(parsed && typeof parsed === 'object' && !Array.isArray(parsed) ? parsed : {});
      } catch (error) {
        reject(error);
      }
    });
  });
}

export async function handleSdkworkChatLocalApiRequest(
  req: IncomingMessage,
  res: ServerResponse,
  requestPath = new URL(req.url ?? '/', 'http://127.0.0.1').pathname,
): Promise<boolean> {
  try {
    if (requestPath === '/api/config/modules' && req.method === 'GET') {
      sendJson(res, 200, { modules: [...LOCAL_APP_MODULES] });
      return true;
    }
    if (requestPath === '/api/agent/doc' && req.method === 'POST') {
      await readJsonBody(req);
      sendJson(res, 501, LOCAL_SHELL_AGENT_RETIRED);
      return true;
    }
    if (requestPath === '/api/agent/icon' && req.method === 'POST') {
      await readJsonBody(req);
      sendJson(res, 501, LOCAL_SHELL_AGENT_RETIRED);
      return true;
    }
    return false;
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Local API processing failed';
    sendJson(res, 500, { error: message });
    return true;
  }
}
