import Foundation

public class ChatApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// List IM contacts
    public func contactsList(limit: Int? = nil, cursor: String? = nil) async throws -> ContactsResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "limit", value: limit, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/contacts"), query), responseType: ContactsResponse.self)
    }

    /// Retrieve current inbox window
    public func inboxRetrieve(limit: Int? = nil, cursor: String? = nil) async throws -> InboxResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "limit", value: limit, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/inbox"), query), responseType: InboxResponse.self)
    }

    /// Create a conversation
    public func conversationsCreate(body: CreateConversationRequest) async throws -> CreateConversationResult? {
        return try await client.post(ApiPaths.imPath("/chat/conversations"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CreateConversationResult.self)
    }

    /// Create an agent dialog
    public func conversationsAgentDialogsCreate(body: CreateAgentDialogRequest) async throws -> CreateConversationResult? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/agent_dialogs"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CreateConversationResult.self)
    }

    /// Create an agent handoff
    public func conversationsAgentHandoffsCreate(body: CreateAgentDialogRequest) async throws -> AckResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/agent_handoffs"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: AckResponse.self)
    }

    /// Create a system channel
    public func conversationsSystemChannelsCreate(body: CreateConversationRequest) async throws -> CreateConversationResult? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/system_channels"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CreateConversationResult.self)
    }

    /// Create a thread conversation
    public func conversationsThreadsCreate(body: CreateConversationRequest) async throws -> CreateConversationResult? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/threads"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CreateConversationResult.self)
    }

    /// Bind a direct chat conversation
    public func conversationsDirectChatsBind(body: BindDirectChatRequest) async throws -> CreateConversationResult? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/direct_chats/bindings"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CreateConversationResult.self)
    }

    /// Retrieve agent handoff state
    public func conversationsAgentHandoffRetrieve(conversationId: String) async throws -> AckResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/agent_handoff"), responseType: AckResponse.self)
    }

    /// Accept agent handoff
    public func conversationsAgentHandoffAccept(conversationId: String) async throws -> AckResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/agent_handoff/accept"), body: nil, responseType: AckResponse.self)
    }

    /// Resolve agent handoff
    public func conversationsAgentHandoffResolve(conversationId: String) async throws -> AckResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/agent_handoff/resolve"), body: nil, responseType: AckResponse.self)
    }

    /// Close agent handoff
    public func conversationsAgentHandoffClose(conversationId: String) async throws -> AckResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/agent_handoff/close"), body: nil, responseType: AckResponse.self)
    }

    /// Retrieve conversation summary
    public func conversationsRetrieve(conversationId: String) async throws -> ConversationSummaryView? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))"), responseType: ConversationSummaryView.self)
    }

    /// List conversation members
    public func conversationsMembersList(conversationId: String, limit: Int? = nil, cursor: String? = nil) async throws -> ListMembersResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "limit", value: limit, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members"), query), responseType: ListMembersResponse.self)
    }

    /// Add a conversation member
    public func conversationsMembersAdd(conversationId: String, body: AddConversationMemberRequest) async throws -> ConversationMember? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/add"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationMember.self)
    }

    /// Remove a conversation member
    public func conversationsMembersRemove(conversationId: String, body: RemoveConversationMemberRequest) async throws -> AckResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/remove"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: AckResponse.self)
    }

    /// Transfer conversation owner
    public func conversationsMembersTransferOwner(conversationId: String, body: TransferConversationOwnerRequest) async throws -> ConversationMember? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/transfer_owner"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationMember.self)
    }

    /// Change conversation member role
    public func conversationsMembersChangeRole(conversationId: String, body: ChangeConversationMemberRoleRequest) async throws -> ConversationMember? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/change_role"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationMember.self)
    }

    /// Leave a conversation
    public func conversationsMembersLeave(conversationId: String) async throws -> AckResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/leave"), body: nil, responseType: AckResponse.self)
    }

    /// Retrieve conversation preferences
    public func conversationsPreferencesRetrieve(conversationId: String) async throws -> ConversationPreferencesView? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/preferences"), responseType: ConversationPreferencesView.self)
    }

    /// Update conversation preferences
    public func conversationsPreferencesUpdate(conversationId: String, body: UpdateConversationPreferencesRequest) async throws -> ConversationPreferencesView? {
        return try await client.patch(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/preferences"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationPreferencesView.self)
    }

    /// Retrieve conversation profile
    public func conversationsProfileRetrieve(conversationId: String) async throws -> ConversationProfileView? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/profile"), responseType: ConversationProfileView.self)
    }

    /// Update conversation profile
    public func conversationsProfileUpdate(conversationId: String, body: UpdateConversationProfileRequest) async throws -> ConversationProfileView? {
        return try await client.patch(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/profile"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationProfileView.self)
    }

    /// Retrieve read cursor
    public func conversationsReadCursorRetrieve(conversationId: String) async throws -> ReadCursorView? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/read_cursor"), responseType: ReadCursorView.self)
    }

    /// Update read cursor
    public func conversationsReadCursorUpdate(conversationId: String, body: UpdateReadCursorRequest) async throws -> ReadCursorView? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/read_cursor"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ReadCursorView.self)
    }

    /// List member directory
    public func conversationsMemberDirectoryList(conversationId: String) async throws -> MemberDirectoryResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/member_directory"), responseType: MemberDirectoryResponse.self)
    }

    /// List conversation message timeline
    public func conversationsMessagesList(conversationId: String, afterSeq: Int? = nil, limit: Int? = nil) async throws -> TimelineResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "afterSeq", value: afterSeq, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "limit", value: limit, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/messages"), query), responseType: TimelineResponse.self)
    }

    /// Post a conversation message
    public func conversationsMessagesCreate(conversationId: String, body: PostMessageRequest) async throws -> PostedMessageResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/messages"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PostedMessageResponse.self)
    }

    /// Publish a system channel message
    public func conversationsSystemChannelPublish(conversationId: String, body: PostMessageRequest) async throws -> PostedMessageResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/system_channel/publish"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PostedMessageResponse.self)
    }

    /// List pinned messages
    public func conversationsPinsList(conversationId: String) async throws -> PinnedMessagesResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/pins"), responseType: PinnedMessagesResponse.self)
    }

    /// Retrieve message interaction summary
    public func conversationsMessagesInteractionSummaryRetrieve(conversationId: String, messageId: String) async throws -> MessageInteractionSummaryView? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/interaction_summary"), responseType: MessageInteractionSummaryView.self)
    }

    /// Edit a message
    public func messagesEdit(messageId: String, body: EditMessageRequest) async throws -> PostedMessageResponse? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/edit"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PostedMessageResponse.self)
    }

    /// Recall a message
    public func messagesRecall(messageId: String) async throws -> PostedMessageResponse? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/recall"), body: nil, responseType: PostedMessageResponse.self)
    }

    /// List message favorites
    public func messagesFavoritesList(limit: Int? = nil, cursor: String? = nil, favoriteType: String? = nil, q: String? = nil) async throws -> FavoriteMessagesResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "limit", value: limit, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "favoriteType", value: favoriteType, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/messages/favorites"), query), responseType: FavoriteMessagesResponse.self)
    }

    /// Favorite a message
    public func messagesFavoritesCreate(messageId: String, body: FavoriteMessageRequest) async throws -> MessageFavoriteView? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/favorites"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MessageFavoriteView.self)
    }

    /// Delete a message favorite
    public func messagesFavoritesDelete(favoriteId: String) async throws -> DeleteMessageFavoriteResponse? {
        return try await client.delete(ApiPaths.imPath("/chat/messages/favorites/\(serializePathParameter(favoriteId, PathParameterSpec(name: "favoriteId", style: "simple", explode: false)))"), responseType: DeleteMessageFavoriteResponse.self)
    }

    /// Delete message visibility for the current principal
    public func messagesVisibilityDelete(messageId: String) async throws -> MessageVisibilityMutationResult? {
        return try await client.delete(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/visibility"), responseType: MessageVisibilityMutationResult.self)
    }

    /// Add a message reaction
    public func messagesReactionsCreate(messageId: String, body: MessageReactionRequest) async throws -> MessageReactionMutationResult? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/reactions"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MessageReactionMutationResult.self)
    }

    /// Remove a message reaction
    public func messagesReactionsDelete(messageId: String, body: MessageReactionRequest) async throws -> MessageReactionMutationResult? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/reactions/remove"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MessageReactionMutationResult.self)
    }

    /// Pin a message
    public func messagesPinCreate(messageId: String) async throws -> MessagePinMutationResult? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/pin"), body: nil, responseType: MessagePinMutationResult.self)
    }

    /// Unpin a message
    public func messagesPinDelete(messageId: String) async throws -> MessagePinMutationResult? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/unpin"), body: nil, responseType: MessagePinMutationResult.self)
    }

    /// Create a live, chat, or game room bound to a group conversation
    public func roomsCreate(body: CreateRoomRequest) async throws -> CreateConversationResult? {
        return try await client.post(ApiPaths.imPath("/chat/rooms"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: CreateConversationResult.self)
    }

    /// Get room metadata and active member count
    public func roomsGet(roomId: String) async throws -> RoomView? {
        return try await client.get(ApiPaths.imPath("/chat/rooms/\(serializePathParameter(roomId, PathParameterSpec(name: "roomId", style: "simple", explode: false)))"), responseType: RoomView.self)
    }

    /// Enter a room as the authenticated principal
    public func roomsEnter(roomId: String) async throws -> EnterRoomResponse? {
        return try await client.post(ApiPaths.imPath("/chat/rooms/\(serializePathParameter(roomId, PathParameterSpec(name: "roomId", style: "simple", explode: false)))/enter"), body: nil, responseType: EnterRoomResponse.self)
    }

    /// Leave a room as the authenticated principal
    public func roomsLeave(roomId: String) async throws -> EnterRoomResponse? {
        return try await client.post(ApiPaths.imPath("/chat/rooms/\(serializePathParameter(roomId, PathParameterSpec(name: "roomId", style: "simple", explode: false)))/leave"), body: nil, responseType: EnterRoomResponse.self)
    }

    private struct PathParameterSpec {
        let name: String
        let style: String
        let explode: Bool
    }

    private func serializePathParameter(_ value: Any?, _ spec: PathParameterSpec) -> String {
        guard let value else { return "" }
        let style = spec.style.isEmpty ? "simple" : spec.style
        if let array = value as? [Any] {
            return serializePathArray(spec.name, array, style, spec.explode)
        }
        if let object = value as? [String: Any] {
            return serializePathObject(spec.name, object, style, spec.explode)
        }
        return pathPrimitivePrefix(spec.name, style) + pathEncode(String(describing: value))
    }

    private func serializePathArray(_ name: String, _ values: [Any], _ style: String, _ explode: Bool) -> String {
        let serialized = values.map { pathEncode(String(describing: $0)) }
        if serialized.isEmpty { return pathPrefix(name, style) }
        if style == "matrix" {
            if explode {
                return serialized.map { ";\(name)=\($0)" }.joined()
            }
            return ";\(name)=" + serialized.joined(separator: ",")
        }
        let separator = explode ? "." : ","
        return pathPrefix(name, style) + serialized.joined(separator: separator)
    }

    private func serializePathObject(_ name: String, _ values: [String: Any], _ style: String, _ explode: Bool) -> String {
        var entries: [String] = []
        var exploded: [String] = []
        for (key, value) in values {
            let escapedKey = pathEncode(key)
            let escapedValue = pathEncode(String(describing: value))
            if explode {
                if style == "matrix" {
                    exploded.append(";\(escapedKey)=\(escapedValue)")
                } else {
                    exploded.append("\(escapedKey)=\(escapedValue)")
                }
            } else {
                entries.append(escapedKey)
                entries.append(escapedValue)
            }
        }
        if style == "matrix" {
            if explode {
                return exploded.joined()
            }
            return ";\(name)=" + entries.joined(separator: ",")
        }
        if explode {
            let separator = style == "label" ? "." : ","
            return pathPrefix(name, style) + exploded.joined(separator: separator)
        }
        return pathPrefix(name, style) + entries.joined(separator: ",")
    }

    private func pathPrefix(_ name: String, _ style: String) -> String {
        if style == "label" { return "." }
        if style == "matrix" { return ";\(name)" }
        return ""
    }

    private func pathPrimitivePrefix(_ name: String, _ style: String) -> String {
        style == "matrix" ? ";\(name)=" : pathPrefix(name, style)
    }

    private func pathEncode(_ value: String) -> String {
        value.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? value
    }

    private struct QueryParameterSpec {
        let name: String
        let value: Any?
        let style: String
        let explode: Bool
        let allowReserved: Bool
        let contentType: String?
    }

    private func buildQueryString(_ parameters: [QueryParameterSpec]) -> String {
        var pairs: [String] = []
        for parameter in parameters {
            appendSerializedParameter(&pairs, parameter)
        }
        return pairs.joined(separator: "&")
    }

    private func appendSerializedParameter(_ pairs: inout [String], _ parameter: QueryParameterSpec) {
        guard let value = parameter.value else { return }
        if let contentType = parameter.contentType, !contentType.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
            let data = (try? JSONSerialization.data(withJSONObject: value, options: [])) ?? Data(String(describing: value).utf8)
            let json = String(data: data, encoding: .utf8) ?? String(describing: value)
            pairs.append("\(urlEncode(parameter.name))=\(encodeQueryValue(json, allowReserved: parameter.allowReserved))")
            return
        }

        let style = parameter.style.isEmpty ? "form" : parameter.style
        if style == "deepObject", let object = value as? [String: Any] {
            appendDeepObjectParameter(&pairs, name: parameter.name, values: object, allowReserved: parameter.allowReserved)
        } else if let array = value as? [Any] {
            appendArrayParameter(&pairs, name: parameter.name, values: array, style: style, explode: parameter.explode, allowReserved: parameter.allowReserved)
        } else if let object = value as? [String: Any] {
            appendObjectParameter(&pairs, name: parameter.name, values: object, style: style, explode: parameter.explode, allowReserved: parameter.allowReserved)
        } else {
            pairs.append("\(urlEncode(parameter.name))=\(encodeQueryValue(String(describing: value), allowReserved: parameter.allowReserved))")
        }
    }

    private func appendArrayParameter(
        _ pairs: inout [String],
        name: String,
        values: [Any],
        style: String,
        explode: Bool,
        allowReserved: Bool
    ) {
        let serialized = values.map { String(describing: $0) }
        guard !serialized.isEmpty else { return }
        if style == "form" && explode {
            for item in serialized {
                pairs.append("\(urlEncode(name))=\(encodeQueryValue(item, allowReserved: allowReserved))")
            }
            return
        }
        pairs.append("\(urlEncode(name))=\(encodeQueryValue(serialized.joined(separator: ","), allowReserved: allowReserved))")
    }

    private func appendObjectParameter(
        _ pairs: inout [String],
        name: String,
        values: [String: Any],
        style: String,
        explode: Bool,
        allowReserved: Bool
    ) {
        var serialized: [String] = []
        for (key, value) in values {
            if style == "form" && explode {
                pairs.append("\(urlEncode(key))=\(encodeQueryValue(String(describing: value), allowReserved: allowReserved))")
            } else {
                serialized.append(key)
                serialized.append(String(describing: value))
            }
        }
        if !serialized.isEmpty {
            pairs.append("\(urlEncode(name))=\(encodeQueryValue(serialized.joined(separator: ","), allowReserved: allowReserved))")
        }
    }

    private func appendDeepObjectParameter(_ pairs: inout [String], name: String, values: [String: Any], allowReserved: Bool) {
        for (key, value) in values {
            pairs.append("\(urlEncode("\(name)[\(key)]"))=\(encodeQueryValue(String(describing: value), allowReserved: allowReserved))")
        }
    }

    private func encodeQueryValue(_ value: String, allowReserved: Bool) -> String {
        var encoded = urlEncode(value)
        if !allowReserved { return encoded }
        [
            "%3A": ":", "%2F": "/", "%3F": "?", "%23": "#",
            "%5B": "[", "%5D": "]", "%40": "@", "%21": "!",
            "%24": "$", "%26": "&", "%27": "'", "%28": "(",
            "%29": ")", "%2A": "*", "%2B": "+", "%2C": ",",
            "%3B": ";", "%3D": "=",
        ].forEach { encoded = encoded.replacingOccurrences(of: $0.key, with: $0.value) }
        return encoded
    }

    private func urlEncode(_ value: String) -> String {
        value.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? value
    }

}
