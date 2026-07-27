import { SdkworkImClient as GeneratedSdkworkImClient } from '../generated/server-openapi/dist/index.js';
import type {
  EditMessageRequest,
  FavoriteMessageRequest,
  MessageFavoriteType,
  MessageFavoriteView,
  MessageMutationResult,
  MessagePinMutationResult,
  MessageReactionMutationResult,
  MessageReactionRequest,
  QueryParams,
  RecallMessageRequest,
  SdkworkImConfig,
} from '../generated/server-openapi/dist/index.js';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import type {
  DeleteMessageFavoriteResponse,
  FavoriteMessagesResponse,
} from './openapi-compat-types.js';
import { ImConversationsModule } from './conversations-module.js';
import { ImMessagesModule } from './messages-module.js';
import { ImRoomsModule } from './rooms-module.js';
import {
  createImLiveConnection,
  type ImConnectOptions,
  type ImLiveConnection,
  type ImWebSocketAuthConfig,
  type ImWebSocketFactory,
} from './realtime.js';
import { ImCallsModule } from './calls-module.js';
import type { ImTransportClientLike } from './transport-client-like.js';
import type {
  ImTransportConnection,
  ImTransportFactory,
  ImTransportKind,
  ImTransportSelectionPolicy,
} from './transport.js';
import { DEFAULT_TRANSPORT_SELECTION_POLICY } from './transport.js';
import { createDefaultTransportFactories } from './transports/index.js';
import { ImTransportSelector } from './transport-selector.js';
import { IM_CCP_WEBSOCKET_SUBPROTOCOL } from './ccp-wire.js';

function normalizeTransportProbeTimeout(value: number): number {
  return Number.isFinite(value) && value > 0
    ? value
    : DEFAULT_TRANSPORT_SELECTION_POLICY.probeTimeoutMs;
}

async function waitForTransportOpen(
  transport: ImTransportConnection,
  timeoutMs: number,
): Promise<void> {
  if (transport.state === 'open') {
    return;
  }
  if (transport.state === 'closed' || transport.state === 'closing') {
    throw new Error(`Transport "${transport.kind}" closed before it became ready.`);
  }

  await new Promise<void>((resolve, reject) => {
    let settled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let unsubscribeOpen: (() => void) | undefined;
    let unsubscribeClose: (() => void) | undefined;
    let unsubscribeError: (() => void) | undefined;

    const cleanup = (): void => {
      if (timer) {
        clearTimeout(timer);
        timer = undefined;
      }
      unsubscribeOpen?.();
      unsubscribeClose?.();
      unsubscribeError?.();
      unsubscribeOpen = undefined;
      unsubscribeClose = undefined;
      unsubscribeError = undefined;
    };
    const finish = (error?: unknown): void => {
      if (settled) {
        return;
      }
      settled = true;
      cleanup();
      if (error === undefined) {
        resolve();
      } else {
        reject(error instanceof Error ? error : new Error(String(error)));
      }
    };

    unsubscribeOpen = transport.onOpen(() => finish());
    unsubscribeClose = transport.onClose((event) => finish(
      new Error(
        `Transport "${transport.kind}" closed before open (${event.code}: ${event.reason}).`,
      ),
    ));
    unsubscribeError = transport.onError((event) => finish(event.error));
    timer = setTimeout(() => finish(
      new Error(`Transport "${transport.kind}" probe timed out after ${timeoutMs}ms.`),
    ), timeoutMs);

    // Re-check after listener registration to close the race between the
    // initial state read and a transport transition.
    if (transport.state === 'open') {
      finish();
    } else if (transport.state === 'closed' || transport.state === 'closing') {
      finish(new Error(`Transport "${transport.kind}" closed before it became ready.`));
    }
  });
}

export interface ImSdkClientOptions {
  accessToken?: string;
  apiKey?: string;
  apiBaseUrl?: string;
  authToken?: string;
  baseUrl?: string;
  headerProvider?: () => Record<string, string>;
  headers?: Record<string, string>;
  platform?: string;
  timeout?: number;
  tokenManager?: unknown;
  tokenProvider?: unknown;
  webSocketAuth?: ImWebSocketAuthConfig;
  webSocketFactory?: ImWebSocketFactory;
  websocketBaseUrl?: string;
  /**
   * 传输类型手动覆盖（'websocket'|'tcp'|'udp'）。
   *
   * 设置后启用多传输路径，SDK 按该类型选择传输工厂。
   * 未设置时使用默认 WebSocket 路径（向后兼容）。
   */
  transport?: ImTransportKind;
  /**
   * 自定义传输工厂集合。
   *
   * 用于注入原生平台（React Native / Flutter / Tauri）的 TCP/UDP 实现。
   * 未提供时使用 createDefaultTransportFactories() 创建默认集合。
   */
  transportFactories?: Map<ImTransportKind, ImTransportFactory>;
  /**
   * 传输选择策略（自动检测 + 降级）。
   *
   * 控制传输优先级和自动降级行为。
   * 默认：preferred=['websocket','tcp','udp'], autoFallback=true。
   */
  transportPolicy?: ImTransportSelectionPolicy;
}

function resolveApiBaseUrl(options: ImSdkClientOptions): string {
  const fromOptions = options.apiBaseUrl ?? options.baseUrl;
  if (fromOptions) {
    return fromOptions;
  }
  if (options.websocketBaseUrl) {
    return options.websocketBaseUrl.replace(/^ws/u, 'http');
  }
  // Fall back to SDKWORK_IM_API_BASE_URL env var (browser/Vite) or throw.
  const fromEnv =
    (typeof import.meta !== 'undefined' &&
      (import.meta as { env?: Record<string, string> }).env?.SDKWORK_IM_API_BASE_URL) ||
    (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.SDKWORK_IM_API_BASE_URL;
  if (fromEnv) {
    return fromEnv;
  }
  throw new Error(
    'ImSdkClient requires an apiBaseUrl or baseUrl option, or SDKWORK_IM_API_BASE_URL env var. ' +
      'Set it explicitly: new ImSdkClient({ apiBaseUrl: "https://your-im-gateway.example.com" })',
  );
}

function resolveWebsocketBaseUrl(options: ImSdkClientOptions): string {
  return options.websocketBaseUrl ?? resolveApiBaseUrl(options).replace(/^http/u, 'ws');
}

function toGeneratedConfig(options: ImSdkClientOptions): SdkworkImConfig {
  assertCredentialMode(options);
  const apiKey = normalizeCredential(options.apiKey);
  return {
    baseUrl: resolveApiBaseUrl(options),
    accessToken: apiKey ? undefined : options.accessToken,
    apiKey,
    authToken: apiKey ? undefined : options.authToken,
    headers: {
      ...(options.headers ?? {}),
      ...(options.headerProvider?.() ?? {}),
    },
    platform: options.platform,
    timeout: options.timeout,
    tokenManager: apiKey
      ? undefined
      : (options.tokenManager ?? options.tokenProvider) as SdkworkImConfig['tokenManager'],
  };
}

function normalizeCredential(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function readProviderCredential(
  provider: unknown,
  getter: 'getAccessToken' | 'getAuthToken',
): string | undefined {
  if (!provider || typeof provider !== 'object') {
    return undefined;
  }
  const resolve = (provider as Record<typeof getter, unknown>)[getter];
  return typeof resolve === 'function'
    ? normalizeCredential(resolve.call(provider))
    : undefined;
}

function assertCredentialMode(options: ImSdkClientOptions): void {
  const apiKey = normalizeCredential(options.apiKey);
  if (!apiKey) {
    return;
  }
  const provider = options.tokenManager ?? options.tokenProvider;
  const authToken = normalizeCredential(options.authToken)
    ?? readProviderCredential(provider, 'getAuthToken');
  const accessToken = normalizeCredential(options.accessToken)
    ?? readProviderCredential(provider, 'getAccessToken');
  if (authToken || accessToken) {
    throw new Error(
      'ImSdkClient apiKey mode must not be combined with authToken, accessToken, tokenManager, or tokenProvider credentials.',
    );
  }
}

export class ImSdkClient {
  readonly chat: ImTransportClientLike['chat'];
  readonly calls: ImCallsModule;
  readonly conversations: ImConversationsModule;
  readonly messages: ImMessagesModule;
  readonly rooms: ImRoomsModule;
  readonly social: ImTransportClientLike['social'];

  private readonly options: ImSdkClientOptions;
  private readonly transportClient: ImTransportClientLike;
  private readonly websocketBaseUrl: string;

  constructor(options: ImSdkClientOptions = {}) {
    this.options = options;
    this.websocketBaseUrl = resolveWebsocketBaseUrl(options);
    this.transportClient = new GeneratedSdkworkImClient(toGeneratedConfig(options)) as unknown as ImTransportClientLike;
    this.chat = this.transportClient.chat;
    this.social = this.transportClient.social;
    this.messages = new ImMessagesModule(this.transportClient);
    this.conversations = new ImConversationsModule(this.transportClient);
    this.rooms = new ImRoomsModule(this.transportClient);
    this.calls = new ImCallsModule(this.transportClient, {
      connect: (connectOptions) => this.connect(connectOptions),
    });
  }

  get transport(): ImTransportClientLike {
    return this.transportClient;
  }

  setApiKey(apiKey: string): this {
    const normalizedApiKey = normalizeCredential(apiKey);
    if (!normalizedApiKey) {
      throw new Error('ImSdkClient apiKey must not be empty.');
    }
    this.options.apiKey = normalizedApiKey;
    this.options.authToken = undefined;
    this.options.accessToken = undefined;
    this.options.tokenManager = undefined;
    this.options.tokenProvider = undefined;
    this.transportClient.setApiKey?.(normalizedApiKey);
    return this;
  }

  setAuthToken(token: string): this {
    this.options.apiKey = undefined;
    this.options.authToken = token;
    this.transportClient.setAuthToken?.(token);
    return this;
  }

  setAccessToken(token: string): this {
    this.options.apiKey = undefined;
    this.options.accessToken = token;
    this.transportClient.setAccessToken?.(token);
    return this;
  }

  setTokenManager(manager: unknown): this {
    this.options.apiKey = undefined;
    this.options.tokenManager = manager;
    this.options.tokenProvider = undefined;
    this.transportClient.setTokenManager?.(manager);
    return this;
  }

  connect(options: ImConnectOptions = {}): Promise<ImLiveConnection> {
    const useMultiTransport = Boolean(
      this.options.transport || this.options.transportFactories || this.options.transportPolicy,
    );

    if (!useMultiTransport) {
      // 向后兼容：使用 WebSocket 路径
      return Promise.resolve(createImLiveConnection({
        accessToken: this.options.accessToken,
        auth: this.options.webSocketAuth,
        authToken: this.options.authToken,
        headerProvider: this.options.headerProvider,
        headers: this.options.headers,
        options,
        tokenManager: this.options.tokenManager ?? this.options.tokenProvider,
        websocketBaseUrl: this.websocketBaseUrl,
        webSocketFactory: this.options.webSocketFactory,
      }));
    }

    // 多传输路径：自动检测 + 手动覆盖 + 连接失败降级
    const factories = this.options.transportFactories
      ?? createDefaultTransportFactories(this.options.webSocketFactory);
    const policy = this.options.transportPolicy ?? DEFAULT_TRANSPORT_SELECTION_POLICY;
    const selector = new ImTransportSelector(factories, policy);
    const connectionTimeoutMs = options.connectionTimeoutMs ?? 15_000;
    const headers = {
      ...(this.options.headers ?? {}),
      ...(this.options.headerProvider?.() ?? {}),
    };
    const connectOptions = {
      connectionTimeoutMs,
      headers,
      protocols: [IM_CCP_WEBSOCKET_SUBPROTOCOL],
    };

    // 构建候选传输列表（按优先级排序，手动覆盖优先）
    const candidates = selector.buildCandidateList(this.options.transport);

    // 检查手动覆盖的传输是否可用：不可用且 autoFallback=false 时抛出错误
    if (this.options.transport && !policy.autoFallback) {
      const preferredFactory = factories.get(this.options.transport);
      if (!preferredFactory?.isAvailable()) {
        return Promise.reject(
          new Error(
            `Preferred transport "${this.options.transport}" is not available in the current environment.`,
          ),
        );
      }
    }

    return this.connectWithFallback(
      candidates,
      selector,
      options.deviceId,
      connectOptions,
      options,
      policy.autoFallback,
      normalizeTransportProbeTimeout(policy.probeTimeoutMs),
    );
  }

  /**
   * 按候选顺序尝试连接，传输层连接失败时自动降级到下一个。
   *
   * 注意：此降级仅覆盖 factory.connect() 阶段（传输层建立）。
   * CCP 握手失败（如认证错误）不触发降级，因为可能是凭据问题而非传输不可用。
   */
  private async connectWithFallback(
    candidates: ImTransportKind[],
    selector: ImTransportSelector,
    deviceId: string | undefined,
    connectOptions: { connectionTimeoutMs: number; headers: Record<string, string>; protocols: string[] },
    options: ImConnectOptions,
    autoFallback: boolean,
    probeTimeoutMs: number,
    lastError?: unknown,
  ): Promise<ImLiveConnection> {
    if (candidates.length === 0) {
      if (lastError !== undefined) {
        throw lastError;
      }
      throw new Error('No transport is available in the current environment.');
    }

    const [kind, ...rest] = candidates;
    const factory = selector.getFactory(kind);
    if (!factory) {
      return this.connectWithFallback(
        rest,
        selector,
        deviceId,
        connectOptions,
        options,
        autoFallback,
        probeTimeoutMs,
        lastError,
      );
    }
    const endpoint = selector.buildEndpoint(kind, this.websocketBaseUrl, deviceId);

    let transport: ImTransportConnection | undefined;
    try {
      transport = await factory.connect(endpoint, connectOptions);
      await waitForTransportOpen(transport, probeTimeoutMs);
    } catch (error) {
      try {
        transport?.close(4008, 'transport_probe_failed');
      } catch {
        // The transport may already have closed itself while the probe failed.
      }
      if (!autoFallback) {
        throw error;
      }
      return this.connectWithFallback(
        rest,
        selector,
        deviceId,
        connectOptions,
        options,
        autoFallback,
        probeTimeoutMs,
        error,
      );
    }

    try {
      return createImLiveConnection({
        accessToken: this.options.accessToken,
        auth: this.options.webSocketAuth,
        authToken: this.options.authToken,
        headerProvider: this.options.headerProvider,
        headers: this.options.headers,
        options,
        tokenManager: this.options.tokenManager ?? this.options.tokenProvider,
        websocketBaseUrl: this.websocketBaseUrl,
        webSocketFactory: this.options.webSocketFactory,
        transport,
      });
    } catch (error) {
      // Live state-machine initialization failures are not transport failures.
      // Do not hide them by silently falling through to another protocol.
      transport.close(4000, 'live_connection_init_failed');
      throw error;
    }
  }

  addReaction(
    messageId: string,
    reactionKeyOrBody: string | MessageReactionRequest,
  ): Promise<MessageReactionMutationResult> {
    return this.messages.addReaction(messageId, reactionKeyOrBody);
  }

  removeReaction(
    messageId: string,
    reactionKeyOrBody: string | MessageReactionRequest,
  ): Promise<MessageReactionMutationResult> {
    return this.messages.removeReaction(messageId, reactionKeyOrBody);
  }

  pinMessage(messageId: string): Promise<MessagePinMutationResult> {
    return this.messages.pinMessage(messageId);
  }

  unpinMessage(messageId: string): Promise<MessagePinMutationResult> {
    return this.messages.unpinMessage(messageId);
  }

  deleteMessageForMe(messageId: string): Promise<void> {
    return this.messages.deleteForMe(messageId);
  }

  recallMessage(messageId: string, body?: RecallMessageRequest): Promise<MessageMutationResult> {
    return this.messages.recall(messageId, body);
  }

  editMessage(messageId: string, body: EditMessageRequest): Promise<MessageMutationResult> {
    return this.messages.edit(messageId, body);
  }

  listMessageFavorites(params?: QueryParams & { favoriteType?: MessageFavoriteType }): Promise<FavoriteMessagesResponse> {
    return this.messages.listFavorites(params);
  }

  favoriteMessage(messageId: string, body: FavoriteMessageRequest): Promise<MessageFavoriteView> {
    return this.messages.favoriteMessage(messageId, body);
  }

  deleteMessageFavorite(favoriteId: string): Promise<DeleteMessageFavoriteResponse> {
    return this.messages.deleteFavorite(favoriteId);
  }
}

export function createClient(options: ImSdkClientOptions = {}): ImSdkClient {
  return new ImSdkClient(options);
}

export default ImSdkClient;
