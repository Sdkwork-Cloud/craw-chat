/**
 * H5 host adapter registration.
 *
 * Browser web mode supplies fallback adapters for unsupported native
 * capabilities. Capacitor targets register native host adapters through
 * `sdkwork-im-h5-capacitor`. Feature packages depend on adapter interfaces,
 * not Capacitor globals, browser globals, or plugin imports.
 */

export type H5HostAdapterId =
  | 'camera'
  | 'qrScanner'
  | 'pushNotifications'
  | 'deepLinks'
  | 'secureStorage'
  | 'biometric'
  | 'shareSheet'
  | 'networkStatus'
  | 'appLifecycle'
  | 'clipboard'
  | 'filePicker'
  | 'filesystemSandbox'
  | 'geolocation'
  | 'deviceInfo'
  | 'haptics'
  | 'contactsPicker'
  | 'paymentHost'
  | 'browserOpen';

export type H5HostAdapterStatus =
  | 'supported'
  | 'unsupported'
  | 'permission-denied'
  | 'unavailable'
  | 'cancelled'
  | 'invalid-state'
  | 'timeout';

export interface H5HostAdapter {
  readonly id: H5HostAdapterId;
  readonly status: H5HostAdapterStatus;
  invoke?(...args: unknown[]): Promise<unknown>;
}

const unsupportedAdapterIds: ReadonlySet<H5HostAdapterId> = new Set([
  'biometric',
  'pushNotifications',
  'secureStorage',
  'filesystemSandbox',
]);

function createBrowserFallbackAdapter(id: H5HostAdapterId): H5HostAdapter {
  return {
    id,
    status: unsupportedAdapterIds.has(id) ? 'unsupported' : 'supported',
  };
}

const registeredAdapters = new Map<H5HostAdapterId, H5HostAdapter>();

export function registerHostAdapter(adapter: H5HostAdapter): void {
  registeredAdapters.set(adapter.id, adapter);
}

export function getHostAdapter(id: H5HostAdapterId): H5HostAdapter {
  const existing = registeredAdapters.get(id);
  if (existing) {
    return existing;
  }
  const fallback = createBrowserFallbackAdapter(id);
  registerHostAdapter(fallback);
  return fallback;
}

export function listRegisteredHostAdapters(): H5HostAdapter[] {
  return Array.from(registeredAdapters.values());
}

export function resetHostAdapters(): void {
  registeredAdapters.clear();
}

export function isHostAdapterSupported(id: H5HostAdapterId): boolean {
  return getHostAdapter(id).status === 'supported';
}
