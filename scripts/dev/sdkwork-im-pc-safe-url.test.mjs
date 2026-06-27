#!/usr/bin/env node

import assert from 'node:assert/strict';

const { sanitizeEnterpriseWebsiteUrl, sanitizeMessageLinkHref, sanitizeMessageUrl } = await import('../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-commons/src/safeUrl.ts');

assert.equal(sanitizeMessageUrl('https://example.com/a.png'), 'https://example.com/a.png');
assert.equal(sanitizeMessageUrl('javascript:alert(1)'), null);
assert.equal(sanitizeMessageUrl('data:text/html,<script>alert(1)</script>'), null);
assert.equal(sanitizeMessageLinkHref('/relative/path'), null);
assert.equal(sanitizeMessageLinkHref('https://example.com/path'), 'https://example.com/path');
assert.equal(sanitizeEnterpriseWebsiteUrl('example.com'), 'https://example.com/');
assert.equal(sanitizeEnterpriseWebsiteUrl('javascript:alert(1)'), null);

console.log('sdkwork im pc safe url contract passed.');
