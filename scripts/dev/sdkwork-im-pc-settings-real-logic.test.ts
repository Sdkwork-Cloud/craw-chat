import assert from 'node:assert/strict';
import type { SdkworkAiotAppClient, SdkworkImAppClient } from '@sdkwork/im-pc-core';
import {
  createSdkworkSettingsService,
  DEFAULT_SIDEBAR_MODULES,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-chat/src/services/SettingsService';

const calls: string[] = [];

const fakeClient = {
  portal: {
    home: {
      async retrieve() {
        calls.push('portal.home.retrieve');
        return {
          enabledModules: ['chat', 'contacts', 'agent', 'enterprise'],
        };
      },
    },
  },
} as unknown as SdkworkImAppClient;

const fakeAiotClient = {
  iot: {
    devices: {
      twin: {
        async retrieve(deviceId: string) {
          calls.push(`iot.devices.twin.retrieve:${deviceId}`);
          return {
            code: 'ok',
            data: {
              desired: {
                loginDevices: [
                  {
                    id: 'd_pad',
                    name: 'iPad Pro',
                    time: '2026-06-04 09:30 active',
                  },
                ],
              },
              reported: {
                currentDeviceName: 'Windows PC',
                loginDevices: [
                  {
                    id: 'd_phone',
                    name: 'iPhone 15 Pro',
                    time: '2026-06-04 08:10 active',
                  },
                ],
              },
            },
          };
        },
      },
      commands: {
        async create(deviceId: string, body: Record<string, unknown>) {
          calls.push(`iot.devices.commands.create:${deviceId}`);
          assert.deepEqual(body, {
            capabilityName: 'login-sessions',
            commandName: 'disable-login-device',
            payload: {
              disabledLoginDeviceIds: ['d_phone'],
            },
          });
          return {
            code: 'ok',
            data: {
              commandId: 'command-disable-d-phone',
            },
          };
        },
      },
    },
  },
} as unknown as SdkworkAiotAppClient;

function installLocalStorage(initial: Record<string, string>): void {
  const store = new Map(Object.entries(initial));
  Object.defineProperty(globalThis, 'localStorage', {
    configurable: true,
    value: {
      getItem(key: string) {
        return store.get(key) ?? null;
      },
      setItem(key: string, value: string) {
        store.set(key, value);
      },
      removeItem(key: string) {
        store.delete(key);
      },
      clear() {
        store.clear();
      },
    },
  });
}

async function main(): Promise<void> {
  installLocalStorage({});
  const defaultService = createSdkworkSettingsService(() => fakeClient, () => fakeAiotClient, () => 'c_pc');
  assert.deepEqual(
    (await defaultService.getSettings()).sidebarModules,
    DEFAULT_SIDEBAR_MODULES,
    'new installations must only show the product-ready sidebar modules by default',
  );

  installLocalStorage({
    'im-settings': JSON.stringify({
      sidebarModules: [
        'chat',
        'workspace',
        'orders',
        'shop',
        'calendar',
        'notary',
        'knowledge',
        'enterprise',
        'devices',
        'community',
        'voice',
        'agent',
        'course',
        'contacts',
        'favorites',
      ],
    }),
  });
  const migratedService = createSdkworkSettingsService(() => fakeClient, () => fakeAiotClient, () => 'c_pc');
  assert.deepEqual(
    (await migratedService.getSettings()).sidebarModules,
    DEFAULT_SIDEBAR_MODULES,
    'legacy all-module settings must migrate to the product-ready default sidebar modules',
  );

  installLocalStorage({});
  const service = createSdkworkSettingsService(() => fakeClient, () => fakeAiotClient, () => 'c_pc');

  const serverModules = await service.getServerModules();
  assert.deepEqual(serverModules, ['chat', 'contacts', 'agent', 'enterprise']);

  const devices = await service.getDevices();
  assert.deepEqual(devices, [
    {
      id: 'd_phone',
      name: 'iPhone 15 Pro',
      time: '2026-06-04 08:10 active',
    },
    {
      id: 'd_pad',
      name: 'iPad Pro',
      time: '2026-06-04 09:30 active',
    },
  ]);

  await service.removeDevice('d_phone');

  assert.deepEqual(calls, [
    'portal.home.retrieve',
    'iot.devices.twin.retrieve:c_pc',
    'iot.devices.commands.create:c_pc',
  ]);

  console.log('sdkwork-im-pc settings real-logic contract passed');
}

void main();
