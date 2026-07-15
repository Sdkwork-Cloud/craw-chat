package api

import (
    "encoding/json"
    "fmt"
    "net/url"
    "strings"
    sdktypes "github.com/sdkwork/im-sdk-generated/types"
    sdkhttp "github.com/sdkwork/im-sdk-generated/http"
)

type ChatApi struct {
    client *sdkhttp.Client
}

func NewChatApi(client *sdkhttp.Client) *ChatApi {
    return &ChatApi{client: client}
}

// List IM contacts
func (a *ChatApi) ContactsList(pageSize *int, cursor *string) (sdktypes.ContactsListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath("/chat/contacts"), query), nil, nil)
    if err != nil {
        var zero sdktypes.ContactsListResponse
        return zero, err
    }
    return decodeResult[sdktypes.ContactsListResponse](raw)
}

// List current inbox window
func (a *ChatApi) InboxList(pageSize *int, cursor *string, conversationType *string) (sdktypes.InboxListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "conversation_type", Value: func() interface{} { if conversationType == nil { return nil }; return *conversationType }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath("/chat/inbox"), query), nil, nil)
    if err != nil {
        var zero sdktypes.InboxListResponse
        return zero, err
    }
    return decodeResult[sdktypes.InboxListResponse](raw)
}

// Create a conversation
func (a *ChatApi) ConversationsCreate(body sdktypes.CreateConversationRequest) (sdktypes.ConversationsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/chat/conversations"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsCreateResponse201](raw)
}

// Create an agent dialog
func (a *ChatApi) ConversationsAgentDialogsCreate(body sdktypes.CreateAgentDialogRequest) (sdktypes.ConversationsAgentDialogsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/chat/conversations/agent_dialogs"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsAgentDialogsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsAgentDialogsCreateResponse201](raw)
}

// Create an agent handoff
func (a *ChatApi) ConversationsAgentHandoffsCreate(body sdktypes.CreateAgentHandoffRequest) (sdktypes.ConversationsAgentHandoffsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/chat/conversations/agent_handoffs"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsAgentHandoffsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsAgentHandoffsCreateResponse201](raw)
}

// Create a system channel
func (a *ChatApi) ConversationsSystemChannelsCreate(body sdktypes.CreateSystemChannelRequest) (sdktypes.ConversationsSystemChannelsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/chat/conversations/system_channels"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsSystemChannelsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsSystemChannelsCreateResponse201](raw)
}

// Create a thread conversation
func (a *ChatApi) ConversationsThreadsCreate(body sdktypes.CreateThreadConversationRequest) (sdktypes.ConversationsThreadsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/chat/conversations/threads"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsThreadsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsThreadsCreateResponse201](raw)
}

// Create a direct chat conversation binding
func (a *ChatApi) ConversationsDirectChatsBindingsCreate(body sdktypes.BindDirectChatRequest) (sdktypes.ConversationsDirectChatsBindingsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/chat/conversations/direct_chats/bindings"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsDirectChatsBindingsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsDirectChatsBindingsCreateResponse201](raw)
}

// Retrieve agent handoff state
func (a *ChatApi) ConversationsAgentHandoffRetrieve(conversationId string) (sdktypes.ConversationsAgentHandoffRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/chat/conversations/%s/agent_handoff", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsAgentHandoffRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsAgentHandoffRetrieveResponse](raw)
}

// Accept agent handoff
func (a *ChatApi) ConversationsAgentHandoffAccept(conversationId string) (sdktypes.ConversationsAgentHandoffAcceptResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/agent_handoff/accept", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.ConversationsAgentHandoffAcceptResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsAgentHandoffAcceptResponse](raw)
}

// Resolve agent handoff
func (a *ChatApi) ConversationsAgentHandoffResolve(conversationId string) (sdktypes.ConversationsAgentHandoffResolveResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/agent_handoff/resolve", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.ConversationsAgentHandoffResolveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsAgentHandoffResolveResponse](raw)
}

// Close agent handoff
func (a *ChatApi) ConversationsAgentHandoffClose(conversationId string) (sdktypes.ConversationsAgentHandoffCloseResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/agent_handoff/close", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.ConversationsAgentHandoffCloseResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsAgentHandoffCloseResponse](raw)
}

// Retrieve conversation summary
func (a *ChatApi) ConversationsRetrieve(conversationId string) (sdktypes.ConversationsRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/chat/conversations/%s", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsRetrieveResponse](raw)
}

// List conversation members
func (a *ChatApi) ConversationsMembersList(conversationId string, pageSize *int, cursor *string) (sdktypes.ConversationsMembersListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/chat/conversations/%s/members", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsMembersListResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMembersListResponse](raw)
}

// Retrieve the current conversation member
func (a *ChatApi) ConversationsMembersCurrentRetrieve(conversationId string) (sdktypes.ConversationsMembersCurrentRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/chat/conversations/%s/members/current", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsMembersCurrentRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMembersCurrentRetrieveResponse](raw)
}

// Retrieve assigned group agents
func (a *ChatApi) ConversationsAgentsRetrieve(conversationId string) (sdktypes.ConversationsAgentsRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/chat/conversations/%s/agents", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsAgentsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsAgentsRetrieveResponse](raw)
}

// Update assigned group agents
func (a *ChatApi) ConversationsAgentsUpdate(conversationId string, body sdktypes.UpdateConversationAgentsRequest) (sdktypes.ConversationsAgentsUpdateResponse, error) {
    raw, err := a.client.Put(ImApiPath(fmt.Sprintf("/chat/conversations/%s/agents", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsAgentsUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsAgentsUpdateResponse](raw)
}

// Add a conversation member
func (a *ChatApi) ConversationsMembersAdd(conversationId string, body sdktypes.AddConversationMemberRequest) (sdktypes.ConversationsMembersAddResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/members/add", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsMembersAddResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMembersAddResponse](raw)
}

// Remove a conversation member
func (a *ChatApi) ConversationsMembersRemove(conversationId string, body sdktypes.RemoveConversationMemberRequest) (sdktypes.ConversationsMembersRemoveResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/members/remove", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsMembersRemoveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMembersRemoveResponse](raw)
}

// Transfer conversation owner
func (a *ChatApi) ConversationsMembersTransferOwner(conversationId string, body sdktypes.TransferConversationOwnerRequest) (sdktypes.ConversationsMembersTransferOwnerResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/members/transfer_owner", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsMembersTransferOwnerResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMembersTransferOwnerResponse](raw)
}

// Change conversation member role
func (a *ChatApi) ConversationsMembersChangeRole(conversationId string, body sdktypes.ChangeConversationMemberRoleRequest) (sdktypes.ConversationsMembersChangeRoleResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/members/change_role", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsMembersChangeRoleResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMembersChangeRoleResponse](raw)
}

// Leave a conversation
func (a *ChatApi) ConversationsMembersLeave(conversationId string) (sdktypes.ConversationsMembersLeaveResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/members/leave", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.ConversationsMembersLeaveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMembersLeaveResponse](raw)
}

// Accept a conversation invitation
func (a *ChatApi) ConversationsMembersAcceptInvitation(conversationId string) (sdktypes.ConversationsMembersAcceptInvitationResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/members/accept_invitation", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.ConversationsMembersAcceptInvitationResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMembersAcceptInvitationResponse](raw)
}

// Retrieve conversation preferences
func (a *ChatApi) ConversationsPreferencesRetrieve(conversationId string) (sdktypes.ConversationsPreferencesRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/chat/conversations/%s/preferences", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsPreferencesRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsPreferencesRetrieveResponse](raw)
}

// Update conversation preferences
func (a *ChatApi) ConversationsPreferencesUpdate(conversationId string, body sdktypes.UpdateConversationPreferencesRequest) (sdktypes.ConversationsPreferencesUpdateResponse, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/chat/conversations/%s/preferences", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsPreferencesUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsPreferencesUpdateResponse](raw)
}

// Retrieve conversation profile
func (a *ChatApi) ConversationsProfileRetrieve(conversationId string) (sdktypes.ConversationsProfileRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/chat/conversations/%s/profile", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsProfileRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsProfileRetrieveResponse](raw)
}

// Update conversation profile
func (a *ChatApi) ConversationsProfileUpdate(conversationId string, body sdktypes.UpdateConversationProfileRequest) (sdktypes.ConversationsProfileUpdateResponse, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/chat/conversations/%s/profile", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsProfileUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsProfileUpdateResponse](raw)
}

// Retrieve read cursor
func (a *ChatApi) ConversationsReadCursorRetrieve(conversationId string) (sdktypes.ConversationsReadCursorRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/chat/conversations/%s/read_cursor", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsReadCursorRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsReadCursorRetrieveResponse](raw)
}

// Update read cursor
func (a *ChatApi) ConversationsReadCursorUpdate(conversationId string, body sdktypes.UpdateReadCursorRequest) (sdktypes.ConversationsReadCursorUpdateResponse, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/chat/conversations/%s/read_cursor", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsReadCursorUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsReadCursorUpdateResponse](raw)
}

// List member directory
func (a *ChatApi) ConversationsMemberDirectoryList(conversationId string, cursor *string, pageSize *int) (sdktypes.ConversationsMemberDirectoryListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/chat/conversations/%s/member_directory", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsMemberDirectoryListResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMemberDirectoryListResponse](raw)
}

// List conversation message history
func (a *ChatApi) ConversationsMessagesList(conversationId string, cursor *string, pageSize *int) (sdktypes.ConversationMessageListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/chat/conversations/%s/messages", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationMessageListResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationMessageListResponse](raw)
}

// Post a conversation message
func (a *ChatApi) ConversationsMessagesCreate(conversationId string, body sdktypes.PostMessageRequest) (sdktypes.ConversationsMessagesCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/messages", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsMessagesCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMessagesCreateResponse201](raw)
}

// Publish a system channel message
func (a *ChatApi) ConversationsSystemChannelPublish(conversationId string, body sdktypes.PostMessageRequest) (sdktypes.ConversationsSystemChannelPublishResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/conversations/%s/system_channel/publish", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.ConversationsSystemChannelPublishResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsSystemChannelPublishResponse](raw)
}

// List pinned messages
func (a *ChatApi) ConversationsPinsList(conversationId string, cursor *string, pageSize *int) (sdktypes.ConversationsPinsListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/chat/conversations/%s/pins", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsPinsListResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsPinsListResponse](raw)
}

// Retrieve message interaction summary
func (a *ChatApi) ConversationsMessagesInteractionSummaryRetrieve(conversationId string, messageId string) (sdktypes.ConversationsMessagesInteractionSummaryRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/chat/conversations/%s/messages/%s/interaction_summary", SerializePathParameter(conversationId, PathParameterSpec{Name: "conversationId", Style: "simple", Explode: false}), SerializePathParameter(messageId, PathParameterSpec{Name: "messageId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.ConversationsMessagesInteractionSummaryRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ConversationsMessagesInteractionSummaryRetrieveResponse](raw)
}

// Edit a message
func (a *ChatApi) MessagesEdit(messageId string, body sdktypes.EditMessageRequest) (sdktypes.MessagesEditResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/messages/%s/edit", SerializePathParameter(messageId, PathParameterSpec{Name: "messageId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.MessagesEditResponse
        return zero, err
    }
    return decodeResult[sdktypes.MessagesEditResponse](raw)
}

// Recall a message
func (a *ChatApi) MessagesRecall(messageId string, body sdktypes.RecallMessageRequest) (sdktypes.MessagesRecallResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/messages/%s/recall", SerializePathParameter(messageId, PathParameterSpec{Name: "messageId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.MessagesRecallResponse
        return zero, err
    }
    return decodeResult[sdktypes.MessagesRecallResponse](raw)
}

// List message favorites
func (a *ChatApi) MessagesFavoritesList(pageSize *int, cursor *string, favoriteType *sdktypes.MessageFavoriteType, q *string) (sdktypes.MessagesFavoritesListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "favoriteType", Value: func() interface{} { if favoriteType == nil { return nil }; return *favoriteType }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath("/chat/messages/favorites"), query), nil, nil)
    if err != nil {
        var zero sdktypes.MessagesFavoritesListResponse
        return zero, err
    }
    return decodeResult[sdktypes.MessagesFavoritesListResponse](raw)
}

// Favorite a message
func (a *ChatApi) MessagesFavoritesCreate(messageId string, body sdktypes.FavoriteMessageRequest) (sdktypes.MessagesFavoritesCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/messages/%s/favorites", SerializePathParameter(messageId, PathParameterSpec{Name: "messageId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.MessagesFavoritesCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.MessagesFavoritesCreateResponse201](raw)
}

// Delete a message favorite
func (a *ChatApi) MessagesFavoritesDelete(favoriteId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/chat/messages/favorites/%s", SerializePathParameter(favoriteId, PathParameterSpec{Name: "favoriteId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// Delete message visibility for the current principal
func (a *ChatApi) MessagesVisibilityDelete(messageId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/chat/messages/%s/visibility", SerializePathParameter(messageId, PathParameterSpec{Name: "messageId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// Add a message reaction
func (a *ChatApi) MessagesReactionsCreate(messageId string, body sdktypes.MessageReactionRequest) (sdktypes.MessagesReactionsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/messages/%s/reactions", SerializePathParameter(messageId, PathParameterSpec{Name: "messageId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.MessagesReactionsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.MessagesReactionsCreateResponse201](raw)
}

// Remove a message reaction
func (a *ChatApi) MessagesReactionsRemove(messageId string, body sdktypes.MessageReactionRequest) (sdktypes.MessagesReactionsRemoveResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/messages/%s/reactions/remove", SerializePathParameter(messageId, PathParameterSpec{Name: "messageId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.MessagesReactionsRemoveResponse
        return zero, err
    }
    return decodeResult[sdktypes.MessagesReactionsRemoveResponse](raw)
}

// Pin a message
func (a *ChatApi) MessagesPin(messageId string) (sdktypes.MessagesPinResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/messages/%s/pin", SerializePathParameter(messageId, PathParameterSpec{Name: "messageId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.MessagesPinResponse
        return zero, err
    }
    return decodeResult[sdktypes.MessagesPinResponse](raw)
}

// Unpin a message
func (a *ChatApi) MessagesUnpin(messageId string) (sdktypes.MessagesUnpinResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/messages/%s/unpin", SerializePathParameter(messageId, PathParameterSpec{Name: "messageId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.MessagesUnpinResponse
        return zero, err
    }
    return decodeResult[sdktypes.MessagesUnpinResponse](raw)
}

// Create a live, chat, or game room bound to a group conversation
func (a *ChatApi) RoomsCreate(body sdktypes.CreateRoomRequest) (sdktypes.RoomsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/chat/rooms"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RoomsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.RoomsCreateResponse201](raw)
}

// Retrieve room metadata and active member count
func (a *ChatApi) RoomsRetrieve(roomId string) (sdktypes.RoomsRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/chat/rooms/%s", SerializePathParameter(roomId, PathParameterSpec{Name: "roomId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.RoomsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.RoomsRetrieveResponse](raw)
}

// Enter a room as the authenticated principal
func (a *ChatApi) RoomsEnter(roomId string) (sdktypes.RoomsEnterResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/rooms/%s/enter", SerializePathParameter(roomId, PathParameterSpec{Name: "roomId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.RoomsEnterResponse
        return zero, err
    }
    return decodeResult[sdktypes.RoomsEnterResponse](raw)
}

// Leave a room as the authenticated principal
func (a *ChatApi) RoomsLeave(roomId string) (sdktypes.RoomsLeaveResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/chat/rooms/%s/leave", SerializePathParameter(roomId, PathParameterSpec{Name: "roomId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.RoomsLeaveResponse
        return zero, err
    }
    return decodeResult[sdktypes.RoomsLeaveResponse](raw)
}

type PathParameterSpec struct {
    Name    string
    Style   string
    Explode bool
}

func SerializePathParameter(value interface{}, spec PathParameterSpec) string {
    if value == nil {
        return ""
    }
    style := spec.Style
    if style == "" {
        style = "simple"
    }

    switch typed := value.(type) {
    case []string:
        return SerializePathArray(spec.Name, stringSliceToInterface(typed), style, spec.Explode)
    case []int:
        return SerializePathArray(spec.Name, intSliceToInterface(typed), style, spec.Explode)
    case []interface{}:
        return SerializePathArray(spec.Name, typed, style, spec.Explode)
    case map[string]string:
        return SerializePathObject(spec.Name, stringMapToInterface(typed), style, spec.Explode)
    case map[string]int:
        return SerializePathObject(spec.Name, intMapToInterface(typed), style, spec.Explode)
    case map[string]interface{}:
        return SerializePathObject(spec.Name, typed, style, spec.Explode)
    default:
        return PathPrefix(spec.Name, style) + url.PathEscape(fmt.Sprint(value))
    }
}

func SerializePathArray(name string, values []interface{}, style string, explode bool) string {
    serialized := make([]string, 0, len(values))
    for _, item := range values {
        if item != nil {
            serialized = append(serialized, url.PathEscape(fmt.Sprint(item)))
        }
    }
    if len(serialized) == 0 {
        return PathPrefix(name, style)
    }
    if style == "matrix" {
        if explode {
            parts := make([]string, 0, len(serialized))
            for _, item := range serialized {
                parts = append(parts, ";"+name+"="+item)
            }
            return strings.Join(parts, "")
        }
        return ";" + name + "=" + strings.Join(serialized, ",")
    }
    separator := ","
    if explode {
        separator = "."
    }
    return PathPrefix(name, style) + strings.Join(serialized, separator)
}

func SerializePathObject(name string, values map[string]interface{}, style string, explode bool) string {
    entries := make([]string, 0, len(values)*2)
    exploded := make([]string, 0, len(values))
    for key, value := range values {
        if value == nil {
            continue
        }
        escapedKey := url.PathEscape(key)
        escapedValue := url.PathEscape(fmt.Sprint(value))
        if explode {
            if style == "matrix" {
                exploded = append(exploded, ";"+escapedKey+"="+escapedValue)
            } else {
                exploded = append(exploded, escapedKey+"="+escapedValue)
            }
        } else {
            entries = append(entries, escapedKey, escapedValue)
        }
    }
    if style == "matrix" {
        if explode {
            return strings.Join(exploded, "")
        }
        return ";" + name + "=" + strings.Join(entries, ",")
    }
    if explode {
        separator := ","
        if style == "label" {
            separator = "."
        }
        return PathPrefix(name, style) + strings.Join(exploded, separator)
    }
    return PathPrefix(name, style) + strings.Join(entries, ",")
}

func PathPrefix(name string, style string) string {
    if style == "label" {
        return "."
    }
    if style == "matrix" {
        return ";" + name
    }
    return ""
}
type QueryParameterSpec struct {
    Name          string
    Value         interface{}
    Style         string
    Explode       bool
    AllowReserved bool
    ContentType   string
}

func BuildQueryString(parameters []QueryParameterSpec) string {
    pairs := make([]string, 0)
    for _, parameter := range parameters {
        AppendSerializedParameter(&pairs, parameter)
    }
    return strings.Join(pairs, "&")
}

func AppendSerializedParameter(pairs *[]string, parameter QueryParameterSpec) {
    if parameter.Value == nil {
        return
    }

    if parameter.ContentType != "" {
        encoded, _ := json.Marshal(parameter.Value)
        *pairs = append(*pairs, url.QueryEscape(parameter.Name)+"="+EncodeQueryValue(string(encoded), parameter.AllowReserved))
        return
    }

    style := parameter.Style
    if style == "" {
        style = "form"
    }

    switch value := parameter.Value.(type) {
    case []string:
        AppendArrayParameter(pairs, parameter.Name, stringSliceToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case []int:
        AppendArrayParameter(pairs, parameter.Name, intSliceToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case []interface{}:
        AppendArrayParameter(pairs, parameter.Name, value, style, parameter.Explode, parameter.AllowReserved)
    case map[string]int:
        AppendObjectParameter(pairs, parameter.Name, intMapToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case map[string]string:
        AppendObjectParameter(pairs, parameter.Name, stringMapToInterface(value), style, parameter.Explode, parameter.AllowReserved)
    case map[string]interface{}:
        if style == "deepObject" {
            AppendDeepObjectParameter(pairs, parameter.Name, value, parameter.AllowReserved)
        } else {
            AppendObjectParameter(pairs, parameter.Name, value, style, parameter.Explode, parameter.AllowReserved)
        }
    default:
        *pairs = append(*pairs, url.QueryEscape(parameter.Name)+"="+EncodeQueryValue(fmt.Sprint(value), parameter.AllowReserved))
    }
}

func AppendArrayParameter(pairs *[]string, name string, value []interface{}, style string, explode bool, allowReserved bool) {
    values := make([]string, 0, len(value))
    for _, item := range value {
        if item != nil {
            values = append(values, fmt.Sprint(item))
        }
    }
    if len(values) == 0 {
        return
    }
    if style == "form" && explode {
        for _, item := range values {
            *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(item, allowReserved))
        }
        return
    }
    *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(strings.Join(values, ","), allowReserved))
}

func AppendObjectParameter(pairs *[]string, name string, value map[string]interface{}, style string, explode bool, allowReserved bool) {
    entries := make([]string, 0, len(value)*2)
    for key, item := range value {
        if item == nil {
            continue
        }
        if style == "form" && explode {
            *pairs = append(*pairs, url.QueryEscape(key)+"="+EncodeQueryValue(fmt.Sprint(item), allowReserved))
            continue
        }
        entries = append(entries, key, fmt.Sprint(item))
    }
    if len(entries) == 0 {
        return
    }
    if !(style == "form" && explode) {
        *pairs = append(*pairs, url.QueryEscape(name)+"="+EncodeQueryValue(strings.Join(entries, ","), allowReserved))
    }
}

func AppendDeepObjectParameter(pairs *[]string, name string, value map[string]interface{}, allowReserved bool) {
    for key, item := range value {
        if item == nil {
            continue
        }
        *pairs = append(*pairs, url.QueryEscape(fmt.Sprintf("%s[%s]", name, key))+"="+EncodeQueryValue(fmt.Sprint(item), allowReserved))
    }
}

func EncodeQueryValue(value string, allowReserved bool) string {
    encoded := url.QueryEscape(value)
    if !allowReserved {
        return encoded
    }
    replacements := map[string]string{
        "%3A": ":", "%2F": "/", "%3F": "?", "%23": "#",
        "%5B": "[", "%5D": "]", "%40": "@", "%21": "!",
        "%24": "$", "%26": "&", "%27": "'", "%28": "(",
        "%29": ")", "%2A": "*", "%2B": "+", "%2C": ",",
        "%3B": ";", "%3D": "=",
    }
    for escaped, reserved := range replacements {
        encoded = strings.ReplaceAll(encoded, escaped, reserved)
    }
    return encoded
}



func stringSliceToInterface(values []string) []interface{} {
    result := make([]interface{}, 0, len(values))
    for _, value := range values {
        result = append(result, value)
    }
    return result
}

func intSliceToInterface(values []int) []interface{} {
    result := make([]interface{}, 0, len(values))
    for _, value := range values {
        result = append(result, value)
    }
    return result
}

func stringMapToInterface(values map[string]string) map[string]interface{} {
    result := make(map[string]interface{}, len(values))
    for key, value := range values {
        result[key] = value
    }
    return result
}

func intMapToInterface(values map[string]int) map[string]interface{} {
    result := make(map[string]interface{}, len(values))
    for key, value := range values {
        result[key] = value
    }
    return result
}
