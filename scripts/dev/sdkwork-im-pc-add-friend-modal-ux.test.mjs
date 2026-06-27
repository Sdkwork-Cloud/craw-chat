import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const source = fs.readFileSync(
  path.join(repoRoot, 'apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/AddFriendModal.tsx'),
  'utf8',
);

assert.match(
  source,
  /const\s+\[searchNotice,\s*setSearchNotice\]\s*=\s*useState<[^>]*>\(null\)/u,
  'add friend modal must keep inline search feedback state below the search box',
);
assert.match(
  source,
  /useTranslation/u,
  'add friend modal must use react-i18next for user-facing copy',
);
assert.match(
  source,
  /<ModalWrapper[\s\S]*width=["']w-\[640px\]["'][\s\S]*height=["']h-\[520px\]["']/u,
  'add friend modal must use a larger fixed dialog size so search feedback and result details do not feel cramped',
);
assert.match(
  source,
  /className=["'][^"']*flex h-full min-h-0 flex-col/u,
  'add friend modal content must fill the larger dialog height with a stable vertical layout',
);
assert.match(
  source,
  /setSearchNotice\(\{\s*type:\s*['"]loading['"],\s*message:\s*t\(['"]contacts\.addFriend\.notice\.searching['"]\)\s*\}\)/u,
  'add friend modal must show an in-modal loading notice while the SDK search request is in flight',
);
assert.match(
  source,
  /setSearchNotice\(\{\s*type:\s*['"]empty['"],\s*message:\s*t\(['"]contacts\.addFriend\.notice\.noResults['"],\s*\{\s*query:\s*normalizedQuery\s*\}\)\s*\}\)/u,
  'add friend modal must show no-result feedback inline under the search box',
);
assert.doesNotMatch(
  source,
  /toast\(['"][^'"]*(?:not found|未找到该用户)[^'"]*['"],\s*['"]error['"]\)/iu,
  'add friend modal must not use a top-level error toast for normal empty search results',
);
assert.match(
  source,
  /aria-live=["']polite["']/u,
  'add friend modal inline search feedback must be announced politely for assistive technology',
);
assert.match(
  source,
  /animate-spin/u,
  'add friend modal must provide a visible search-in-progress indicator inside the modal',
);
assert.match(
  source,
  /className=["'][^"']*min-h-0 flex-1 overflow-y-auto/u,
  'add friend modal must keep the searchable result area scrollable inside the larger dialog',
);
assert.match(
  source,
  /onChange=\{\(?event\)?\s*=>\s*\{[\s\S]*?setSearchNotice\(null\)/u,
  'editing the search query must clear stale inline search feedback',
);
assert.match(
  source,
  /React\.useEffect\(\(\)\s*=>\s*\{[\s\S]*?setSearchNotice\(null\)/u,
  'closing the add friend modal must reset inline search feedback',
);

console.log('sdkwork-im-pc add friend modal UX contract passed');
