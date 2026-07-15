#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const read = (relativePath) => fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
const service = read('apps/sdkwork-im-pc/packages/sdkwork-im-console-communications/src/services/GroupService.ts');
const component = read('apps/sdkwork-im-pc/packages/sdkwork-im-console-communications/src/ConsoleGroups.tsx');
const messages = read('apps/sdkwork-im-pc/packages/sdkwork-im-console-communications/src/ConsoleMessages.tsx');

assert.match(service, /q\?: string/u);
assert.match(service, /\.\.\.\(q \? \{ q \} : \{\}\)/u);
assert.doesNotMatch(service, /matchesGroupSearch|search\?: string/u);
assert.match(component, /window\.setTimeout\([\s\S]*?, 250\)/u);
assert.match(component, /requestSequenceRef/u);
assert.match(component, /加载失败，重试/u);
assert.doesNotMatch(component, /搜索仅作用于已加载结果页/u);
assert.match(messages, /catch\s*\{/u);
assert.match(messages, /setLoadError\(true\)/u);
assert.match(messages, /消息审计能力当前不可用/u);

process.stdout.write('sdkwork-im console groups contract passed\n');
