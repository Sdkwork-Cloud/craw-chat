const SDKWORK_IM_PC_CLIENT_ID_KEY = 'sdkwork-im-pc:client-id:v1';
const SDKWORK_IM_PC_LEGACY_IM_DEVICE_ID_KEY = 'sdkwork-im-pc:im-device-id:v1';
const LOCAL_CLIENT_ID_PATTERN = /^[cd]_[a-z0-9]+_[a-z0-9]+$/u;

function getStorage(): Storage | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }
  return window.localStorage;
}

function createClientId(): string {
  const timestamp = Date.now().toString(36);
  const entropy = Math.random().toString(36).slice(2, 10).padEnd(8, '0');
  return `c_${timestamp}_${entropy}`;
}

function isLocalClientId(value: string): boolean {
  return LOCAL_CLIENT_ID_PATTERN.test(value.trim());
}

export function resolveSdkworkChatPcClientId(): string {
  const storage = getStorage();
  if (!storage) {
    return createClientId();
  }

  const existing = storage.getItem(SDKWORK_IM_PC_CLIENT_ID_KEY);
  if (existing && isLocalClientId(existing)) {
    return existing.trim();
  }

  const legacy = storage.getItem(SDKWORK_IM_PC_LEGACY_IM_DEVICE_ID_KEY);
  if (legacy && isLocalClientId(legacy)) {
    const migratedClientId = legacy.trim();
    storage.setItem(SDKWORK_IM_PC_CLIENT_ID_KEY, migratedClientId);
    storage.removeItem(SDKWORK_IM_PC_LEGACY_IM_DEVICE_ID_KEY);
    return migratedClientId;
  }

  const clientId = createClientId();
  storage.setItem(SDKWORK_IM_PC_CLIENT_ID_KEY, clientId);
  return clientId;
}
