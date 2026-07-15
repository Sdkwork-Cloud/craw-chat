#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const read = (...segments) => fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
const service = read('packages', 'sdkwork-im-h5-chat', 'src', 'services', 'chatInboxService.ts');
const page = read('packages', 'sdkwork-im-h5-chat', 'src', 'pages', 'ChatInboxPage.tsx');
const zhCN = JSON.parse(read(
  'packages',
  'sdkwork-im-h5-commons',
  'src',
  'i18n',
  'zh-CN',
  'communication',
  'im-h5-commons',
  'chat-inbox.json',
));
const enUS = JSON.parse(read(
  'packages',
  'sdkwork-im-h5-commons',
  'src',
  'i18n',
  'en-US',
  'communication',
  'im-h5-commons',
  'chat-inbox.json',
));

assert.match(service, /q\?: string/u);
assert.match(service, /\.\.\.\(q \? \{ q \} : \{\}\)/u);
assert.match(page, /window\.setTimeout\([\s\S]*?, searchQuery\.trim\(\) \? 250 : 0\)/u);
assert.match(page, /cursor: requestCursor,[\s\S]*?q: searchQuery\.trim\(\) \|\| undefined/u);
assert.equal(zhCN['chat.inbox.searchAria'], '搜索会话');
assert.equal(enUS['chat.inbox.searchAria'], 'Search conversations');

process.stdout.write('sdkwork-im H5 inbox search contract passed\n');
