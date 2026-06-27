import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  createDeviceService,
} from '../../apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/services/DeviceService';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath: string): string {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath: string): Record<string, unknown> {
  return JSON.parse(read(relativePath)) as Record<string, unknown>;
}

interface FakeAiotDevice {
  chipFamily?: string;
  deviceId: string;
  displayName: string;
  id: string;
  metadata: Record<string, unknown>;
  status: string;
}

async function main(): Promise<void> {
  const packageJson = readJson('apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/package.json');
  const dependencies = packageJson.dependencies as Record<string, string>;
  const deviceServiceSource = read(
    'apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/services/DeviceService.ts',
  );
  const bindAgentModalSource = read(
    'apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/components/BindAgentModal.tsx',
  );
  const deviceDetailPanelSource = read(
    'apps/sdkwork-im-pc/packages/sdkwork-im-pc-devices/src/components/DeviceDetailPanel.tsx',
  );
  assert.equal(
    dependencies['@sdkwork/aiot-backend-sdk'],
    undefined,
    'non-admin PC devices package must not depend on the AIoT backend SDK',
  );
  assert.doesNotMatch(
    deviceServiceSource,
    /@sdkwork\/aiot-backend-sdk|backendClient|BackendClient|getBackendClient/u,
    'non-admin PC devices service must not import, configure, or call backend SDK clients',
  );
  assert.match(
    bindAgentModalSource,
    /deviceService\.bindAgent\s*\(/u,
    'agent binding modal must persist selected agent through the SDK-backed device service',
  );
  assert.doesNotMatch(
    bindAgentModalSource,
    /toast\s*\([^)]*Agent[^)]*success|toast\s*\([^)]*Agent[^)]*鎴愬姛/u,
    'agent binding modal must not show a local fake success toast before the service call resolves',
  );
  assert.match(
    deviceDetailPanelSource,
    /deviceService\.unbindAgent\s*\(/u,
    'device detail panel must persist agent unbinding through the SDK-backed device service',
  );

  const devices = new Map<string, FakeAiotDevice>([
    [
      'dev-001',
      {
        chipFamily: 'camera',
        deviceId: 'dev-001',
        displayName: 'Door Camera',
        id: 'dev-001',
        metadata: {
          agentId: 'agent.device.guard',
          firmwareVersion: '1.0.0',
          macAddress: '00:11:22:33:44:55',
        },
        status: 'online',
      },
    ],
    [
      'dev-002',
      {
        chipFamily: 'display',
        deviceId: 'dev-002',
        displayName: 'Lobby Display',
        id: 'dev-002',
        metadata: {
          firmwareVersion: '1.1.0',
          macAddress: '00:11:22:33:44:66',
        },
        status: 'offline',
      },
    ],
  ]);
  const commandCalls: Array<{
    body: Record<string, unknown>;
    deviceId: string;
    idempotencyKey?: string;
  }> = [];
  const appClient = {
    iot: {
      devices: {
        list: async () => ({ data: Array.from(devices.values()) }),
        retrieve: async (id: string) => ({ data: devices.get(id) }),
        commands: {
          create: async (
            deviceId: string,
            body: Record<string, unknown>,
            params: { idempotencyKey?: string },
          ) => {
            commandCalls.push({ body, deviceId, idempotencyKey: params.idempotencyKey });
            const device = devices.get(deviceId);
            const payload = body.payload as Record<string, unknown>;
            if (device) {
              if (body.capabilityName === 'agent-hosting' && body.commandName === 'bind-agent') {
                device.metadata.agentId = payload.agentId;
              }
              if (body.capabilityName === 'agent-hosting' && body.commandName === 'unbind-agent') {
                delete device.metadata.agentId;
              }
              if (body.capabilityName === 'device-activation' && body.commandName === 'activate') {
                device.status = 'offline';
              }
            }
            return {
              data: {
                commandId: `cmd-${body.commandName as string}`,
              },
            };
          },
        },
      },
    },
  };
  const deviceService = createDeviceService({
    client: appClient,
  });

  const initialDevices = await deviceService.getDevices();
  assert.equal(
    initialDevices
      .map((device) => device.agentId)
      .filter((agentId): agentId is string => Boolean(agentId))
      .every((agentId) => /^agent\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u.test(agentId)),
    true,
    'device agent bindings must use standard agent. ids in local product data',
  );

  await deviceService.bindAgent('dev-002', 'agent.device.concierge');
  assert.deepEqual(
    commandCalls[0],
    {
      body: {
        capabilityName: 'agent-hosting',
        commandName: 'bind-agent',
        payload: {
          agentId: 'agent.device.concierge',
        },
      },
      deviceId: 'dev-002',
      idempotencyKey: 'bind-agent:dev-002:agent.device.concierge',
    },
    'device agent binding must submit a real AIoT app SDK command instead of using backend SDK twin mutation',
  );
  const boundDevice = await deviceService.getDevice('dev-002');
  assert.equal(
    boundDevice?.agentId,
    'agent.device.concierge',
    'device agent binding must store standard agent. ids',
  );

  await assert.rejects(
    () => deviceService.bindAgent('dev-002', 'agent-legacy'),
    /Device agent binding id must use the standard agent\./,
    'device agent binding must reject legacy agent-* ids',
  );
  const afterInvalidBind = await deviceService.getDevice('dev-002');
  assert.equal(
    afterInvalidBind?.agentId,
    'agent.device.concierge',
    'invalid legacy agent ids must not overwrite the device binding',
  );
  assert.equal(commandCalls.length, 1, 'invalid legacy agent ids must not submit device commands');

  await deviceService.unbindAgent('dev-002');
  assert.deepEqual(
    commandCalls[1],
    {
      body: {
        capabilityName: 'agent-hosting',
        commandName: 'unbind-agent',
        payload: {},
      },
      deviceId: 'dev-002',
      idempotencyKey: 'unbind-agent:dev-002',
    },
    'device agent unbinding must submit a real AIoT app SDK command',
  );
  assert.equal((await deviceService.getDevice('dev-002'))?.agentId, undefined);

  await deviceService.activateDevice('dev-002', 'ACT-123456');
  assert.deepEqual(
    commandCalls[2],
    {
      body: {
        capabilityName: 'device-activation',
        commandName: 'activate',
        payload: {
          activationCode: 'ACT-123456',
        },
      },
      deviceId: 'dev-002',
      idempotencyKey: 'activate-device:dev-002',
    },
    'device activation must submit a real AIoT app SDK command',
  );

  await assert.rejects(
    () => deviceService.addDevice({
      firmwareVersion: '1.0.0',
      macAddress: '00:11:22:33:44:77',
      name: 'New Camera',
      status: 'unactivated',
      type: 'camera',
    }),
    /AIoT app SDK does not expose device creation/,
    'non-admin PC devices service must fail closed when app SDK lacks device creation',
  );
  await assert.rejects(
    () => deviceService.deleteDevice('dev-002'),
    /AIoT app SDK does not expose device deletion/,
    'non-admin PC devices service must fail closed when app SDK lacks device deletion',
  );

  console.log('sdkwork-im-pc device agent binding standard contract passed');
}

void main();
