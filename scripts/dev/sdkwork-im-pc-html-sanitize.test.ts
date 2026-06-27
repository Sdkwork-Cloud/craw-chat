import assert from 'node:assert/strict';
import { sanitizeHtmlForDisplay } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-commons/src/htmlSanitize.ts';

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
