import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const requireFromImPcCommons = createRequire(
  path.resolve(
    fileURLToPath(import.meta.url),
    '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-commons/package.json',
  ),
);

async function main(): Promise<void> {
  const { JSDOM } = requireFromImPcCommons('jsdom') as typeof import('jsdom');
  const { window } = new JSDOM('');
  Object.assign(globalThis, {
    window,
    document: window.document,
  });

  const { sanitizeHtmlForDisplay } = await import('../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-commons/src/htmlSanitize.ts');

  assert.equal(
    sanitizeHtmlForDisplay('<p>Hello</p><script>alert(1)</script>'),
    '<p>Hello</p>',
    'sanitizeHtmlForDisplay must strip script tags',
  );

  assert.equal(
    sanitizeHtmlForDisplay('<a href="javascript:alert(1)">bad</a>'),
    '<a>bad</a>',
    'sanitizeHtmlForDisplay must remove javascript: href values',
  );

  assert.equal(
    sanitizeHtmlForDisplay('<svg onload="alert(1)"><circle /></svg><p>ok</p>'),
    '<p>ok</p>',
    'sanitizeHtmlForDisplay must strip svg-based XSS vectors',
  );

  console.log('sdkwork-im-pc html sanitize contract passed');
}

void main();
