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
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ContactsResponse?> ContactsListAsync(int? limit = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("limit", limit, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ContactsResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath("/chat/contacts"), queryString));
        }

        /// <summary>
        /// Retrieve current inbox window
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.InboxResponse?> InboxRetrieveAsync(int? limit = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("limit", limit, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.InboxResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath("/chat/inbox"), queryString));
        }

        /// <summary>
        /// Create a conversation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult?> ConversationsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateConversationRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult>(ApiPaths.ImPath("/chat/conversations"), body, null, null, "application/json");
        }

        /// <summary>
        /// Create an agent dialog
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult?> ConversationsAgentDialogsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateAgentDialogRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult>(ApiPaths.ImPath("/chat/conversations/agent_dialogs"), body, null, null, "application/json");
        }

        /// <summary>
        /// Create an agent handoff
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.AckResponse?> ConversationsAgentHandoffsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateAgentDialogRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.AckResponse>(ApiPaths.ImPath("/chat/conversations/agent_handoffs"), body, null, null, "application/json");
        }

        /// <summary>
        /// Create a system channel
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult?> ConversationsSystemChannelsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateConversationRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult>(ApiPaths.ImPath("/chat/conversations/system_channels"), body, null, null, "application/json");
        }

        /// <summary>
        /// Create a thread conversation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult?> ConversationsThreadsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateConversationRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult>(ApiPaths.ImPath("/chat/conversations/threads"), body, null, null, "application/json");
        }

        /// <summary>
        /// Bind a direct chat conversation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult?> ConversationsDirectChatsBindAsync(Sdkwork.Im.Sdk.Generated.Models.BindDirectChatRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult>(ApiPaths.ImPath("/chat/conversations/direct_chats/bindings"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve agent handoff state
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.AckResponse?> ConversationsAgentHandoffRetrieveAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.AckResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/agent_handoff"));
        }

        /// <summary>
        /// Accept agent handoff
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.AckResponse?> ConversationsAgentHandoffAcceptAsync(string conversationId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.AckResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/agent_handoff/accept"), null);
        }

        /// <summary>
        /// Resolve agent handoff
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.AckResponse?> ConversationsAgentHandoffResolveAsync(string conversationId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.AckResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/agent_handoff/resolve"), null);
        }

        /// <summary>
        /// Close agent handoff
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.AckResponse?> ConversationsAgentHandoffCloseAsync(string conversationId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.AckResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/agent_handoff/close"), null);
        }

        /// <summary>
        /// Retrieve conversation summary
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationSummaryView?> ConversationsRetrieveAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationSummaryView>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}"));
        }

        /// <summary>
        /// List conversation members
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ListMembersResponse?> ConversationsMembersListAsync(string conversationId, int? limit = null, string? cursor = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("limit", limit, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ListMembersResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members"), queryString));
        }

        /// <summary>
        /// Add a conversation member
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationMember?> ConversationsMembersAddAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.AddConversationMemberRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationMember>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/add"), body, null, null, "application/json");
        }

        /// <summary>
        /// Remove a conversation member
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.AckResponse?> ConversationsMembersRemoveAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.RemoveConversationMemberRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.AckResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/remove"), body, null, null, "application/json");
        }

        /// <summary>
        /// Transfer conversation owner
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationMember?> ConversationsMembersTransferOwnerAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.TransferConversationOwnerRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationMember>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/transfer_owner"), body, null, null, "application/json");
        }

        /// <summary>
        /// Change conversation member role
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationMember?> ConversationsMembersChangeRoleAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.ChangeConversationMemberRoleRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationMember>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/change_role"), body, null, null, "application/json");
        }

        /// <summary>
        /// Leave a conversation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.AckResponse?> ConversationsMembersLeaveAsync(string conversationId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.AckResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/members/leave"), null);
        }

        /// <summary>
        /// Retrieve conversation preferences
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationPreferencesView?> ConversationsPreferencesRetrieveAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationPreferencesView>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/preferences"));
        }

        /// <summary>
        /// Update conversation preferences
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationPreferencesView?> ConversationsPreferencesUpdateAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.UpdateConversationPreferencesRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationPreferencesView>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/preferences"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve conversation profile
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationProfileView?> ConversationsProfileRetrieveAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationProfileView>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/profile"));
        }

        /// <summary>
        /// Update conversation profile
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ConversationProfileView?> ConversationsProfileUpdateAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.UpdateConversationProfileRequest body)
        {
            return await _client.PatchAsync<Sdkwork.Im.Sdk.Generated.Models.ConversationProfileView>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/profile"), body, null, null, "application/json");
        }

        /// <summary>
        /// Retrieve read cursor
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ReadCursorView?> ConversationsReadCursorRetrieveAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.ReadCursorView>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/read_cursor"));
        }

        /// <summary>
        /// Update read cursor
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.ReadCursorView?> ConversationsReadCursorUpdateAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.UpdateReadCursorRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.ReadCursorView>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/read_cursor"), body, null, null, "application/json");
        }

        /// <summary>
        /// List member directory
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MemberDirectoryResponse?> ConversationsMemberDirectoryListAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.MemberDirectoryResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/member_directory"));
        }

        /// <summary>
        /// List conversation message timeline
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.TimelineResponse?> ConversationsMessagesListAsync(string conversationId, int? afterSeq = null, int? limit = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("afterSeq", afterSeq, "form", true, false, null),
                new QueryParameterSpec("limit", limit, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.TimelineResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/messages"), queryString));
        }

        /// <summary>
        /// Post a conversation message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.PostedMessageResponse?> ConversationsMessagesCreateAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.PostMessageRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.PostedMessageResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/messages"), body, null, null, "application/json");
        }

        /// <summary>
        /// Publish a system channel message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.PostedMessageResponse?> ConversationsSystemChannelPublishAsync(string conversationId, Sdkwork.Im.Sdk.Generated.Models.PostMessageRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.PostedMessageResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/system_channel/publish"), body, null, null, "application/json");
        }

        /// <summary>
        /// List pinned messages
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.PinnedMessagesResponse?> ConversationsPinsListAsync(string conversationId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.PinnedMessagesResponse>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/pins"));
        }

        /// <summary>
        /// Retrieve message interaction summary
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessageInteractionSummaryView?> ConversationsMessagesInteractionSummaryRetrieveAsync(string conversationId, string messageId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.MessageInteractionSummaryView>(ApiPaths.ImPath($"/chat/conversations/{SerializePathParameter(conversationId, new PathParameterSpec("conversationId", "simple", false))}/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/interaction_summary"));
        }

        /// <summary>
        /// Edit a message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.PostedMessageResponse?> MessagesEditAsync(string messageId, Sdkwork.Im.Sdk.Generated.Models.EditMessageRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.PostedMessageResponse>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/edit"), body, null, null, "application/json");
        }

        /// <summary>
        /// Recall a message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.PostedMessageResponse?> MessagesRecallAsync(string messageId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.PostedMessageResponse>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/recall"), null);
        }

        /// <summary>
        /// List message favorites
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.FavoriteMessagesResponse?> MessagesFavoritesListAsync(int? limit = null, string? cursor = null, string? favoriteType = null, string? q = null)
        {
            var queryString = BuildQueryString(new[]
            {
                new QueryParameterSpec("limit", limit, "form", true, false, null),
                new QueryParameterSpec("cursor", cursor, "form", true, false, null),
                new QueryParameterSpec("favoriteType", favoriteType, "form", true, false, null),
                new QueryParameterSpec("q", q, "form", true, false, null),
            });
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.FavoriteMessagesResponse>(ApiPaths.AppendQueryString(ApiPaths.ImPath("/chat/messages/favorites"), queryString));
        }

        /// <summary>
        /// Favorite a message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessageFavoriteView?> MessagesFavoritesCreateAsync(string messageId, Sdkwork.Im.Sdk.Generated.Models.FavoriteMessageRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessageFavoriteView>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/favorites"), body, null, null, "application/json");
        }

        /// <summary>
        /// Delete a message favorite
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.DeleteMessageFavoriteResponse?> MessagesFavoritesDeleteAsync(string favoriteId)
        {
            return await _client.DeleteAsync<Sdkwork.Im.Sdk.Generated.Models.DeleteMessageFavoriteResponse>(ApiPaths.ImPath($"/chat/messages/favorites/{SerializePathParameter(favoriteId, new PathParameterSpec("favoriteId", "simple", false))}"));
        }

        /// <summary>
        /// Delete message visibility for the current principal
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessageVisibilityMutationResult?> MessagesVisibilityDeleteAsync(string messageId)
        {
            return await _client.DeleteAsync<Sdkwork.Im.Sdk.Generated.Models.MessageVisibilityMutationResult>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/visibility"));
        }

        /// <summary>
        /// Add a message reaction
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessageReactionMutationResult?> MessagesReactionsCreateAsync(string messageId, Sdkwork.Im.Sdk.Generated.Models.MessageReactionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessageReactionMutationResult>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/reactions"), body, null, null, "application/json");
        }

        /// <summary>
        /// Remove a message reaction
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessageReactionMutationResult?> MessagesReactionsDeleteAsync(string messageId, Sdkwork.Im.Sdk.Generated.Models.MessageReactionRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessageReactionMutationResult>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/reactions/remove"), body, null, null, "application/json");
        }

        /// <summary>
        /// Pin a message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessagePinMutationResult?> MessagesPinCreateAsync(string messageId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessagePinMutationResult>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/pin"), null);
        }

        /// <summary>
        /// Unpin a message
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.MessagePinMutationResult?> MessagesPinDeleteAsync(string messageId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.MessagePinMutationResult>(ApiPaths.ImPath($"/chat/messages/{SerializePathParameter(messageId, new PathParameterSpec("messageId", "simple", false))}/unpin"), null);
        }

        /// <summary>
        /// Create a live, chat, or game room bound to a group conversation
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult?> RoomsCreateAsync(Sdkwork.Im.Sdk.Generated.Models.CreateRoomRequest body)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.CreateConversationResult>(ApiPaths.ImPath("/chat/rooms"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get room metadata and active member count
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.RoomView?> RoomsGetAsync(string roomId)
        {
            return await _client.GetAsync<Sdkwork.Im.Sdk.Generated.Models.RoomView>(ApiPaths.ImPath($"/chat/rooms/{SerializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false))}"));
        }

        /// <summary>
        /// Enter a room as the authenticated principal
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.EnterRoomResponse?> RoomsEnterAsync(string roomId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.EnterRoomResponse>(ApiPaths.ImPath($"/chat/rooms/{SerializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false))}/enter"), null);
        }

        /// <summary>
        /// Leave a room as the authenticated principal
        /// </summary>
        public async Task<Sdkwork.Im.Sdk.Generated.Models.EnterRoomResponse?> RoomsLeaveAsync(string roomId)
        {
            return await _client.PostAsync<Sdkwork.Im.Sdk.Generated.Models.EnterRoomResponse>(ApiPaths.ImPath($"/chat/rooms/{SerializePathParameter(roomId, new PathParameterSpec("roomId", "simple", false))}/leave"), null);
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
