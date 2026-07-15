import {
  isImDesktopGroupKnowledgebaseHostAvailable,
  isValidGroupKnowledgebaseLaunchTicket,
  openImDesktopGroupKnowledgebase,
} from '@sdkwork/im-pc-core/host';
import { isSdkworkChatDesktopRuntime } from '@sdkwork/im-pc-core/runtime/desktopEnvironment';
import {
  buildGroupKnowledgebaseBrowserUrl,
  buildGroupKnowledgebaseBrowserUrlForBaseUrl,
  isGroupKnowledgebaseBrowserDestinationConfigured,
} from '@sdkwork/im-pc-core/runtime/groupKnowledgebaseBrowserDestination';
import {
  getAppSdkClientWithSession,
  type SdkworkImAppClient,
} from '@sdkwork/im-pc-core/sdk/appSdkClient';
import { uuid } from '@sdkwork/utils';

export type GroupKnowledgebaseLaunchOutcome =
  | { kind: 'opened' }
  | { kind: 'provisioning' }
  | { kind: 'unavailable' }
  | { kind: 'cancelled' }
  | { kind: 'failed' };

export type GroupKnowledgebaseInitializationOutcome =
  | { kind: 'active' }
  | { kind: 'provisioning' }
  | { kind: 'unavailable' }
  | { kind: 'cancelled' }
  | { kind: 'failed' };

type GroupKnowledgebaseCreateOperation = SdkworkImAppClient['chat']['conversations']['knowledgebase']['create'];
type GroupKnowledgebaseCreateArguments = Parameters<GroupKnowledgebaseCreateOperation>;
type GroupKnowledgebaseCreateRequest = GroupKnowledgebaseCreateArguments[1];
type GroupKnowledgebaseCreateParams = GroupKnowledgebaseCreateArguments[2];
type GroupKnowledgebaseLaunchOperation = SdkworkImAppClient['chat']['conversations']['knowledgebase']['launch'];
type GroupKnowledgebaseLaunchArguments = Parameters<GroupKnowledgebaseLaunchOperation>;
type GroupKnowledgebaseLaunchRequest = GroupKnowledgebaseLaunchArguments[1];
type GroupKnowledgebaseLaunchParams = GroupKnowledgebaseLaunchArguments[2];
type GroupKnowledgebaseRetrieveOperation = SdkworkImAppClient['chat']['conversations']['knowledgebase']['retrieve'];

export interface GroupKnowledgebaseLaunchClient {
  readonly chat: {
    readonly conversations: {
      readonly knowledgebase: {
        create: GroupKnowledgebaseCreateOperation;
        launch: GroupKnowledgebaseLaunchOperation;
        retrieve: GroupKnowledgebaseRetrieveOperation;
      };
    };
  };
}

type GroupKnowledgebaseCreateResponse = Awaited<ReturnType<
  GroupKnowledgebaseLaunchClient['chat']['conversations']['knowledgebase']['create']
>>;
type GroupKnowledgebaseLaunchResponse = Awaited<ReturnType<
  GroupKnowledgebaseLaunchClient['chat']['conversations']['knowledgebase']['launch']
>>;
type GroupKnowledgebaseRetrieveResponse = Awaited<ReturnType<
  GroupKnowledgebaseLaunchClient['chat']['conversations']['knowledgebase']['retrieve']
>>;
export type GroupKnowledgebaseLifecycleState = GroupKnowledgebaseLaunchResponse['lifecycleState'];
export type GroupKnowledgebaseAccessMode = 'loading' | 'retry' | 'initialize' | 'open' | 'provisioning' | 'contact-owner' | 'unavailable';

export type GroupKnowledgebaseLifecycleLookup =
  | { kind: 'resolved'; lifecycleState: GroupKnowledgebaseLifecycleState }
  | { kind: 'unavailable' }
  | { kind: 'failed' };

export interface GroupKnowledgebaseAccessContext {
  canInitialize: boolean;
  canOpen: boolean;
  hasAuthenticatedSession: boolean;
  hasMemberAccessLoadError?: boolean;
  hasLifecycleUnavailable?: boolean;
  hasLifecycleLoadError?: boolean;
  isLoading?: boolean;
}

/**
 * This is an interaction hint only. The IM service remains the authorization
 * authority for both initialization and launch-ticket issuance.
 */
export function resolveGroupKnowledgebaseAccessMode(
  lifecycleState: GroupKnowledgebaseLifecycleState | null | undefined,
  access: GroupKnowledgebaseAccessContext,
): GroupKnowledgebaseAccessMode {
  if (!access.hasAuthenticatedSession) {
    return 'unavailable';
  }
  if (access.isLoading === true) {
    return 'loading';
  }
  if (access.hasMemberAccessLoadError === true) {
    return 'retry';
  }
  if (access.hasLifecycleUnavailable === true) {
    return 'unavailable';
  }
  if (access.hasLifecycleLoadError === true) {
    return access.canInitialize ? 'retry' : 'unavailable';
  }

  switch (lifecycleState) {
    case 'active':
      return access.canOpen ? 'open' : 'unavailable';
    case 'absent':
    case 'failed':
      return access.canInitialize ? 'initialize' : 'contact-owner';
    case 'provisioning':
      return access.canInitialize ? 'provisioning' : 'contact-owner';
    case 'archived':
    case 'deleted':
    default:
      return 'unavailable';
  }
}

export interface GroupKnowledgebaseLaunchServiceDependencies {
  createInitializationIdempotencyKey?: () => string;
  createIdempotencyKey?: () => string;
  getClient?: () => GroupKnowledgebaseLaunchClient;
  isBrowserDestinationConfigured?: () => boolean;
  isDesktopHostAvailable?: () => boolean;
  isDesktopRuntime?: () => boolean;
  openDesktop?: (request: { launchTicket: string }) => Promise<boolean>;
  reserveBrowserWindow?: () => GroupKnowledgebaseBrowserWindow | null;
  resolveBrowserUrl?: (launchTicket: string) => string | null;
}

export interface GroupKnowledgebaseLaunchOptions {
  signal?: AbortSignal;
}

export function createGroupKnowledgebaseLaunchIdempotencyKey(): string {
  return `pc-group-knowledgebase-launch-${uuid()}`;
}

export function createGroupKnowledgebaseInitializationIdempotencyKey(): string {
  return `pc-group-knowledgebase-initialize-${uuid()}`;
}

export interface GroupKnowledgebaseBrowserWindow {
  close(): void;
  navigate(url: string): boolean;
}

export {
  buildGroupKnowledgebaseBrowserUrl,
  buildGroupKnowledgebaseBrowserUrlForBaseUrl,
  isGroupKnowledgebaseBrowserDestinationConfigured,
};

export function reserveGroupKnowledgebaseBrowserWindow(): GroupKnowledgebaseBrowserWindow | null {
  if (typeof window === 'undefined') {
    return null;
  }

  try {
    // This runs synchronously in the click task so browsers do not treat the
    // later ticket request as an unsolicited popup. The page never receives
    // a group identity or ticket until its opener has been severed.
    const popup = window.open('about:blank', '_blank');
    if (!popup) {
      return null;
    }
    try {
      popup.opener = null;
      if (popup.opener !== null) {
        popup.close();
        return null;
      }
    } catch {
      popup.close();
      return null;
    }
    return {
      close() {
        popup.close();
      },
      navigate(url: string): boolean {
        try {
          popup.location.replace(url);
          return true;
        } catch {
          popup.close();
          return false;
        }
      },
    };
  } catch {
    return null;
  }
}

class GroupKnowledgebaseLaunchCancelledError extends Error {
  constructor() {
    super('Group knowledgebase launch was cancelled.');
  }
}

function throwIfGroupKnowledgebaseLaunchCancelled(signal?: AbortSignal): void {
  if (signal?.aborted) {
    throw new GroupKnowledgebaseLaunchCancelledError();
  }
}

/**
 * Generated SDK calls do not take an AbortSignal. This races their result with
 * cancellation while retaining a rejection handler on the original promise,
 * so a group/session switch releases UI state without leaking an unhandled
 * rejection if the transport settles later.
 */
function awaitGroupKnowledgebaseAbortable<T>(
  operation: Promise<T>,
  signal?: AbortSignal,
): Promise<T> {
  if (!signal) {
    return operation;
  }

  return new Promise<T>((resolve, reject) => {
    let settled = false;
    const settle = (callback: () => void): void => {
      if (settled) {
        return;
      }
      settled = true;
      signal.removeEventListener('abort', onAbort);
      callback();
    };
    const onAbort = (): void => {
      settle(() => reject(new GroupKnowledgebaseLaunchCancelledError()));
    };

    signal.addEventListener('abort', onAbort, { once: true });
    operation.then(
      (value) => settle(() => resolve(value)),
      (error: unknown) => settle(() => reject(error)),
    );
    if (signal.aborted) {
      onAbort();
    }
  });
}

function readGroupKnowledgebaseLifecycleState(value: unknown): GroupKnowledgebaseLifecycleState | null {
  if (typeof value !== 'string') {
    return null;
  }

  switch (value.toLowerCase()) {
    case 'absent':
    case 'provisioning':
    case 'active':
    case 'failed':
    case 'archived':
    case 'deleted':
      return value.toLowerCase() as GroupKnowledgebaseLifecycleState;
    default:
      return null;
  }
}

function readHttpStatus(error: unknown): number | undefined {
  if (!error || typeof error !== 'object') {
    return undefined;
  }
  const record = error as {
    httpStatus?: unknown;
    http_status?: unknown;
    status?: unknown;
    statusCode?: unknown;
    response?: { status?: unknown };
    body?: { status?: unknown };
    detail?: { status?: unknown };
  };
  for (const value of [
    record.httpStatus,
    record.http_status,
    record.status,
    record.statusCode,
    record.response?.status,
    record.body?.status,
    record.detail?.status,
  ]) {
    const status = typeof value === 'number' ? value : Number(value);
    if (Number.isInteger(status) && status >= 400 && status <= 599) {
      return status;
    }
  }
  return undefined;
}

class SdkworkGroupKnowledgebaseLaunchService {
  private readonly createInitializationIdempotencyKey: () => string;
  private readonly createIdempotencyKey: () => string;
  private readonly getClient: () => GroupKnowledgebaseLaunchClient;
  private readonly isBrowserDestinationConfigured: () => boolean;
  private readonly isDesktopHostAvailable: () => boolean;
  private readonly isDesktopRuntime: () => boolean;
  private readonly openDesktop: (request: { launchTicket: string }) => Promise<boolean>;
  private readonly reserveBrowserWindow: () => GroupKnowledgebaseBrowserWindow | null;
  private readonly resolveBrowserUrl: (launchTicket: string) => string | null;

  constructor(dependencies: GroupKnowledgebaseLaunchServiceDependencies = {}) {
    this.createInitializationIdempotencyKey = dependencies.createInitializationIdempotencyKey
      ?? createGroupKnowledgebaseInitializationIdempotencyKey;
    this.createIdempotencyKey = dependencies.createIdempotencyKey
      ?? createGroupKnowledgebaseLaunchIdempotencyKey;
    this.getClient = dependencies.getClient ?? getAppSdkClientWithSession;
    this.isBrowserDestinationConfigured = dependencies.isBrowserDestinationConfigured
      ?? isGroupKnowledgebaseBrowserDestinationConfigured;
    this.isDesktopHostAvailable = dependencies.isDesktopHostAvailable
      ?? isImDesktopGroupKnowledgebaseHostAvailable;
    this.isDesktopRuntime = dependencies.isDesktopRuntime ?? isSdkworkChatDesktopRuntime;
    this.openDesktop = dependencies.openDesktop ?? openImDesktopGroupKnowledgebase;
    this.reserveBrowserWindow = dependencies.reserveBrowserWindow
      ?? reserveGroupKnowledgebaseBrowserWindow;
    this.resolveBrowserUrl = dependencies.resolveBrowserUrl ?? buildGroupKnowledgebaseBrowserUrl;
  }

  async initialize(
    conversationId: string,
    options: GroupKnowledgebaseLaunchOptions = {},
  ): Promise<GroupKnowledgebaseInitializationOutcome> {
    const signal = options.signal;
    if (signal?.aborted) {
      return { kind: 'cancelled' };
    }

    const normalizedConversationId = conversationId.trim();
    if (!normalizedConversationId) {
      return { kind: 'unavailable' };
    }

    try {
      throwIfGroupKnowledgebaseLaunchCancelled(signal);
      const idempotencyKey = this.createInitializationIdempotencyKey();
      if (typeof idempotencyKey !== 'string' || idempotencyKey.trim().length === 0) {
        throw new Error('A non-empty group knowledgebase initialization idempotency key is required.');
      }
      const request: GroupKnowledgebaseCreateRequest = {};
      const params: GroupKnowledgebaseCreateParams = { idempotencyKey };
      const response: GroupKnowledgebaseCreateResponse = await awaitGroupKnowledgebaseAbortable(
        this.getClient().chat.conversations.knowledgebase.create(
          normalizedConversationId,
          request,
          params,
        ),
        signal,
      );
      throwIfGroupKnowledgebaseLaunchCancelled(signal);
      if (response.conversationId !== normalizedConversationId) {
        return { kind: 'failed' };
      }
      switch (readGroupKnowledgebaseLifecycleState(response.lifecycleState)) {
        case 'active':
          return { kind: 'active' };
        case 'provisioning':
          return { kind: 'provisioning' };
        case 'archived':
        case 'deleted':
          return { kind: 'unavailable' };
        case 'absent':
        case 'failed':
        default:
          return { kind: 'failed' };
      }
    } catch (error) {
      if (signal?.aborted || error instanceof GroupKnowledgebaseLaunchCancelledError) {
        return { kind: 'cancelled' };
      }
      const status = readHttpStatus(error);
      return status === 403 || status === 404
        ? { kind: 'unavailable' }
        : { kind: 'failed' };
    }
  }

  async open(
    conversationId: string,
    options: GroupKnowledgebaseLaunchOptions = {},
  ): Promise<GroupKnowledgebaseLaunchOutcome> {
    const signal = options.signal;
    if (signal?.aborted) {
      return { kind: 'cancelled' };
    }

    const normalizedConversationId = conversationId.trim();
    if (!normalizedConversationId) {
      return { kind: 'unavailable' };
    }
    const desktopRuntime = this.isDesktopRuntime();
    if (desktopRuntime && !this.isDesktopHostAvailable()) {
      return { kind: 'failed' };
    }
    const browserWindow = desktopRuntime ? null : this.reserveBrowserWindow();
    if (!desktopRuntime && !browserWindow) {
      return { kind: 'failed' };
    }
    if (!desktopRuntime && !this.isBrowserDestinationConfigured()) {
      browserWindow?.close();
      return { kind: 'failed' };
    }

    let outcome: GroupKnowledgebaseLaunchOutcome;
    try {
      // Generate before constructing a client so an unavailable secure random source
      // never starts a network-capable SDK flow.
      throwIfGroupKnowledgebaseLaunchCancelled(signal);
      const initialLaunchParams = this.createLaunchParams();
      const client = this.getClient();
      const launchResponse = await this.launch(client, normalizedConversationId, initialLaunchParams, signal);
      outcome = await this.completeLaunch(
        launchResponse,
        normalizedConversationId,
        desktopRuntime,
        browserWindow,
        signal,
      );
    } catch (error) {
      outcome = signal?.aborted || error instanceof GroupKnowledgebaseLaunchCancelledError
        ? { kind: 'cancelled' }
        : { kind: 'failed' };
    }

    if (outcome.kind !== 'opened') {
      browserWindow?.close();
    }
    return outcome;
  }

  async retrieveLifecycle(
    conversationId: string,
  ): Promise<GroupKnowledgebaseLifecycleLookup> {
    const normalizedConversationId = conversationId.trim();
    if (!normalizedConversationId) {
      return { kind: 'failed' };
    }
    try {
      const response: GroupKnowledgebaseRetrieveResponse = await this.getClient()
        .chat.conversations.knowledgebase.retrieve(normalizedConversationId);
      if (response.conversationId !== normalizedConversationId) {
        return { kind: 'failed' };
      }
      const lifecycleState = readGroupKnowledgebaseLifecycleState(response.lifecycleState);
      return lifecycleState
        ? { kind: 'resolved', lifecycleState }
        : { kind: 'failed' };
    } catch (error) {
      const status = readHttpStatus(error);
      return status === 403 || status === 404
        ? { kind: 'unavailable' }
        : { kind: 'failed' };
    }
  }

  async retrieveLifecycleState(
    conversationId: string,
  ): Promise<GroupKnowledgebaseLifecycleState | null> {
    const lookup = await this.retrieveLifecycle(conversationId);
    return lookup.kind === 'resolved' ? lookup.lifecycleState : null;
  }

  private createLaunchParams(): GroupKnowledgebaseLaunchParams {
    const idempotencyKey = this.createIdempotencyKey();
    if (typeof idempotencyKey !== 'string' || idempotencyKey.trim().length === 0) {
      throw new Error('A non-empty group knowledgebase launch idempotency key is required.');
    }
    return { idempotencyKey };
  }

  private async launch(
    client: GroupKnowledgebaseLaunchClient,
    conversationId: string,
    launchParams: GroupKnowledgebaseLaunchParams,
    signal?: AbortSignal,
  ): Promise<GroupKnowledgebaseLaunchResponse> {
    throwIfGroupKnowledgebaseLaunchCancelled(signal);
    const launchRequest: GroupKnowledgebaseLaunchRequest = {};
    const response = await awaitGroupKnowledgebaseAbortable(
      client.chat.conversations.knowledgebase.launch(
        conversationId,
        launchRequest,
        launchParams,
      ),
      signal,
    );
    throwIfGroupKnowledgebaseLaunchCancelled(signal);
    return response;
  }

  private async completeLaunch(
    launchResponse: GroupKnowledgebaseLaunchResponse,
    conversationId: string,
    desktopRuntime: boolean,
    browserWindow: GroupKnowledgebaseBrowserWindow | null,
    signal?: AbortSignal,
  ): Promise<GroupKnowledgebaseLaunchOutcome> {
    throwIfGroupKnowledgebaseLaunchCancelled(signal);
    if (launchResponse.conversationId !== conversationId) {
      return { kind: 'failed' };
    }
    const lifecycleState = readGroupKnowledgebaseLifecycleState(launchResponse.lifecycleState);
    if (!lifecycleState) {
      return { kind: 'failed' };
    }

    if (lifecycleState === 'provisioning'
      || (lifecycleState === 'active' && launchResponse.launchTicket === undefined)) {
      // Provisioning can require Drive and ACL work. Keeping a user-gesture
      // popup and header button blocked while polling is unreliable and can
      // cause duplicate ticket issuance. The next explicit click receives a
      // new one-time ticket only after the aggregate is active.
      return { kind: 'provisioning' };
    }

    if (lifecycleState !== 'active' || !launchResponse.launchTicket) {
      return { kind: 'unavailable' };
    }
    if (!isValidGroupKnowledgebaseLaunchTicket(launchResponse.launchTicket)) {
      return { kind: 'failed' };
    }

    if (desktopRuntime) {
      throwIfGroupKnowledgebaseLaunchCancelled(signal);
      if (!this.isDesktopHostAvailable()) {
        return { kind: 'failed' };
      }
      try {
        const opened = await awaitGroupKnowledgebaseAbortable(
          this.openDesktop({ launchTicket: launchResponse.launchTicket }),
          signal,
        );
        throwIfGroupKnowledgebaseLaunchCancelled(signal);
        return opened
          ? { kind: 'opened' }
          : { kind: 'failed' };
      } catch (error) {
        return error instanceof GroupKnowledgebaseLaunchCancelledError
          ? { kind: 'cancelled' }
          : { kind: 'failed' };
      }
    }

    throwIfGroupKnowledgebaseLaunchCancelled(signal);
    const browserUrl = this.resolveBrowserUrl(launchResponse.launchTicket);
    if (browserUrl && browserWindow?.navigate(browserUrl)) {
      return { kind: 'opened' };
    }
    return { kind: 'failed' };
  }

}

export function createGroupKnowledgebaseLaunchService(
  dependencies?: GroupKnowledgebaseLaunchServiceDependencies,
): SdkworkGroupKnowledgebaseLaunchService {
  return new SdkworkGroupKnowledgebaseLaunchService(dependencies);
}

export const groupKnowledgebaseLaunchService = createGroupKnowledgebaseLaunchService();
