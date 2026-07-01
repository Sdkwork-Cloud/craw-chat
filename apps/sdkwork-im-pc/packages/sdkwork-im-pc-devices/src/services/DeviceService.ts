import type {
  AiotCommandCreateRequest,
  AiotDevice,
  SdkworkAiotAppClient,
} from '@sdkwork/aiot-app-sdk';
import {
  getAiotAppSdkClientWithSession,
} from '@sdkwork/im-pc-core/sdk/aiotAppSdkClient';

export type DeviceType = 'camera' | 'speaker' | 'display' | 'sensor' | 'other';
export type DeviceStatus = 'online' | 'offline' | 'error' | 'unactivated';

export interface Device {
  id: string;
  name: string;
  type: DeviceType;
  status: DeviceStatus;
  agentId?: string;
  macAddress: string;
  firmwareVersion: string;
}

export interface DeviceService {
  getDevices(): Promise<Device[]>;
  getDevice(id: string): Promise<Device | undefined>;
  addDevice(device: Omit<Device, 'id'>): Promise<Device>;
  updateDevice(id: string, device: Partial<Device>): Promise<void>;
  deleteDevice(id: string): Promise<void>;
  bindAgent(deviceId: string, agentId: string): Promise<void>;
  unbindAgent(deviceId: string): Promise<void>;
  activateDevice(deviceId: string, activationCode: string): Promise<void>;
}

export interface AiotDeviceServiceOptions {
  client?: SdkworkAiotAppClient;
}

let configuredClient: SdkworkAiotAppClient | undefined;

function readRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function readString(value: unknown, fallback = ''): string {
  if (typeof value === 'string') {
    return value;
  }
  if (typeof value === 'number' || typeof value === 'boolean') {
    return String(value);
  }
  return fallback;
}

function assertStandardAgentId(agentId: string): void {
  if (!STANDARD_AGENT_ID_PATTERN.test(agentId)) {
    throw new Error('Device agent binding id must use the standard agent. prefix.');
  }
}

function assertDeviceAgentMetadata(device: Partial<Device>): void {
  if (device.agentId !== undefined) {
    assertStandardAgentId(device.agentId);
  }
}

function normalizeDeviceType(value: unknown, fallback: unknown): DeviceType {
  const type = readString(value, readString(fallback)).trim().toLowerCase();
  if (type === 'camera' || type === 'speaker' || type === 'display' || type === 'sensor') {
    return type;
  }
  return 'other';
}

function normalizeDeviceStatus(value: unknown): DeviceStatus {
  const status = readString(value).trim().toLowerCase();
  if (status === 'online') {
    return 'online';
  }
  if (status === 'error' || status === 'warning' || status === 'degraded' || status === 'alarm') {
    return 'error';
  }
  if (status === 'unactivated' || status === 'inactive' || status === 'pending_activation') {
    return 'unactivated';
  }
  return 'offline';
}

function resolveDeviceId(device: AiotDevice): string {
  return device.deviceId || device.id;
}

function mapAiotDevice(device: AiotDevice): Device {
  const metadata = readRecord(device.metadata);
  const deviceId = resolveDeviceId(device);
  return {
    id: deviceId,
    name: device.displayName || deviceId,
    type: normalizeDeviceType(metadata.type, device.chipFamily),
    status: normalizeDeviceStatus(device.status),
    ...(readString(metadata.agentId) ? { agentId: readString(metadata.agentId) } : {}),
    macAddress: readString(metadata.macAddress, readString(metadata.mac)),
    firmwareVersion: readString(metadata.firmwareVersion),
  };
}

const STANDARD_AGENT_ID_PATTERN = /^agent\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u;

function getClient(override?: SdkworkAiotAppClient): SdkworkAiotAppClient {
  return override ?? configuredClient ?? getAiotAppSdkClientWithSession();
}

async function submitDeviceCommand(
  client: SdkworkAiotAppClient,
  deviceId: string,
  body: AiotCommandCreateRequest,
  idempotencyKey: string,
): Promise<void> {
  await client.iot.devices.commands.create(deviceId, body, { idempotencyKey });
}

function unsupportedAppDeviceManagementCapability(capability: string): Error {
  return new Error(
    `AIoT app SDK does not expose device ${capability}. Use a backend-admin device management surface or extend sdkwork-aiot app API before enabling this user-facing workflow.`,
  );
}

class AiotDeviceService implements DeviceService {
  constructor(private readonly options: AiotDeviceServiceOptions = {}) {}

  async getDevices(): Promise<Device[]> {
    const page = await getClient(this.options.client).iot.devices.list();
    return page.items.map((item) => mapAiotDevice(item as unknown as AiotDevice));
  }

  async getDevice(id: string): Promise<Device | undefined> {
    try {
      const device = await getClient(this.options.client).iot.devices.retrieve(id);
      return mapAiotDevice(device);
    } catch {
      return undefined;
    }
  }

  async addDevice(device: Omit<Device, 'id'>): Promise<Device> {
    assertDeviceAgentMetadata(device);
    throw unsupportedAppDeviceManagementCapability('creation');
  }

  async updateDevice(id: string, device: Partial<Device>): Promise<void> {
    if (!id.trim()) {
      throw new Error('Device id is required.');
    }
    assertDeviceAgentMetadata(device);
    throw unsupportedAppDeviceManagementCapability('update');
  }

  async deleteDevice(id: string): Promise<void> {
    if (!id.trim()) {
      throw new Error('Device id is required.');
    }
    throw unsupportedAppDeviceManagementCapability('deletion');
  }

  async bindAgent(deviceId: string, agentId: string): Promise<void> {
    assertStandardAgentId(agentId);
    await submitDeviceCommand(
      getClient(this.options.client),
      deviceId,
      {
        capabilityName: 'agent-hosting',
        commandName: 'bind-agent',
        payload: { agentId },
      },
      `bind-agent:${deviceId}:${agentId}`,
    );
  }

  async unbindAgent(deviceId: string): Promise<void> {
    await submitDeviceCommand(
      getClient(this.options.client),
      deviceId,
      {
        capabilityName: 'agent-hosting',
        commandName: 'unbind-agent',
        payload: {},
      },
      `unbind-agent:${deviceId}`,
    );
  }

  async activateDevice(deviceId: string, activationCode: string): Promise<void> {
    const normalizedActivationCode = activationCode.trim();
    if (!normalizedActivationCode) {
      throw new Error('Activation code is required.');
    }
    await submitDeviceCommand(
      getClient(this.options.client),
      deviceId,
      {
        capabilityName: 'device-activation',
        commandName: 'activate',
        payload: {
          activationCode: normalizedActivationCode,
        },
      },
      `activate-device:${deviceId}`,
    );
  }
}

export function configureDeviceService(options: AiotDeviceServiceOptions = {}): void {
  configuredClient = options.client ?? configuredClient;
}

export function createDeviceService(options: AiotDeviceServiceOptions = {}): DeviceService {
  return new AiotDeviceService(options);
}

export const deviceService: DeviceService = createDeviceService();
