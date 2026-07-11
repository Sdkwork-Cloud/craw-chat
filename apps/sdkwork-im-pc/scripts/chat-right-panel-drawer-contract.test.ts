import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

const chatRightPanelSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/ChatRightPanel.tsx',
  'utf8',
);

const chatLayoutSource = readFileSync(
  './packages/sdkwork-im-pc-chat/src/pages/ChatLayout.tsx',
  'utf8',
);

function readJson(relativePath: string) {
  return JSON.parse(readFileSync(relativePath, 'utf8')) as Record<string, unknown>;
}

function mergeJson(relativePaths: string[]) {
  return Object.assign({}, ...relativePaths.map(readJson));
}

function readChatLocale(locale: string) {
  return mergeJson([
    `./packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/sidebar.json`,
    `./packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/agent.json`,
    `./packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/profile.json`,
    `./packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/contacts.json`,
    `./packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/favorites.json`,
    `./packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/settings-modal.json`,
    `./packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/chat.json`,
    `./packages/sdkwork-im-pc-chat/src/i18n/${locale}/communication/im-pc-chat/scan-qr.json`,
  ]);
}

const zhLocale = readChatLocale('zh-CN') as { chat?: { rightPanel?: { actions?: Record<string, string> } } };

const enLocale = readChatLocale('en-US') as { chat?: { rightPanel?: { actions?: Record<string, string> } } };

assert.match(
  chatRightPanelSource,
  /onClose:\s*\(\)\s*=>\s*void/u,
  'ChatRightPanel must accept an explicit onClose callback for the drawer close button.',
);

assert.match(
  chatRightPanelSource,
  /sticky\s+top-0/u,
  'ChatRightPanel must keep a sticky drawer header at the top so profile content is not hidden under surrounding app chrome.',
);

assert.match(
  chatRightPanelSource,
  /aria-label=\{t\(['"]chat\.rightPanel\.actions\.close['"]\)\}/u,
  'ChatRightPanel close button must expose a localized accessible name.',
);

assert.match(
  chatRightPanelSource,
  /<X\b[\s\S]*size=\{18\}/u,
  'ChatRightPanel header must render a right-aligned X close icon.',
);

assert.match(
  chatLayoutSource,
  /onClose=\{\(\)\s*=>\s*setShowRHSPanel\(false\)\}/u,
  'ChatLayout must wire the right-panel drawer close button to hide the drawer.',
);

for (const [localeName, locale] of [['zh-CN', zhLocale], ['en-US', enLocale]] as const) {
  assert.equal(
    typeof locale.chat?.rightPanel?.actions?.close,
    'string',
    `${localeName} must define chat.rightPanel.actions.close for the drawer close button.`,
  );
}

console.log('sdkwork im pc right panel drawer contract passed.');
