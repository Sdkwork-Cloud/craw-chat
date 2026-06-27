import assert from 'node:assert/strict';
import { resolveSdkworkChatPcClientId } from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/ClientIdentityService';

const CLIENT_ID_STORAGE_KEY = 'sdkwork-im-pc:client-id:v1';
const LEGACY_IM_DEVICE_ID_STORAGE_KEY = 'sdkwork-im-pc:im-device-id:v1';

class MemoryStorage implements Storage {
  private readonly data = new Map<string, string>();

  get length(): number {
    return this.data.size;
  }

  clear(): void {
    this.data.clear();
  }

  getItem(key: string): string | null {
    return this.data.get(key) ?? null;
  }

  key(index: number): string | null {
    return Array.from(this.data.keys())[index] ?? null;
  }

  removeItem(key: string): void {
    this.data.delete(key);
  }

  setItem(key: string, value: string): void {
    this.data.set(key, String(value));
  }
}

const localStorage = new MemoryStorage();
Object.defineProperty(globalThis, 'window', {
  configurable: true,
  value: { localStorage },
});

function assertClientIdIsLocal(value: string): void {
  assert.match(
    value,
    /^[cd]_[a-z0-9]+_[a-z0-9]+$/u,
    'local client identity must be compact and path-safe',
  );
  assert.doesNotMatch(
    value,
    /sdkwork-im-pc/u,
    'local client identity must not leak the app/project directory name',
  );
}

async function main(): Promise<void> {
  localStorage.clear();

  const generatedClientId = resolveSdkworkChatPcClientId();
  assertClientIdIsLocal(generatedClientId);
  assert.equal(localStorage.getItem(CLIENT_ID_STORAGE_KEY), generatedClientId);

  localStorage.clear();
  localStorage.setItem(LEGACY_IM_DEVICE_ID_STORAGE_KEY, 'd_mpz9hezz_t1rjq2g8');
  const migratedClientId = resolveSdkworkChatPcClientId();
  assert.equal(migratedClientId, 'd_mpz9hezz_t1rjq2g8');
  assert.equal(localStorage.getItem(CLIENT_ID_STORAGE_KEY), migratedClientId);
  assert.equal(localStorage.getItem(LEGACY_IM_DEVICE_ID_STORAGE_KEY), null);

  localStorage.clear();
  localStorage.setItem(LEGACY_IM_DEVICE_ID_STORAGE_KEY, 'sdkwork-im-pc-1780523380759-zhyejitm');
  const regeneratedClientId = resolveSdkworkChatPcClientId();
  assertClientIdIsLocal(regeneratedClientId);
  assert.notEqual(regeneratedClientId, 'sdkwork-im-pc-1780523380759-zhyejitm');
  assert.equal(localStorage.getItem(CLIENT_ID_STORAGE_KEY), regeneratedClientId);

  console.log('sdkwork-im-pc client identity contract passed');
}

void main();
