import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const messageListSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/MessageList.tsx');

assert.doesNotMatch(
  messageListSource,
  /group-hover\/msg:flex/u,
  'Message items must not reveal a hover-only action toolbar; right-click is the canonical message action entrypoint.',
);

assert.match(
  messageListSource,
  /\{\s*id:\s*['"]reaction['"][\s\S]*?label:\s*t\(['"]chat\.messageList\.contextMenu\.reaction['"]\)[\s\S]*?icon:\s*<Smile\s+size=\{14\}/u,
  'Message context menu must expose the emoji reaction action after removing the hover toolbar.',
);

assert.match(
  messageListSource,
  /handleReaction\(contextMenu\.msg\.id,\s*['"]👍['"]\)/u,
  'Message context menu reaction action must apply the default thumbs-up reaction.',
);

assert.doesNotMatch(
  messageListSource,
  /isMultiSelect\s*\|\|\s*fallbackMessageIds\.has\(msg\.id\)/u,
  'Fallback/system welcome messages must still open the right-click menu.',
);

assert.match(
  messageListSource,
  /const\s+isFallbackMessage\s*=\s*fallbackMessageIds\.has\(contextMenu\.msg\.id\)/u,
  'Message context menu must explicitly identify local fallback messages before choosing actions.',
);

assert.match(
  messageListSource,
  /if\s*\(\s*isFallbackMessage\s*\)\s*\{\s*return\s*\[\s*copyItem\s*\]/u,
  'Fallback/system welcome message menus must be limited to copy-only safe actions.',
);

console.log('sdkwork-im-pc message actions contract passed');
