using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.Sdk.Generated.Models;
using SdkHttpClient = Sdkwork.Im.Sdk.Generated.Http.HttpClient;

namespace Sdkwork.Im.Sdk.Generated.Api
{
    public class ChatApi
    {
        private readonly SdkHttpClient _client;

        public ChatApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// List IM contacts
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ContactsListResponse?> ContactsListAsync(int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ContactsListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath("/chat/contacts"), queryString));
        }

        /// <summary>
        /// List current inbox window
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.InboxListResponse?> InboxListAsync(int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.InboxListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath("/chat/inbox"), queryString));
        }

        /// <summary>
        /// Create a conversation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsCreateResponse201?> ConversationsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateConversationRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsCreateResponse201>(ApiPaths.ImPath("/chat/conversations"), body, null, null, "application/json");
        }

        /// <summary>
        /// Create an agent dialog
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentDialogsCreateResponse201?> ConversationsAgentDialogsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateAgentDialogRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentDialogsCreateResponse201>(ApiPaths.ImPath("/chat/conversations/agent_dialogs"), body, null, null, "application/json");
        }

        /// <summary>
        /// Create an agent handoff
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentHandoffsCreateResponse201?> ConversationsAgentHandoffsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateAgentHandoffRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentHandoffsCreateResponse201>(ApiPaths.ImPath("/chat/conversations/agent_handoffs"), body, null, null, "application/json");
        }

        /// <summary>
        /// Create a system channel
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsSystemChannelsCreateResponse201?> ConversationsSystemChannelsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateSystemChannelRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsSystemChannelsCreateResponse201>(ApiPaths.ImPath("/chat/conversations/system_channels"), body, null, null, "application/json");
        }

        /// <summary>
        /// Create a thread conversation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsThreadsCreateResponse201?> ConversationsThreadsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateThreadConversationRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsThreadsCreateResponse201>(ApiPaths.ImPath("/chat/conversations/threads"), body, null, null, "application/json");
        }

        /// <summary>
        /// Create a direct chat conversation binding
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsDirectChatsBindingsCreateResponse201?> ConversationsDirectChatsBindingsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.BindDirectChatRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsDirectChatsBindingsCreateResponse201>(ApiPaths.ImPath("/chat/conversations/direct_chats/bindings"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve agent handoff state
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentHandoffRetrieveResponse?> ConversationsAgentHandoffRetrieveAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentHandoffRetrieveResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/agent_handoff"));
        }

        /// <summary>
        /// Accept agent handoff
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentHandoffAcceptResponse?> ConversationsAgentHandoffAcceptAsync(string conversationId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentHandoffAcceptResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/agent_handoff/accept"), null);
        }

        /// <summary>
        /// Resolve agent handoff
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentHandoffResolveResponse?> ConversationsAgentHandoffResolveAsync(string conversationId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentHandoffResolveResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/agent_handoff/resolve"), null);
        }

        /// <summary>
        /// Close agent handoff
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentHandoffCloseResponse?> ConversationsAgentHandoffCloseAsync(string conversationId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsAgentHandoffCloseResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/agent_handoff/close"), null);
        }

        /// <summary>
        /// Retrieve conversation summary
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsRetrieveResponse?> ConversationsRetrieveAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsRetrieveResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}"));
        }

        /// <summary>
        /// List conversation members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersListResponse?> ConversationsMembersListAsync(string conversationId, int? pageSize = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members"), queryString));
        }

        /// <summary>
        /// Add a conversation member
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersAddResponse?> ConversationsMembersAddAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.AddConversationMemberRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersAddResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/add"), body, null, null, "application/json");
        }

        /// <summary>
        /// Remove a conversation member
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersRemoveResponse?> ConversationsMembersRemoveAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.RemoveConversationMemberRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersRemoveResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/remove"), body, null, null, "application/json");
        }

        /// <summary>
        /// Transfer conversation owner
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersTransferOwnerResponse?> ConversationsMembersTransferOwnerAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.TransferConversationOwnerRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersTransferOwnerResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/transfer_owner"), body, null, null, "application/json");
        }

        /// <summary>
        /// Change conversation member role
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersChangeRoleResponse?> ConversationsMembersChangeRoleAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.ChangeConversationMemberRoleRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersChangeRoleResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/change_role"), body, null, null, "application/json");
        }

        /// <summary>
        /// Leave a conversation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersLeaveResponse?> ConversationsMembersLeaveAsync(string conversationId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersLeaveResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/leave"), null);
        }

        /// <summary>
        /// Accept a conversation invitation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersAcceptInvitationResponse?> ConversationsMembersAcceptInvitationAsync(string conversationId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsMembersAcceptInvitationResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/accept_invitation"), null);
        }

        /// <summary>
        /// Retrieve conversation preferences
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsPreferencesRetrieveResponse?> ConversationsPreferencesRetrieveAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsPreferencesRetrieveResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/preferences"));
        }

        /// <summary>
        /// Update conversation preferences
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsPreferencesUpdateResponse?> ConversationsPreferencesUpdateAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.UpdateConversationPreferencesRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsPreferencesUpdateResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/preferences"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve conversation profile
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsProfileRetrieveResponse?> ConversationsProfileRetrieveAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsProfileRetrieveResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/profile"));
        }

        /// <summary>
        /// Update conversation profile
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsProfileUpdateResponse?> ConversationsProfileUpdateAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.UpdateConversationProfileRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsProfileUpdateResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/profile"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve read cursor
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsReadCursorRetrieveResponse?> ConversationsReadCursorRetrieveAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsReadCursorRetrieveResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/read_cursor"));
        }

        /// <summary>
        /// Update read cursor
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsReadCursorUpdateResponse?> ConversationsReadCursorUpdateAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.UpdateReadCursorRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsReadCursorUpdateResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/read_cursor"), body, null, null, "application/json");
        }

        /// <summary>
        /// List member directory
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsMemberDirectoryListResponse?> ConversationsMemberDirectoryListAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsMemberDirectoryListResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/member_directory"));
        }

        /// <summary>
        /// List conversation message history
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationMessageListResponse?> ConversationsMessagesListAsync(string conversationId, int? afterSeq = null, int? pageSize = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("afterSeq", afterSeq, "form", true, false, null),
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationMessageListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/messages"), queryString));
        }

        /// <summary>
        /// Post a conversation message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsMessagesCreateResponse201?> ConversationsMessagesCreateAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.PostMessageRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsMessagesCreateResponse201>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/messages"), body, null, null, "application/json");
        }

        /// <summary>
        /// Publish a system channel message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsSystemChannelPublishResponse?> ConversationsSystemChannelPublishAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.PostMessageRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsSystemChannelPublishResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/system_channel/publish"), body, null, null, "application/json");
        }

        /// <summary>
        /// List pinned messages
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsPinsListResponse?> ConversationsPinsListAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsPinsListResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/pins"));
        }

        /// <summary>
        /// Retrieve message interaction summary
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationsMessagesInteractionSummaryRetrieveResponse?> ConversationsMessagesInteractionSummaryRetrieveAsync(string conversationId, string messageId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationsMessagesInteractionSummaryRetrieveResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/interaction_summary"));
        }

        /// <summary>
        /// Edit a message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessagesEditResponse?> MessagesEditAsync(string messageId, Sdkwork.Im.Sdk.Generated.Models.EditMessageRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessagesEditResponse>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/edit"), body, null, null, "application/json");
        }

        /// <summary>
        /// Recall a message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessagesRecallResponse?> MessagesRecallAsync(string messageId, Sdkwork.Im.Sdk.Generated.Models.RecallMessageRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessagesRecallResponse>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/recall"), body, null, null, "application/json");
        }

        /// <summary>
        /// List message favorites
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessagesFavoritesListResponse?> MessagesFavoritesListAsync(int? pageSize = null, string? cursor = null, string? favoriteType = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("page_size", pageSize, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("favoriteType", favoriteType, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.MessagesFavoritesListResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath("/chat/messages/favorites"), queryString));
        }

        /// <summary>
        /// Favorite a message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessagesFavoritesCreateResponse201?> MessagesFavoritesCreateAsync(string messageId, Sdkwork.Im.Sdk.Generated.Models.FavoriteMessageRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessagesFavoritesCreateResponse201>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/favorites"), body, null, null, "application/json");
        }

        /// <summary>
        /// Delete a message favorite
        /// </summary>
        public async Task MessagesFavoritesDeleteAsync(string favoriteId)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/chat/messages/favorites/{SerializePathParameter(favoriteId, new PathParameterSpec("favoriteId", "simple", false))}"));
        }

        /// <summary>
        /// Delete message visibility for the current principal
        /// </summary>
        public async Task MessagesVisibilityDeleteAsync(string messageId)
        {
            await _client.DeleteAsync<object>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/visibility"));
        }

        /// <summary>
        /// Add a message reaction
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessagesReactionsCreateResponse201?> MessagesReactionsCreateAsync(string messageId, Sdkwork.Im.Sdk.Generated.Models.MessageReactionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessagesReactionsCreateResponse201>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/reactions"), body, null, null, "application/json");
        }

        /// <summary>
        /// Remove a message reaction
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessagesReactionsRemoveResponse?> MessagesReactionsRemoveAsync(string messageId, Sdkwork.Im.Sdk.Generated.Models.MessageReactionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessagesReactionsRemoveResponse>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/reactions/remove"), body, null, null, "application/json");
        }

        /// <summary>
        /// Pin a message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessagesPinResponse?> MessagesPinAsync(string messageId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessagesPinResponse>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/pin"), null);
        }

        /// <summary>
        /// Unpin a message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessagesUnpinResponse?> MessagesUnpinAsync(string messageId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessagesUnpinResponse>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/unpin"), null);
        }

        /// <summary>
        /// Create a live, chat, or game room bound to a group conversation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RoomsCreateResponse201?> RoomsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateRoomRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.RoomsCreateResponse201>(ApiPaths.ImPath("/chat/rooms"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve room metadata and active member count
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RoomsRetrieveResponse?> RoomsRetrieveAsync(string roomId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.RoomsRetrieveResponse>(ApiPaths.ImPath($"/chat/rooms/{SerializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false))}"));
        }

        /// <summary>
        /// Enter a room as the authenticated principal
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RoomsEnterResponse?> RoomsEnterAsync(string roomId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.RoomsEnterResponse>(ApiPaths.ImPath($"/chat/rooms/{SerializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false))}/enter"), null);
        }

        /// <summary>
        /// Leave a room as the authenticated principal
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RoomsLeaveResponse?> RoomsLeaveAsync(string roomId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.RoomsLeaveResponse>(ApiPaths.ImPath($"/chat/rooms/{SerializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false))}/leave"), null);
        }

        private sealed record PathParameterSpec(string Name, string Style, bool Explode);

        private static string SerializePathParameter(object? value, PathParameterSpec spec)
        {
            if (value is null)
            {
                return string.Empty;
            }
            var style = string.IsNullOrWhiteSpace(spec.Style) ? "simple" : spec.Style;
            if (value is System.Collections.IDictionary dictionary)
            {
                return SerializePathObject(spec.Name, dictionary, style, spec.Explode);
            }
            if (value is System.Collections.IEnumerable enumerable && value is not string)
            {
                return SerializePathArray(spec.Name, enumerable, style, spec.Explode);
            }
            return PathPrimitivePrefix(spec.Name, style) + Uri.EscapeDataString(value.ToString() ?? string.Empty);
        }

        private static string SerializePathArray(string name, System.Collections.IEnumerable values, string style, bool explode)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(Uri.EscapeDataString(item.ToString() ?? string.Empty));
                }
            }
            if (serialized.Count == 0)
            {
                return PathPrefix(name, style);
            }
            if (style == "matrix")
            {
                if (explode)
                {
                    var parts = new List<string>();
                    foreach (var item in serialized)
                    {
                        parts.Add(";" + name + "=" + item);
                    }
                    return string.Join(string.Empty, parts);
                }
                return ";" + name + "=" + string.Join(",", serialized);
            }
            var separator = explode ? "." : ",";
            return PathPrefix(name, style) + string.Join(separator, serialized);
        }

        private static string SerializePathObject(string name, System.Collections.IDictionary values, string style, bool explode)
        {
            var entries = new List<string>();
            var exploded = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                var escapedKey = Uri.EscapeDataString(item.Key.ToString() ?? string.Empty);
                var escapedValue = Uri.EscapeDataString(item.Value.ToString() ?? string.Empty);
                if (explode)
                {
                    exploded.Add(style == "matrix" ? ";" + escapedKey + "=" + escapedValue : escapedKey + "=" + escapedValue);
                }
                else
                {
                    entries.Add(escapedKey);
                    entries.Add(escapedValue);
                }
            }
            if (style == "matrix")
            {
                return explode ? string.Join(string.Empty, exploded) : ";" + name + "=" + string.Join(",", entries);
            }
            if (explode)
            {
                var separator = style == "label" ? "." : ",";
                return PathPrefix(name, style) + string.Join(separator, exploded);
            }
            return PathPrefix(name, style) + string.Join(",", entries);
        }

        private static string PathPrefix(string name, string style)
        {
            return style switch
            {
                "label" => ".",
                "matrix" => ";" + name,
                _ => string.Empty,
            };
        }

        private static string PathPrimitivePrefix(string name, string style)
        {
            return style == "matrix" ? ";" + name + "=" : PathPrefix(name, style);
        }

        private sealed record QueryParameterSpec(
            string Name,
            object? Value,
            string Style,
            bool Explode,
            bool AllowReserved,
            string? ContentType);

        private static string BuildQueryString(IEnumerable<QueryParameterSpec> parameters)
        {
            var pairs = new List<string>();
            foreach (var parameter in parameters)
            {
                AppendSerializedParameter(pairs, parameter);
            }
            return string.Join("&", pairs);
        }

        private static void AppendSerializedParameter(List<string> pairs, QueryParameterSpec parameter)
        {
            if (parameter.Value is null)
            {
                return;
            }

            if (!string.IsNullOrWhiteSpace(parameter.ContentType))
            {
                var json = System.Text.Json.JsonSerializer.Serialize(parameter.Value);
                pairs.Add(Uri.EscapeDataString(parameter.Name) + "=" + EncodeQueryValue(json, parameter.AllowReserved));
                return;
            }

            var style = string.IsNullOrWhiteSpace(parameter.Style) ? "form" : parameter.Style;
            if (style == "deepObject" && parameter.Value is System.Collections.IDictionary deepObject)
            {
                AppendDeepObjectParameter(pairs, parameter.Name, deepObject, parameter.AllowReserved);
            }
            else if (parameter.Value is System.Collections.IEnumerable enumerable && parameter.Value is not string && parameter.Value is not System.Collections.IDictionary)
            {
                AppendArrayParameter(pairs, parameter.Name, enumerable, style, parameter.Explode, parameter.AllowReserved);
            }
            else if (parameter.Value is System.Collections.IDictionary dictionary)
            {
                AppendObjectParameter(pairs, parameter.Name, dictionary, style, parameter.Explode, parameter.AllowReserved);
            }
            else
            {
                pairs.Add(Uri.EscapeDataString(parameter.Name) + "=" + EncodeQueryValue(parameter.Value.ToString() ?? string.Empty, parameter.AllowReserved));
            }
        }

        private static void AppendArrayParameter(List<string> pairs, string name, System.Collections.IEnumerable values, string style, bool explode, bool allowReserved)
        {
            var serialized = new List<string>();
            foreach (var item in values)
            {
                if (item is not null)
                {
                    serialized.Add(item.ToString() ?? string.Empty);
                }
            }
            if (serialized.Count == 0)
            {
                return;
            }
            if (style == "form" && explode)
            {
                foreach (var item in serialized)
                {
                    pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(item, allowReserved));
                }
                return;
            }
            pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(string.Join(",", serialized), allowReserved));
        }

        private static void AppendObjectParameter(List<string> pairs, string name, System.Collections.IDictionary values, string style, bool explode, bool allowReserved)
        {
            var serialized = new List<string>();
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is null)
                {
                    continue;
                }
                if (style == "form" && explode)
                {
                    pairs.Add(Uri.EscapeDataString(item.Key.ToString() ?? string.Empty) + "=" + EncodeQueryValue(item.Value.ToString() ?? string.Empty, allowReserved));
                }
                else
                {
                    serialized.Add(item.Key.ToString() ?? string.Empty);
                    serialized.Add(item.Value.ToString() ?? string.Empty);
                }
            }
            if (serialized.Count > 0)
            {
                pairs.Add(Uri.EscapeDataString(name) + "=" + EncodeQueryValue(string.Join(",", serialized), allowReserved));
            }
        }

        private static void AppendDeepObjectParameter(List<string> pairs, string name, System.Collections.IDictionary values, bool allowReserved)
        {
            foreach (System.Collections.DictionaryEntry item in values)
            {
                if (item.Value is not null)
                {
                    pairs.Add(Uri.EscapeDataString(name + "[" + item.Key + "]") + "=" + EncodeQueryValue(item.Value.ToString() ?? string.Empty, allowReserved));
                }
            }
        }

        private static string EncodeQueryValue(string value, bool allowReserved)
        {
            var encoded = Uri.EscapeDataString(value);
            if (!allowReserved)
            {
                return encoded;
            }
            return encoded
                .Replace("%3A", ":").Replace("%2F", "/").Replace("%3F", "?").Replace("%23", "#")
                .Replace("%5B", "[").Replace("%5D", "]").Replace("%40", "@").Replace("%21", "!")
                .Replace("%24", "$").Replace("%26", "&").Replace("%27", "'").Replace("%28", "(")
                .Replace("%29", ")").Replace("%2A", "*").Replace("%2B", "+").Replace("%2C", ",")
                .Replace("%3B", ";").Replace("%3D", "=");
        }

    }
}
