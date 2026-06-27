import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const orgContainerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/OrgContainer.tsx');
const contactDetailSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/ContactDetailPane.tsx');
const favoritesViewSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/pages/FavoritesView.tsx');
const tagsContainerSource = read('apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/components/contacts/TagsContainer.tsx');

assert.match(
  orgContainerSource,
  /const\s+loadRoot\s*=\s*async\s*\(\)\s*=>\s*\{[\s\S]*?try\s*\{[\s\S]*?await\s+organizationDirectoryService\.getOrganizationDirectoryTree\(\)/u,
  'OrgContainer root organization-directory loading must handle backend failures instead of leaving loading stuck',
);
assert.match(
  orgContainerSource,
  /const\s+loadRoot\s*=\s*async\s*\(\)\s*=>\s*\{[\s\S]*?catch\s*\{[\s\S]*?toast\(t\(['"]contacts\.organizationDirectory\.toast\.loadTreeFailed['"]\),\s*['"]error['"]\)/u,
  'OrgContainer root organization-directory loading must surface backend failures',
);
assert.match(
  orgContainerSource,
  /const\s+loadRoot\s*=\s*async\s*\(\)\s*=>\s*\{[\s\S]*?finally\s*\{[\s\S]*?setLoading\(false\)/u,
  'OrgContainer root organization loading must always clear loading',
);
assert.match(
  orgContainerSource,
  /void\s+loadRoot\(\)/u,
  'OrgContainer root loading effect must explicitly fire-and-forget the handled async loader',
);
assert.match(
  orgContainerSource,
  /const\s+handleNavigate\s*=\s*\(\s*node:[\s\S]*?\)\s*=>\s*\{[\s\S]*?void\s+selectDirectoryNode\(node\)/u,
  'OrgContainer tree navigation must not create unhandled promises',
);

assert.match(
  contactDetailSource,
  /contactService\.getStarredContacts\(\)[\s\S]*?\.catch\s*\(/u,
  'ContactDetailPane starred status hydration must handle backend failures',
);
assert.match(
  contactDetailSource,
  /await\s+contactService\.setContactRemark\(user\.id,\s*name\.trim\(\)\)[\s\S]*?catch\s*\{/u,
  'ContactDetailPane remark update must surface SDK failures',
);

assert.match(
  favoritesViewSource,
  /try\s*\{[\s\S]*?await\s+favoriteService\.removeFavorite\(item\.id\)/u,
  'FavoritesView remove action must await the SDK-backed favorite deletion before local state changes',
);
assert.match(
  favoritesViewSource,
  /catch\s*\{[\s\S]*?toast\(t\(['"]favorites\.toast\.removeFailed['"]\),\s*['"]error['"]\)/u,
  'FavoritesView remove action must surface SDK deletion failures',
);

assert.match(
  tagsContainerSource,
  /contactService\.getTags\(\)[\s\S]*?\.catch\s*\(/u,
  'TagsContainer tag loading must handle backend failures instead of leaving loading stuck',
);
assert.match(
  tagsContainerSource,
  /contactService\.getTags\(\)[\s\S]*?\.finally\s*\(\s*\(\)\s*=>\s*setLoading\(false\)\s*\)/u,
  'TagsContainer tag loading must always clear loading after success or failure',
);
assert.match(
  tagsContainerSource,
  /await\s+contactService\.addTag[\s\S]*?catch\s*\{/u,
  'TagsContainer add-tag flow must surface SDK failures',
);
assert.match(
  tagsContainerSource,
  /await\s+contactService\.updateTag[\s\S]*?catch\s*\{/u,
  'TagsContainer update-tag flows must surface SDK failures',
);
assert.match(
  tagsContainerSource,
  /await\s+contactService\.removeTag\(tag\.id\)[\s\S]*?catch\s*\{/u,
  'TagsContainer remove-tag flow must surface SDK failures',
);

console.log('sdkwork-im-pc contact and favorites UI failure handling contract passed');
