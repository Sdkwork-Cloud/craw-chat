import {
  ImSdkClient,
  type ConversationMessageListResponse,
  type ImSdkClientOptions,
  type MessageHistoryListParams,
  type PostMessageResult,
} from '@sdkwork/im-sdk';

let imSdkClient: ImSdkClient | null = null;

function resolveImApiBaseUrl(): string {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  const fromEnv = meta.env?.SDKWORK_IM_API_BASE_URL
    ?? meta.env?.VITE_SDKWORK_IM_API_BASE_URL;
  if (typeof fromEnv === 'string' && fromEnv.trim().length > 0) {
    return fromEnv.trim();
  }
  return '/';
}

function resolveImAuthToken(): string | undefined {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  return meta.env?.SDKWORK_IM_AUTH_TOKEN
    ?? meta.env?.VITE_SDKWORK_IM_AUTH_TOKEN;
}

function resolveImAccessToken(): string | undefined {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | undefined>;
  };
  return meta.env?.SDKWORK_IM_ACCESS_TOKEN
    ?? meta.env?.VITE_SDKWORK_IM_ACCESS_TOKEN;
}

function resolveImSdkClientOptions(): ImSdkClientOptions {
  return {
    apiBaseUrl: resolveImApiBaseUrl(),
    authToken: resolveImAuthToken(),
    accessToken: resolveImAccessToken(),
    platform: 'h5',
  };
}

export function setImSdkClient(client: ImSdkClient | null): void {
  imSdkClient = client;
}

export function getImSdkClient(): ImSdkClient {
  if (!imSdkClient) {
    imSdkClient = new ImSdkClient(resolveImSdkClientOptions());
  }
  return imSdkClient;
}

export interface ListMessagesOptions {
  params?: MessageHistoryListParams;
}

export async function listMessages(
  conversationId: string,
  options: ListMessagesOptions = {},
): Promise<ConversationMessageListResponse> {
  const client = getImSdkClient();
  return client.conversations.listMessages(conversationId, options.params);
}

export async function postText(
  conversationId: string,
  text: string,
): Promise<PostMessageResult> {
  const client = getImSdkClient();
  return client.conversations.postText(conversationId, text);
}
