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
} from '../generated/server-openapi/dist/index.js';
import type {
  DeleteMessageFavoriteResponse,
  FavoriteMessagesResponse,
} from './openapi-compat-types';
import { requireStringIdentifier } from './identifier-boundary';
import type { ImTransportClientLike } from './transport-client-like';

export class ImMessagesModule {
  readonly favorites = {
    list: (params?: QueryParams & { favoriteType?: MessageFavoriteType }): Promise<FavoriteMessagesResponse> =>
      this.listFavorites(params),
    create: (messageId: string, body: FavoriteMessageRequest): Promise<MessageFavoriteView> =>
      this.favoriteMessage(messageId, body),
    delete: (favoriteId: string): Promise<DeleteMessageFavoriteResponse> =>
      this.deleteFavorite(favoriteId),
  };

  constructor(private readonly transportClient: ImTransportClientLike) {}

  addReaction(
    messageId: string,
    reactionKeyOrBody: string | MessageReactionRequest,
  ): Promise<MessageReactionMutationResult> {
    const body = typeof reactionKeyOrBody === 'string'
      ? { reactionKey: reactionKeyOrBody }
      : reactionKeyOrBody;
    return this.transportClient.chat.messages.reactions.create(
      requireStringIdentifier(messageId, 'messageId'),
      body,
    );
  }

  removeReaction(
    messageId: string,
    reactionKeyOrBody: string | MessageReactionRequest,
  ): Promise<MessageReactionMutationResult> {
    const body = typeof reactionKeyOrBody === 'string'
      ? { reactionKey: reactionKeyOrBody }
      : reactionKeyOrBody;
    return this.transportClient.chat.messages.reactions.remove(
      requireStringIdentifier(messageId, 'messageId'),
      body,
    );
  }

  pinMessage(messageId: string): Promise<MessagePinMutationResult> {
    return this.transportClient.chat.messages.pin(
      requireStringIdentifier(messageId, 'messageId'),
    );
  }

  unpinMessage(messageId: string): Promise<MessagePinMutationResult> {
    return this.transportClient.chat.messages.unpin(
      requireStringIdentifier(messageId, 'messageId'),
    );
  }

  deleteForMe(messageId: string): Promise<void> {
    return this.transportClient.chat.messages.visibility.delete(
      requireStringIdentifier(messageId, 'messageId'),
    );
  }

  recall(messageId: string, body: RecallMessageRequest = {}): Promise<MessageMutationResult> {
    return this.transportClient.chat.messages.recall(
      requireStringIdentifier(messageId, 'messageId'),
      body,
    );
  }

  edit(messageId: string, body: EditMessageRequest): Promise<MessageMutationResult> {
    return this.transportClient.chat.messages.edit(
      requireStringIdentifier(messageId, 'messageId'),
      body,
    );
  }

  listFavorites(params?: QueryParams & { favoriteType?: MessageFavoriteType }): Promise<FavoriteMessagesResponse> {
    return this.transportClient.chat.messages.favorites.list(params);
  }

  favoriteMessage(messageId: string, body: FavoriteMessageRequest): Promise<MessageFavoriteView> {
    return this.transportClient.chat.messages.favorites.create(
      requireStringIdentifier(messageId, 'messageId'),
      body,
    );
  }

  deleteFavorite(favoriteId: string): Promise<DeleteMessageFavoriteResponse> {
    return this.transportClient.chat.messages.favorites.delete(
      requireStringIdentifier(favoriteId, 'favoriteId'),
    );
  }
}
