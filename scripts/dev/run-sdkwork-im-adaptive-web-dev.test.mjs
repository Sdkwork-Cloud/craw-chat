import assert from 'node:assert/strict';
import http from 'node:http';
import test from 'node:test';

import { createAdaptiveServer } from './run-sdkwork-im-adaptive-web-dev.mjs';
import { IM_WEB_CLIENTS } from '../lib/im-web-client-routing.mjs';

function listen(server) {
  return new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(0, '127.0.0.1', () => {
      server.off('error', reject);
      const address = server.address();
      resolve(new URL(`http://127.0.0.1:${address.port}`));
    });
  });
}

function close(server) {
  return new Promise((resolve, reject) => {
    server.close((error) => error ? reject(error) : resolve());
  });
}

function markerServer(marker) {
  return http.createServer((request, response) => {
    response.writeHead(200, { 'content-type': 'text/plain; charset=utf-8' });
    response.end(`${marker}:${request.url}`);
  });
}

function fetchText(origin, requestPath, userAgent) {
  return new Promise((resolve, reject) => {
    const request = http.get(new URL(requestPath, origin), {
      headers: { 'user-agent': userAgent },
    }, (response) => {
      const chunks = [];
      response.on('data', (chunk) => chunks.push(chunk));
      response.once('end', () => resolve({
        body: Buffer.concat(chunks).toString('utf8'),
        headers: response.headers,
        statusCode: response.statusCode,
      }));
    });
    request.setTimeout(5_000, () => request.destroy(new Error('request timed out')));
    request.once('error', reject);
  });
}

test('routes one browser origin by user agent while keeping API paths on the application ingress', async (t) => {
  const pcServer = markerServer('pc');
  const h5Server = markerServer('h5');
  const apiServer = markerServer('api');
  const pcTarget = await listen(pcServer);
  const h5Target = await listen(h5Server);
  const apiTarget = await listen(apiServer);
  const renderers = new Map([
    [IM_WEB_CLIENTS.PC, { client: IM_WEB_CLIENTS.PC, ready: true, target: pcTarget }],
    [IM_WEB_CLIENTS.H5, { client: IM_WEB_CLIENTS.H5, ready: true, target: h5Target }],
  ]);
  const ingress = createAdaptiveServer({ apiTarget, renderers });
  const ingressOrigin = await listen(ingress);

  t.after(async () => Promise.all([
    close(ingress),
    close(pcServer),
    close(h5Server),
    close(apiServer),
  ]));

  const desktop = await fetchText(ingressOrigin, '/workspace/inbox', 'Windows NT 10.0');
  assert.equal(desktop.body, 'pc:/workspace/inbox');
  assert.equal(desktop.headers.vary, 'user-agent');

  const mobile = await fetchText(ingressOrigin, '/workspace/inbox', 'iPhone Mobile');
  assert.equal(mobile.body, 'h5:/workspace/inbox');
  assert.equal(mobile.headers.vary, 'user-agent');

  const canonicalViteDependency = await fetchText(
    ingressOrigin,
    '/node_modules/.vite/sdkwork-im-pc/deps/dompurify.js?v=current',
    'Windows NT 10.0',
  );
  assert.equal(
    canonicalViteDependency.body,
    'pc:/node_modules/.vite/sdkwork-im-pc/deps/dompurify.js?v=current',
  );

  const staleViteDependency = await fetchText(
    ingressOrigin,
    '/node_modules/.vite/deps/dompurify.js?v=stale',
    'Windows NT 10.0',
  );
  assert.equal(staleViteDependency.statusCode, 410);
  assert.match(staleViteDependency.headers['content-type'], /^text\/plain/u);
  assert.equal(staleViteDependency.headers['cache-control'], 'no-store');
  assert.equal(staleViteDependency.headers.vary, 'user-agent');
  assert.doesNotMatch(staleViteDependency.body, /<html/u);

  const api = await fetchText(ingressOrigin, '/im/v3/api/realtime/ws?transport=polling', 'iPhone Mobile');
  assert.equal(api.body, 'api:/im/v3/api/realtime/ws?transport=polling');
  assert.equal(api.headers.vary, undefined);

  renderers.get(IM_WEB_CLIENTS.H5).ready = false;
  const mobileFallback = await fetchText(ingressOrigin, '/', 'iPhone Mobile');
  assert.equal(mobileFallback.body, 'pc:/');
  assert.equal(mobileFallback.headers.vary, 'user-agent');
});

test('falls back to H5 when a ready PC renderer becomes unreachable', async (t) => {
  const unavailableServer = markerServer('unavailable');
  const unavailableTarget = await listen(unavailableServer);
  await close(unavailableServer);

  const h5Server = markerServer('h5-fallback');
  const h5Target = await listen(h5Server);
  const apiServer = markerServer('api');
  const apiTarget = await listen(apiServer);
  const renderers = new Map([
    [IM_WEB_CLIENTS.PC, { client: IM_WEB_CLIENTS.PC, ready: true, target: unavailableTarget }],
    [IM_WEB_CLIENTS.H5, { client: IM_WEB_CLIENTS.H5, ready: true, target: h5Target }],
  ]);
  const ingress = createAdaptiveServer({ apiTarget, renderers });
  const ingressOrigin = await listen(ingress);

  t.after(async () => Promise.all([
    close(ingress),
    close(h5Server),
    close(apiServer),
  ]));

  const response = await fetchText(ingressOrigin, '/', 'Windows NT 10.0');
  assert.equal(response.statusCode, 200);
  assert.equal(response.body, 'h5-fallback:/');
  assert.equal(response.headers.vary, 'user-agent');
});
