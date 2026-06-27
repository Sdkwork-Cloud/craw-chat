import React from 'react';

export const CHAT_MODULE_ID = 'chat';

/** Tabs that render full-bleed capability views without the unified app header bar. */
export const FULLSCREEN_MODULE_TABS = new Set<string>([
  'orders',
  'notary',
  'workspace',
  'calendar',
  'shop',
  'drive',
  'approval',
  'report',
  'attendance',
  'knowledge',
  'create-agent',
  'course',
  'enterprise',
  'voice',
  'videogen',
  'imagegen',
  'voicegen',
  'musicgen',
  'writing',
]);

export function isChatModule(activeTab: string): boolean {
  return activeTab === CHAT_MODULE_ID;
}

export function isFullscreenModule(activeTab: string): boolean {
  return FULLSCREEN_MODULE_TABS.has(activeTab);
}
