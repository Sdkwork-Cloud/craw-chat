import { SdkworkImClient as GeneratedSdkworkImClient } from '@sdkwork/im-sdk-generated';
import type {
  EditMessageRequest,
  FavoriteMessageRequest,
  MessageFavoriteType,
  MessageFavoriteView,
  MessagePinMutationResult,
  MessageReactionMutationResult,
  MessageReactionRequest,
  PostedMessageResponse,
  QueryParams,
  SdkworkImConfig,
} from '@sdkwork/im-sdk-generated';
import type {
  DeleteMessageFavoriteResponse,
  FavoriteMessagesResponse,
  MessageVisibilityMutationResult,
} from './openapi-compat-types';
import { ImConversationsModule } from './conversations-module';
import { ImMessagesModule } from './messages-module';
import { ImRoomsModule } from './rooms-module';
import {
  createImLiveConnection,
  type ImConnectOptions,
  type ImLiveConnection,
  type ImWebSocketAuthConfig,
  type ImWebSocketFactory,
} from './realtime';
import { ImCallsModule } from './calls-module';
import type { ImTransportClientLike } from './transport-client-like';

export interface ImSdkClientOptions {
  accessToken?: string;
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
    (typeof process !== 'undefined' &&
      process.env?.SDKWORK_IM_API_BASE_URL);
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
  return {
    baseUrl: resolveApiBaseUrl(options),
    accessToken: options.accessToken,
    authToken: options.authToken,
    headers: {
      ...(options.headers ?? {}),
      ...(options.headerProvider?.() ?? {}),
    },
    platform: options.platform,
    timeout: options.timeout,
    tokenManager: (options.tokenManager ?? options.tokenProvider) as SdkworkImConfig['tokenManager'],
  };
}

function createSocialFacade(transportClient: ImTransportClientLike): ImTransportClientLike['social'] {
  return {
    ...transportClient.social,
    contacts: {
      ...transportClient.social.contacts,
      list: (params?: QueryParams) => transportClient.chat.contacts.list(params),
    },
  };
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
    this.social = createSocialFacade(this.transportClient);
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

  setAuthToken(token: string): this {
    this.options.authToken = token;
    this.transportClient.setAuthToken?.(token);
    return this;
  }

  setAccessToken(token: string): this {
    this.options.accessToken = token;
    this.transportClient.setAccessToken?.(token);
    return this;
  }

  setTokenManager(manager: unknown): this {
    this.options.tokenManager = manager;
    this.transportClient.setTokenManager?.(manager);
    return this;
  }

  connect(options: ImConnectOptions = {}): Promise<ImLiveConnection> {
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

  addReaction(
    messageId: string | number,
    reactionKeyOrBody: string | MessageReactionRequest,
  ): Promise<MessageReactionMutationResult> {
    return this.messages.addReaction(messageId, reactionKeyOrBody);
  }

  removeReaction(
    messageId: string | number,
    reactionKeyOrBody: string | MessageReactionRequest,
  ): Promise<MessageReactionMutationResult> {
    return this.messages.removeReaction(messageId, reactionKeyOrBody);
  }

  pinMessage(messageId: string | number): Promise<MessagePinMutationResult> {
    return this.messages.pinMessage(messageId);
  }

  unpinMessage(messageId: string | number): Promise<MessagePinMutationResult> {
    return this.messages.unpinMessage(messageId);
  }

  deleteMessageForMe(messageId: string | number): Promise<MessageVisibilityMutationResult> {
    return this.messages.deleteForMe(messageId);
  }

  recallMessage(messageId: string | number): Promise<PostedMessageResponse> {
    return this.messages.recall(messageId);
  }

  editMessage(messageId: string | number, body: EditMessageRequest): Promise<PostedMessageResponse> {
    return this.messages.edit(messageId, body);
  }

  listMessageFavorites(params?: QueryParams & { favoriteType?: MessageFavoriteType }): Promise<FavoriteMessagesResponse> {
    return this.messages.listFavorites(params);
  }

  favoriteMessage(messageId: string | number, body: FavoriteMessageRequest): Promise<MessageFavoriteView> {
    return this.messages.favoriteMessage(messageId, body);
  }

  deleteMessageFavorite(favoriteId: string | number): Promise<DeleteMessageFavoriteResponse> {
    return this.messages.deleteFavorite(favoriteId);
  }
}

export function createClient(options: ImSdkClientOptions = {}): ImSdkClient {
  return new ImSdkClient(options);
}

export default ImSdkClient;
