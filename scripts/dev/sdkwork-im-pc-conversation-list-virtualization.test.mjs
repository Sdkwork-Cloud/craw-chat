import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const chatListSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ChatList.tsx',
);
const packageJson = JSON.parse(read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/package.json',
));

assert.ok(
  packageJson.dependencies?.['@tanstack/react-virtual'],
  '@sdkwork/im-pc-chat must depend on @tanstack/react-virtual',
);
assert.match(
  chatListSource,
  /import\s+\{\s*useVirtualizer\s*\}\s+from\s+['"]@tanstack\/react-virtual['"]/u,
  'ChatList must use the established tanstack virtualizer dependency',
);
assert.match(
  chatListSource,
  /const\s+CHAT_LIST_ROW_HEIGHT\s*=\s*64/u,
  'ChatList must keep a stable 64px conversation row estimate',
);
assert.match(
  chatListSource,
  /const\s+CHAT_LIST_OVERSCAN\s*=\s*[1-9]\d*/u,
  'ChatList must declare a positive overscan window',
);
assert.match(
  chatListSource,
  /useVirtualizer\(\{[\s\S]*?count:\s*sortedChats\.length,[\s\S]*?getScrollElement:\s*\(\)\s*=>\s*listContainerRef\.current,[\s\S]*?estimateSize:\s*\(\)\s*=>\s*CHAT_LIST_ROW_HEIGHT,[\s\S]*?overscan:\s*CHAT_LIST_OVERSCAN,[\s\S]*?\}\)/u,
  'ChatList must virtualize the loaded conversation window with its scroll element, fixed row size, and overscan',
);
assert.match(
  chatListSource,
  /getVirtualItems\(\)/u,
  'ChatList must render only the virtual conversation rows',
);
assert.doesNotMatch(
  chatListSource,
  /sortedChats\.map\s*\(/u,
  'ChatList must not render every loaded conversation',
);
assert.doesNotMatch(
  chatListSource,
  /AnimatePresence|motion\.(?:button|div)/u,
  'ChatList must not keep per-row motion or AnimatePresence rendering that defeats virtualization',
);
assert.match(
  chatListSource,
  /remaining\s*<\s*120[\s\S]*?onLoadMoreChats\(\)/u,
  'ChatList must retain on-demand server cursor loading near the scroll boundary',
);
assert.match(
  chatListSource,
  /hasMoreChats\s*&&\s*onLoadMoreChats[\s\S]*?<button[\s\S]*?onClick=\{onLoadMoreChats\}[\s\S]*?chat\.list\.loadMore/u,
  'ChatList must retain the explicit server cursor load-more affordance',
);

console.log('sdkwork im pc conversation list virtualization contract passed.');
