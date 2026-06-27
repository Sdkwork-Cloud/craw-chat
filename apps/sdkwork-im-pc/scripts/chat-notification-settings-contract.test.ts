import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import type * as SettingsServiceModule from '../packages/sdkwork-im-pc-chat/src/services/SettingsService.ts';

type SettingsServiceExports = typeof SettingsServiceModule;

class MemoryStorage implements Storage {
  private readonly entries = new Map<string, string>();

  get length(): number {
    return this.entries.size;
  }

  clear(): void {
    this.entries.clear();
  }

  getItem(key: string): string | null {
    return this.entries.get(key) ?? null;
  }

  key(index: number): string | null {
    return Array.from(this.entries.keys())[index] ?? null;
  }

  removeItem(key: string): void {
    this.entries.delete(key);
  }

  setItem(key: string, value: string): void {
    this.entries.set(key, value);
  }
}

Object.defineProperty(globalThis, 'localStorage', {
  configurable: true,
  value: new MemoryStorage(),
});

async function loadSettingsServiceModule(): Promise<SettingsServiceExports> {
  const moduleUrl = pathToFileURL(
    './packages/sdkwork-im-pc-chat/src/services/SettingsService.ts',
  ).href;
  return await import(moduleUrl) as SettingsServiceExports;
}

const { createSdkworkSettingsService } = await loadSettingsServiceModule();
const service = createSdkworkSettingsService(
  () => ({
    portal: {
      home: {
        async retrieve() {
          return {};
        },
      },
    },
  } as never),
  () => ({
    iot: {
      devices: {
        commands: {
          async create() {
            return {};
          },
        },
        twin: {
          async retrieve() {
            return {};
          },
        },
      },
    },
  } as never),
  () => 'client.contract',
);

const defaults = await service.getSettings();

assert.equal(defaults.notifyDesktop, true);
assert.equal(defaults.notifySound, true);
assert.equal(defaults.notifySystem, false);
assert.equal(defaults.notificationPreview, 'sender-and-preview');
assert.equal(defaults.notificationWhenFocused, false);

const updated = await service.updateSettings({
  notificationPreview: 'hidden',
  notificationWhenFocused: true,
  notifySystem: true,
});

assert.equal(updated.notificationPreview, 'hidden');
assert.equal(updated.notificationWhenFocused, true);
assert.equal(updated.notifySystem, true);

const invalid = await service.updateSettings({
  notificationPreview: 'invalid-preview-mode' as never,
});

assert.equal(
  invalid.notificationPreview,
  'sender-and-preview',
  'Invalid notification preview settings should normalize to the professional default.',
);

console.log('sdkwork im pc notification settings contract passed.');
