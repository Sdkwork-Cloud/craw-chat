import type {
  RtcClient,
  RtcDataSource,
  RtcDriverManager,
  RtcTrackKind,
} from '@sdkwork/rtc-sdk';

export interface SdkworkRtcParticipantCredential {
  credential: string;
  expiresAt: string;
  participantId: string;
  rtcSessionId: string;
  tenantId: string;
}

export interface SdkworkRtcMediaJoinOptions {
  accessEndpoint?: string;
  credential: SdkworkRtcParticipantCredential;
  metadata?: Record<string, unknown>;
  participantId: string;
  providerKey?: string;
  providerRegion?: string;
  roomId: string;
  rtcMode?: string;
  rtcSessionId: string;
}

export interface SdkworkRtcMediaPublishOptions {
  kinds: readonly Extract<RtcTrackKind, 'audio' | 'video'>[];
  rtcSessionId: string;
}

export interface SdkworkRtcMediaService {
  bindLocalVideoElement(element: HTMLElement | null): Promise<void>;
  join(options: SdkworkRtcMediaJoinOptions): Promise<void>;
  leave(): Promise<void>;
  muteAudio(muted: boolean): Promise<void>;
  muteVideo(muted: boolean): Promise<void>;
  publish(options: SdkworkRtcMediaPublishOptions): Promise<void>;
}

interface SdkworkRtcMediaServiceDependencies {
  createDataSource?: (options: SdkworkRtcMediaJoinOptions) => Promise<RtcDataSource> | RtcDataSource;
}

interface VolcengineLocalVideoEngine {
  play?(userId?: string, mediaType?: unknown, streamIndex?: number, playerId?: string): Promise<void>;
  setLocalVideoPlayer(
    streamIndex: number,
    options?: {
      playerId?: string;
      renderDom?: HTMLElement;
      renderMode?: number;
    },
  ): HTMLVideoElement | undefined;
  stop?(userId?: string, mediaType?: unknown, streamIndex?: number, playerId?: string): void;
}

interface VolcengineNativeClient {
  engine?: VolcengineLocalVideoEngine;
}

type VolcengineProviderNamespace = {
  VOLCENGINE_RTC_PROVIDER_MODULE?: unknown;
};

interface RuntimeImportMetaEnv {
  VITE_SDKWORK_RTC_VOLCENGINE_APP_ID?: string;
  VITE_SDKWORK_RTC_VOLCENGINE_AUDIO_DEVICE_ID?: string;
  VITE_SDKWORK_RTC_VOLCENGINE_ENGINE_ENV?: string;
  VITE_SDKWORK_RTC_VOLCENGINE_PROFILE?: string;
  VITE_SDKWORK_RTC_VOLCENGINE_VIDEO_DEVICE_ID?: string;
}

function readRuntimeImportMetaEnv(): RuntimeImportMetaEnv {
  return (import.meta.env ?? {}) as RuntimeImportMetaEnv;
}

function readEnvValue(key: keyof RuntimeImportMetaEnv): string | undefined {
  const value = readRuntimeImportMetaEnv()[key];
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function toProviderKey(value: string | undefined): string | undefined {
  if (!value) {
    return undefined;
  }
  return value.replace(/^rtc-/u, '').trim() || undefined;
}

function buildVolcengineNativeConfig(): Record<string, unknown> {
  const appId = readEnvValue('VITE_SDKWORK_RTC_VOLCENGINE_APP_ID');
  const engineEnv = readEnvValue('VITE_SDKWORK_RTC_VOLCENGINE_ENGINE_ENV');
  const profile = readEnvValue('VITE_SDKWORK_RTC_VOLCENGINE_PROFILE');
  const audioDeviceId = readEnvValue('VITE_SDKWORK_RTC_VOLCENGINE_AUDIO_DEVICE_ID');
  const videoDeviceId = readEnvValue('VITE_SDKWORK_RTC_VOLCENGINE_VIDEO_DEVICE_ID');
  return {
    ...(appId ? { appId } : {}),
    ...(engineEnv ? { engineConfig: { env: engineEnv } } : {}),
    roomConfig: {
      profile: profile ?? 'communication',
    },
    capture: {
      ...(audioDeviceId ? { audioDeviceId } : {}),
      ...(videoDeviceId ? { videoDeviceId } : {}),
    },
  };
}

async function createDefaultRtcDataSource(options: SdkworkRtcMediaJoinOptions): Promise<RtcDataSource> {
  const [
    { RtcDataSource: RtcDataSourceClass, RtcDriverManager, createRtcProviderPackageLoader, installRtcProviderPackage },
    volcengineProvider,
  ] = await Promise.all([
    import('@sdkwork/rtc-sdk'),
    import('@sdkwork/rtc-sdk-provider-volcengine') as Promise<VolcengineProviderNamespace>,
  ]);
  const providerKey = toProviderKey(options.providerKey) ?? 'volcengine';
  const driverManager = await installRtcProviderPackage(
    new RtcDriverManager(),
    { providerKey },
    createRtcProviderPackageLoader(async () => volcengineProvider),
  ) as RtcDriverManager;

  return new RtcDataSourceClass({
    defaultProviderKey: 'volcengine',
    driverManager,
    providerKey,
    nativeConfig: buildVolcengineNativeConfig(),
  });
}

function shouldPublishVideo(options: SdkworkRtcMediaJoinOptions): boolean {
  const rtcMode = options.rtcMode?.toLowerCase();
  return rtcMode === 'video' || rtcMode === 'video_call';
}

function createTrackId(rtcSessionId: string, kind: RtcTrackKind): string {
  return `${rtcSessionId}-${kind}`;
}

const LOCAL_VIDEO_PLAYER_ID = 'sdkwork-im-pc-local-video-preview';
const VOLCENGINE_MAIN_STREAM_INDEX = 0;
const VOLCENGINE_RENDER_MODE_HIDDEN = 0;
const VOLCENGINE_NATIVE_CLIENT_EXTENSION_KEY = 'volcengine.native-client';

export class SdkworkStandardRtcMediaService implements SdkworkRtcMediaService {
  private readonly createDataSource: (options: SdkworkRtcMediaJoinOptions) => Promise<RtcDataSource> | RtcDataSource;
  private client?: RtcClient;
  private joinedRtcSessionId?: string;
  private localVideoBound = false;
  private localVideoElement?: HTMLElement;
  private publishedTrackIds = new Set<string>();

  constructor(dependencies: SdkworkRtcMediaServiceDependencies = {}) {
    this.createDataSource = dependencies.createDataSource ?? createDefaultRtcDataSource;
  }

  async bindLocalVideoElement(element: HTMLElement | null): Promise<void> {
    this.localVideoElement = element ?? undefined;
    await this.syncLocalVideoBinding();
  }

  async join(options: SdkworkRtcMediaJoinOptions): Promise<void> {
    if (this.joinedRtcSessionId === options.rtcSessionId && this.client) {
      await this.syncLocalVideoBinding();
      return;
    }

    if (this.client) {
      await this.leave();
    }

    const dataSource = await this.createDataSource(options);
    const client = await dataSource.createClient({
      providerKey: toProviderKey(options.providerKey),
    });
    try {
      await client.join({
        sessionId: options.rtcSessionId,
        roomId: options.roomId,
        participantId: options.participantId,
        token: options.credential.credential,
        metadata: {
          ...(options.metadata ?? {}),
          ...(options.accessEndpoint ? { accessEndpoint: options.accessEndpoint } : {}),
          ...(options.providerRegion ? { providerRegion: options.providerRegion } : {}),
          ...(options.rtcMode ? { rtcMode: options.rtcMode } : {}),
        },
      });
    } catch (error) {
      await this.unbindLocalVideo(client);
      await client.leave().catch(() => undefined);
      throw error;
    }
    this.client = client;
    this.joinedRtcSessionId = options.rtcSessionId;
    await this.syncLocalVideoBinding();
  }

  async publish(options: SdkworkRtcMediaPublishOptions): Promise<void> {
    const client = this.requireClient();
    for (const kind of options.kinds) {
      const trackId = createTrackId(options.rtcSessionId, kind);
      if (this.publishedTrackIds.has(trackId)) {
        continue;
      }
      await client.publish({ trackId, kind });
      this.publishedTrackIds.add(trackId);
    }
  }

  async muteAudio(muted: boolean): Promise<void> {
    await this.client?.muteAudio(muted);
  }

  async muteVideo(muted: boolean): Promise<void> {
    await this.client?.muteVideo(muted);
  }

  async leave(): Promise<void> {
    const client = this.client;
    await this.unbindLocalVideo(client);
    this.client = undefined;
    this.joinedRtcSessionId = undefined;
    this.publishedTrackIds.clear();
    await client?.leave();
  }

  private getVolcengineLocalVideoEngine(client: RtcClient | undefined): VolcengineLocalVideoEngine | undefined {
    try {
      if (!client?.supportsProviderExtension(VOLCENGINE_NATIVE_CLIENT_EXTENSION_KEY)) {
        return undefined;
      }
      const nativeClient = client.unwrap() as VolcengineNativeClient;
      return nativeClient.engine;
    } catch {
      return undefined;
    }
  }

  private async syncLocalVideoBinding(): Promise<void> {
    const client = this.client;
    const engine = this.getVolcengineLocalVideoEngine(client);
    if (!engine) {
      this.localVideoBound = false;
      return;
    }

    if (!this.localVideoElement) {
      await this.unbindLocalVideo(client);
      return;
    }

    try {
      engine.setLocalVideoPlayer(VOLCENGINE_MAIN_STREAM_INDEX, {
        playerId: LOCAL_VIDEO_PLAYER_ID,
        renderDom: this.localVideoElement,
        renderMode: VOLCENGINE_RENDER_MODE_HIDDEN,
      });
      this.localVideoBound = true;
    } catch {
      this.localVideoBound = false;
      return;
    }

    try {
      await engine.play?.(
        undefined,
        undefined,
        VOLCENGINE_MAIN_STREAM_INDEX,
        LOCAL_VIDEO_PLAYER_ID,
      );
    } catch {
      // Local preview playback is best-effort; RTC room join and publishing must continue.
    }
  }

  private async unbindLocalVideo(client: RtcClient | undefined): Promise<void> {
    if (!this.localVideoBound) {
      return;
    }
    const engine = this.getVolcengineLocalVideoEngine(client);
    if (!engine) {
      this.localVideoBound = false;
      return;
    }
    try {
      engine.stop?.(undefined, undefined, VOLCENGINE_MAIN_STREAM_INDEX, LOCAL_VIDEO_PLAYER_ID);
    } catch {
      // Local preview teardown is best-effort; RTC room leave must continue.
    }
    try {
      engine.setLocalVideoPlayer(VOLCENGINE_MAIN_STREAM_INDEX, {
        playerId: LOCAL_VIDEO_PLAYER_ID,
      });
    } catch {
      // Local preview teardown is best-effort; RTC room leave must continue.
    }
    this.localVideoBound = false;
  }

  private requireClient(): RtcClient {
    if (!this.client) {
      throw new Error('RTC media runtime is not joined.');
    }
    return this.client;
  }
}

export function createSdkworkRtcMediaService(
  dependencies?: SdkworkRtcMediaServiceDependencies,
): SdkworkRtcMediaService {
  return new SdkworkStandardRtcMediaService(dependencies);
}

export function resolveRtcMediaPublishKinds(
  options: SdkworkRtcMediaJoinOptions,
): readonly Extract<RtcTrackKind, 'audio' | 'video'>[] {
  return shouldPublishVideo(options) ? ['audio', 'video'] : ['audio'];
}

export const rtcMediaService = createSdkworkRtcMediaService();
