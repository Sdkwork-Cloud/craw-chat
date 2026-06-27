import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const profileMenuSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/ProfileMenuModal.tsx');
const contactDetailSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/ContactDetailPane.tsx');
const sidebarSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/Sidebar.tsx');

assert.match(
  profileMenuSource,
  /currentUserChatId/u,
  'ProfileMenuModal must resolve a public chat id before rendering the profile header id',
);

assert.match(
  profileMenuSource,
  /navigator\.clipboard\.writeText\(\s*currentUserChatId\s*\)/u,
  'ProfileMenuModal id copy must copy the public chat id instead of the internal current user id',
);

assert.doesNotMatch(
  profileMenuSource,
  /navigator\.clipboard\.writeText\(\s*currentUser\.id\s*\)/u,
  'ProfileMenuModal must not copy the internal current user id as the user-facing IM id',
);

assert.match(
  profileMenuSource,
  /useTranslation/u,
  'ProfileMenuModal must use react-i18next for user-facing profile copy',
);

assert.match(
  profileMenuSource,
  /title=\{t\(['"]profile\.copyChatId['"]\)\}/u,
  'ProfileMenuModal user id copy control must expose a localized copy-ID title',
);

assert.match(
  profileMenuSource,
  /toast\(t\(['"]profile\.toast\.chatIdCopied['"]\),\s*["']success["']\)/u,
  'ProfileMenuModal must show a localized in-app success toast after copying the public chat id',
);

assert.match(
  contactDetailSource,
  /displayUserChatId/u,
  'ContactDetailPane must resolve a public chat id for profile display and copy actions',
);

assert.match(
  contactDetailSource,
  /navigator\.clipboard\.writeText\(\s*displayUserChatId\s*\)/u,
  'ContactDetailPane must copy the public chat id from the profile details menu',
);

assert.doesNotMatch(
  contactDetailSource,
  /navigator\.clipboard\.writeText\(\s*user\.id\s*\)/u,
  'ContactDetailPane must not copy the internal user id as the user-facing IM id',
);

assert.match(
  sidebarSource,
  /setCurrentUser\(\s*contactService\.getCurrentUser\(\)\s*\)/u,
  'Sidebar must keep current user state in sync with the current login session',
);

assert.match(
  sidebarSource,
  /contactService\.getUserById\(\s*sessionUser\.id\s*\)/u,
  'Sidebar must hydrate the current user from the real social profile API before rendering the profile menu Chat ID',
);

assert.match(
  sidebarSource,
  /setCurrentUser\(\s*\{[\s\S]*?chatId:\s*hydratedUser\.chatId[\s\S]*?\}\s*\)/u,
  'Sidebar must copy the server-provided chatId into the current user passed to ProfileMenuModal',
);

assert.match(
  sidebarSource,
  /const\s+openProfileMenu\s*=\s*useCallback\(\s*async\s*\(\)\s*=>\s*\{[\s\S]*?await\s+refreshCurrentUser\(\)[\s\S]*?setShowProfileMenu\(\s*true\s*\)/u,
  'Sidebar must refresh the real social profile before opening ProfileMenuModal so the header does not render an unavailable Chat ID',
);

assert.doesNotMatch(
  sidebarSource,
  /Chat ID is not ready|profile\.toast\.chatIdNotReady|if\s*\(\s*!\s*refreshedUser\.chatId\s*\)[\s\S]*?return\s*;/u,
  'Sidebar avatar click must not block the profile menu with a Chat ID not-ready toast; ProfileMenuModal owns the localized copy-disabled state',
);

assert.doesNotMatch(
  profileMenuSource,
  />\s*(?:Loading|loading|Unavailable|Unavaiable)\s*</u,
  'Profile surfaces must not render Loading, loading, or Unavailable as a user-facing Chat ID fallback',
);

assert.doesNotMatch(
  contactDetailSource,
  /displayUserChatId\s*(?:\|\||\?\?)\s*["'](?:Loading|loading|Unavailable|Unavaiable)["']/u,
  'ContactDetailPane must not render Loading, loading, or Unavailable as a user-facing Chat ID fallback',
);

console.log('sdkwork-im-pc profile menu user id contract passed');
