import Foundation

public class ChatApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// List IM contacts
    public func contactsList(pageSize: Int? = nil, cursor: String? = nil) async throws -> ContactsListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/contacts"), query), responseType: ContactsListResponse.self)
    }

    /// List current inbox window
    public func inboxList(pageSize: Int? = nil, cursor: String? = nil) async throws -> InboxListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/inbox"), query), responseType: InboxListResponse.self)
    }

    /// Create a conversation
    public func conversationsCreate(body: CreateConversationRequest) async throws -> ConversationsCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/chat/conversations"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsCreateResponse201.self)
    }

    /// Create an agent dialog
    public func conversationsAgentDialogsCreate(body: CreateAgentDialogRequest) async throws -> ConversationsAgentDialogsCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/agent_dialogs"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsAgentDialogsCreateResponse201.self)
    }

    /// Create an agent handoff
    public func conversationsAgentHandoffsCreate(body: CreateAgentHandoffRequest) async throws -> ConversationsAgentHandoffsCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/agent_handoffs"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsAgentHandoffsCreateResponse201.self)
    }

    /// Create a system channel
    public func conversationsSystemChannelsCreate(body: CreateSystemChannelRequest) async throws -> ConversationsSystemChannelsCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/system_channels"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsSystemChannelsCreateResponse201.self)
    }

    /// Create a thread conversation
    public func conversationsThreadsCreate(body: CreateThreadConversationRequest) async throws -> ConversationsThreadsCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/threads"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsThreadsCreateResponse201.self)
    }

    /// Create a direct chat conversation binding
    public func conversationsDirectChatsBindingsCreate(body: BindDirectChatRequest) async throws -> ConversationsDirectChatsBindingsCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/direct_chats/bindings"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsDirectChatsBindingsCreateResponse201.self)
    }

    /// Retrieve agent handoff state
    public func conversationsAgentHandoffRetrieve(conversationId: String) async throws -> ConversationsAgentHandoffRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/agent_handoff"), responseType: ConversationsAgentHandoffRetrieveResponse.self)
    }

    /// Accept agent handoff
    public func conversationsAgentHandoffAccept(conversationId: String) async throws -> ConversationsAgentHandoffAcceptResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/agent_handoff/accept"), body: nil, responseType: ConversationsAgentHandoffAcceptResponse.self)
    }

    /// Resolve agent handoff
    public func conversationsAgentHandoffResolve(conversationId: String) async throws -> ConversationsAgentHandoffResolveResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/agent_handoff/resolve"), body: nil, responseType: ConversationsAgentHandoffResolveResponse.self)
    }

    /// Close agent handoff
    public func conversationsAgentHandoffClose(conversationId: String) async throws -> ConversationsAgentHandoffCloseResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/agent_handoff/close"), body: nil, responseType: ConversationsAgentHandoffCloseResponse.self)
    }

    /// Retrieve conversation summary
    public func conversationsRetrieve(conversationId: String) async throws -> ConversationsRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))"), responseType: ConversationsRetrieveResponse.self)
    }

    /// List conversation members
    public func conversationsMembersList(conversationId: String, pageSize: Int? = nil, cursor: String? = nil) async throws -> ConversationsMembersListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members"), query), responseType: ConversationsMembersListResponse.self)
    }

    /// Add a conversation member
    public func conversationsMembersAdd(conversationId: String, body: AddConversationMemberRequest) async throws -> ConversationsMembersAddResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/add"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsMembersAddResponse.self)
    }

    /// Remove a conversation member
    public func conversationsMembersRemove(conversationId: String, body: RemoveConversationMemberRequest) async throws -> ConversationsMembersRemoveResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/remove"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsMembersRemoveResponse.self)
    }

    /// Transfer conversation owner
    public func conversationsMembersTransferOwner(conversationId: String, body: TransferConversationOwnerRequest) async throws -> ConversationsMembersTransferOwnerResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/transfer_owner"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsMembersTransferOwnerResponse.self)
    }

    /// Change conversation member role
    public func conversationsMembersChangeRole(conversationId: String, body: ChangeConversationMemberRoleRequest) async throws -> ConversationsMembersChangeRoleResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/change_role"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsMembersChangeRoleResponse.self)
    }

    /// Leave a conversation
    public func conversationsMembersLeave(conversationId: String) async throws -> ConversationsMembersLeaveResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/leave"), body: nil, responseType: ConversationsMembersLeaveResponse.self)
    }

    /// Accept a conversation invitation
    public func conversationsMembersAcceptInvitation(conversationId: String) async throws -> ConversationsMembersAcceptInvitationResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/members/accept_invitation"), body: nil, responseType: ConversationsMembersAcceptInvitationResponse.self)
    }

    /// Retrieve conversation preferences
    public func conversationsPreferencesRetrieve(conversationId: String) async throws -> ConversationsPreferencesRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/preferences"), responseType: ConversationsPreferencesRetrieveResponse.self)
    }

    /// Update conversation preferences
    public func conversationsPreferencesUpdate(conversationId: String, body: UpdateConversationPreferencesRequest) async throws -> ConversationsPreferencesUpdateResponse? {
        return try await client.patch(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/preferences"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsPreferencesUpdateResponse.self)
    }

    /// Retrieve conversation profile
    public func conversationsProfileRetrieve(conversationId: String) async throws -> ConversationsProfileRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/profile"), responseType: ConversationsProfileRetrieveResponse.self)
    }

    /// Update conversation profile
    public func conversationsProfileUpdate(conversationId: String, body: UpdateConversationProfileRequest) async throws -> ConversationsProfileUpdateResponse? {
        return try await client.patch(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/profile"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsProfileUpdateResponse.self)
    }

    /// Retrieve read cursor
    public func conversationsReadCursorRetrieve(conversationId: String) async throws -> ConversationsReadCursorRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/read_cursor"), responseType: ConversationsReadCursorRetrieveResponse.self)
    }

    /// Update read cursor
    public func conversationsReadCursorUpdate(conversationId: String, body: UpdateReadCursorRequest) async throws -> ConversationsReadCursorUpdateResponse? {
        return try await client.patch(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/read_cursor"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsReadCursorUpdateResponse.self)
    }

    /// List member directory
    public func conversationsMemberDirectoryList(conversationId: String) async throws -> ConversationsMemberDirectoryListResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/member_directory"), responseType: ConversationsMemberDirectoryListResponse.self)
    }

    /// List conversation message history
    public func conversationsMessagesList(conversationId: String, afterSeq: Int? = nil, pageSize: Int? = nil) async throws -> ConversationMessageListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "afterSeq", value: afterSeq, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/messages"), query), responseType: ConversationMessageListResponse.self)
    }

    /// Post a conversation message
    public func conversationsMessagesCreate(conversationId: String, body: PostMessageRequest) async throws -> ConversationsMessagesCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/messages"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsMessagesCreateResponse201.self)
    }

    /// Publish a system channel message
    public func conversationsSystemChannelPublish(conversationId: String, body: PostMessageRequest) async throws -> ConversationsSystemChannelPublishResponse? {
        return try await client.post(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/system_channel/publish"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: ConversationsSystemChannelPublishResponse.self)
    }

    /// List pinned messages
    public func conversationsPinsList(conversationId: String) async throws -> ConversationsPinsListResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/pins"), responseType: ConversationsPinsListResponse.self)
    }

    /// Retrieve message interaction summary
    public func conversationsMessagesInteractionSummaryRetrieve(conversationId: String, messageId: String) async throws -> ConversationsMessagesInteractionSummaryRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/chat/conversations/\(serializePathParameter(conversationId, PathParameterSpec(name: "conversationId", style: "simple", explode: false)))/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/interaction_summary"), responseType: ConversationsMessagesInteractionSummaryRetrieveResponse.self)
    }

    /// Edit a message
    public func messagesEdit(messageId: String, body: EditMessageRequest) async throws -> MessagesEditResponse? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/edit"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MessagesEditResponse.self)
    }

    /// Recall a message
    public func messagesRecall(messageId: String, body: RecallMessageRequest) async throws -> MessagesRecallResponse? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/recall"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MessagesRecallResponse.self)
    }

    /// List message favorites
    public func messagesFavoritesList(pageSize: Int? = nil, cursor: String? = nil, favoriteType: String? = nil, q: String? = nil) async throws -> MessagesFavoritesListResponse? {
        let query = buildQueryString([
            QueryParameterSpec(name: "page_size", value: pageSize, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "cursor", value: cursor, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "favoriteType", value: favoriteType, style: "form", explode: true, allowReserved: false, contentType: nil),
            QueryParameterSpec(name: "q", value: q, style: "form", explode: true, allowReserved: false, contentType: nil)
        ])
        return try await client.get(ApiPaths.appendQueryString(ApiPaths.imPath("/chat/messages/favorites"), query), responseType: MessagesFavoritesListResponse.self)
    }

    /// Favorite a message
    public func messagesFavoritesCreate(messageId: String, body: FavoriteMessageRequest) async throws -> MessagesFavoritesCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/favorites"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MessagesFavoritesCreateResponse201.self)
    }

    /// Delete a message favorite
    public func messagesFavoritesDelete(favoriteId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/chat/messages/favorites/\(serializePathParameter(favoriteId, PathParameterSpec(name: "favoriteId", style: "simple", explode: false)))"))
    }

    /// Delete message visibility for the current principal
    public func messagesVisibilityDelete(messageId: String) async throws -> Void {
        _ = try await client.delete(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/visibility"))
    }

    /// Add a message reaction
    public func messagesReactionsCreate(messageId: String, body: MessageReactionRequest) async throws -> MessagesReactionsCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/reactions"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MessagesReactionsCreateResponse201.self)
    }

    /// Remove a message reaction
    public func messagesReactionsRemove(messageId: String, body: MessageReactionRequest) async throws -> MessagesReactionsRemoveResponse? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/reactions/remove"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: MessagesReactionsRemoveResponse.self)
    }

    /// Pin a message
    public func messagesPin(messageId: String) async throws -> MessagesPinResponse? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/pin"), body: nil, responseType: MessagesPinResponse.self)
    }

    /// Unpin a message
    public func messagesUnpin(messageId: String) async throws -> MessagesUnpinResponse? {
        return try await client.post(ApiPaths.imPath("/chat/messages/\(serializePathParameter(messageId, PathParameterSpec(name: "messageId", style: "simple", explode: false)))/unpin"), body: nil, responseType: MessagesUnpinResponse.self)
    }

    /// Create a live, chat, or game room bound to a group conversation
    public func roomsCreate(body: CreateRoomRequest) async throws -> RoomsCreateResponse201? {
        return try await client.post(ApiPaths.imPath("/chat/rooms"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: RoomsCreateResponse201.self)
    }

    /// Retrieve room metadata and active member count
    public func roomsRetrieve(roomId: String) async throws -> RoomsRetrieveResponse? {
        return try await client.get(ApiPaths.imPath("/chat/rooms/\(serializePathParameter(roomId, PathParameterSpec(name: "roomId", style: "simple", explode: false)))"), responseType: RoomsRetrieveResponse.self)
    }

    /// Enter a room as the authenticated principal
    public func roomsEnter(roomId: String) async throws -> RoomsEnterResponse? {
        return try await client.post(ApiPaths.imPath("/chat/rooms/\(serializePathParameter(roomId, PathParameterSpec(name: "roomId", style: "simple", explode: false)))/enter"), body: nil, responseType: RoomsEnterResponse.self)
    }

    /// Leave a room as the authenticated principal
    public func roomsLeave(roomId: String) async throws -> RoomsLeaveResponse? {
        return try await client.post(ApiPaths.imPath("/chat/rooms/\(serializePathParameter(roomId, PathParameterSpec(name: "roomId", style: "simple", explode: false)))/leave"), body: nil, responseType: RoomsLeaveResponse.self)
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
