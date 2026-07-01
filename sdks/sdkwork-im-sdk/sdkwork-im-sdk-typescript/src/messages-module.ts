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
} from '@sdkwork/im-sdk-generated';
import type {
  DeleteMessageFavoriteResponse,
  FavoriteMessagesResponse,
  MessageVisibilityMutationResult,
} from './openapi-compat-types';
import type { ImTransportClientLike } from './transport-client-like';

export class ImMessagesModule {
  readonly favorites = {
    list: (params?: QueryParams & { favoriteType?: MessageFavoriteType }): Promise<FavoriteMessagesResponse> =>
      this.listFavorites(params),
    create: (messageId: string | number, body: FavoriteMessageRequest): Promise<MessageFavoriteView> =>
      this.favoriteMessage(messageId, body),
    delete: (favoriteId: string | number): Promise<DeleteMessageFavoriteResponse> =>
      this.deleteFavorite(favoriteId),
  };

  constructor(private readonly transportClient: ImTransportClientLike) {}

  addReaction(
    messageId: string | number,
    reactionKeyOrBody: string | MessageReactionRequest,
  ): Promise<MessageReactionMutationResult> {
    const body = typeof reactionKeyOrBody === 'string'
      ? { reactionKey: reactionKeyOrBody }
      : reactionKeyOrBody;
    return this.transportClient.chat.messages.reactions.create(messageId, body);
  }

  removeReaction(
    messageId: string | number,
    reactionKeyOrBody: string | MessageReactionRequest,
  ): Promise<MessageReactionMutationResult> {
    const body = typeof reactionKeyOrBody === 'string'
      ? { reactionKey: reactionKeyOrBody }
      : reactionKeyOrBody;
    return this.transportClient.chat.messages.reactions.delete(messageId, body);
  }

  pinMessage(messageId: string | number): Promise<MessagePinMutationResult> {
    return this.transportClient.chat.messages.pin.create(messageId);
  }

  unpinMessage(messageId: string | number): Promise<MessagePinMutationResult> {
    return this.transportClient.chat.messages.pin.delete(messageId);
  }

  deleteForMe(messageId: string | number): Promise<MessageVisibilityMutationResult> {
    return this.transportClient.chat.messages.visibility.delete(messageId);
  }

  recall(messageId: string | number): Promise<PostedMessageResponse> {
    return this.transportClient.chat.messages.recall(messageId);
  }

  edit(messageId: string | number, body: EditMessageRequest): Promise<PostedMessageResponse> {
    return this.transportClient.chat.messages.edit(messageId, body);
  }

  listFavorites(params?: QueryParams & { favoriteType?: MessageFavoriteType }): Promise<FavoriteMessagesResponse> {
    return this.transportClient.chat.messages.favorites.list(params);
  }

  favoriteMessage(messageId: string | number, body: FavoriteMessageRequest): Promise<MessageFavoriteView> {
    return this.transportClient.chat.messages.favorites.create(messageId, body);
  }

  deleteFavorite(favoriteId: string | number): Promise<DeleteMessageFavoriteResponse> {
    return this.transportClient.chat.messages.favorites.delete(favoriteId);
  }
}
