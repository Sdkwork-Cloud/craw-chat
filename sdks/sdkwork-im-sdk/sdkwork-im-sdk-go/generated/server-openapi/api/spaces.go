package api

import (
    "encoding/json"
    "fmt"
    "net/url"
    "strings"
    sdktypes "github.com/sdkwork/im-sdk-generated/types"
    sdkhttp "github.com/sdkwork/im-sdk-generated/http"
)

type SpacesApi struct {
    client *sdkhttp.Client
}

func NewSpacesApi(client *sdkhttp.Client) *SpacesApi {
    return &SpacesApi{client: client}
}

// Create a space
func (a *SpacesApi) Create(body sdktypes.SpaceCreateRequest) (sdktypes.SpacesCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/spaces"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SpacesCreateResponse201](raw)
}

// List spaces
func (a *SpacesApi) List(pageSize *int, cursor *string) (sdktypes.SpacesListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath("/spaces"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesListResponse](raw)
}

// Retrieve a space
func (a *SpacesApi) Retrieve(spaceId string) (sdktypes.SpacesRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesRetrieveResponse](raw)
}

// Update a space
func (a *SpacesApi) Update(spaceId string, body sdktypes.SpaceUpdateRequest) (sdktypes.SpacesUpdateResponse, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/spaces/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesUpdateResponse](raw)
}

// Delete a space
func (a *SpacesApi) Delete(spaceId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/spaces/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// List spaces members
func (a *SpacesApi) MembersList(spaceId string, pageSize *int, cursor *string) (sdktypes.SpacesMembersListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/spaces/%s/members", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesMembersListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesMembersListResponse](raw)
}

// Create spaces members
func (a *SpacesApi) MembersCreate(spaceId string, body sdktypes.SpaceMemberCreateRequest) (sdktypes.SpacesMembersCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/members", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesMembersCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SpacesMembersCreateResponse201](raw)
}

// retrieve spaces members
func (a *SpacesApi) MembersRetrieve(spaceId string, userId string) (sdktypes.SpacesMembersRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/members/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesMembersRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesMembersRetrieveResponse](raw)
}

// Update spaces members
func (a *SpacesApi) MembersUpdate(spaceId string, userId string, body sdktypes.SpaceMemberUpdateRequest) (sdktypes.SpacesMembersUpdateResponse, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/spaces/%s/members/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesMembersUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesMembersUpdateResponse](raw)
}

// Delete spaces members
func (a *SpacesApi) MembersDelete(spaceId string, userId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/spaces/%s/members/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// List spaces groups
func (a *SpacesApi) GroupsList(spaceId string, pageSize *int, cursor *string) (sdktypes.SpacesGroupsListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/spaces/%s/groups", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesGroupsListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesGroupsListResponse](raw)
}

// Create spaces groups
func (a *SpacesApi) GroupsCreate(spaceId string, body sdktypes.SpaceGroupCreateRequest) (sdktypes.SpacesGroupsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/groups", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesGroupsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SpacesGroupsCreateResponse201](raw)
}

// retrieve spaces groups
func (a *SpacesApi) GroupsRetrieve(spaceId string, groupId string) (sdktypes.SpacesGroupsRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesGroupsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesGroupsRetrieveResponse](raw)
}

// Update spaces groups
func (a *SpacesApi) GroupsUpdate(spaceId string, groupId string, body sdktypes.SpaceGroupUpdateRequest) (sdktypes.SpacesGroupsUpdateResponse, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesGroupsUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesGroupsUpdateResponse](raw)
}

// Delete spaces groups
func (a *SpacesApi) GroupsDelete(spaceId string, groupId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// List spaces groups members
func (a *SpacesApi) GroupsMembersList(spaceId string, groupId string, pageSize *int, cursor *string) (sdktypes.SpacesGroupsMembersListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s/members", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesGroupsMembersListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesGroupsMembersListResponse](raw)
}

// Create spaces groups members
func (a *SpacesApi) GroupsMembersCreate(spaceId string, groupId string, body sdktypes.SpaceGroupMemberCreateRequest) (sdktypes.SpacesGroupsMembersCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s/members", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesGroupsMembersCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SpacesGroupsMembersCreateResponse201](raw)
}

// retrieve spaces groups members
func (a *SpacesApi) GroupsMembersRetrieve(spaceId string, groupId string, userId string) (sdktypes.SpacesGroupsMembersRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s/members/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesGroupsMembersRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesGroupsMembersRetrieveResponse](raw)
}

// Update spaces groups members
func (a *SpacesApi) GroupsMembersUpdate(spaceId string, groupId string, userId string, body sdktypes.SpaceGroupMemberUpdateRequest) (sdktypes.SpacesGroupsMembersUpdateResponse, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s/members/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesGroupsMembersUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesGroupsMembersUpdateResponse](raw)
}

// Delete spaces groups members
func (a *SpacesApi) GroupsMembersDelete(spaceId string, groupId string, userId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s/members/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// List spaces channels
func (a *SpacesApi) ChannelsList(spaceId string, pageSize *int, cursor *string) (sdktypes.SpacesChannelsListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/spaces/%s/channels", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesChannelsListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesChannelsListResponse](raw)
}

// Create spaces channels
func (a *SpacesApi) ChannelsCreate(spaceId string, body sdktypes.SpaceChannelCreateRequest) (sdktypes.SpacesChannelsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/channels", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesChannelsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SpacesChannelsCreateResponse201](raw)
}

// retrieve spaces channels
func (a *SpacesApi) ChannelsRetrieve(spaceId string, channelId string) (sdktypes.SpacesChannelsRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/channels/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesChannelsRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesChannelsRetrieveResponse](raw)
}

// Update spaces channels
func (a *SpacesApi) ChannelsUpdate(spaceId string, channelId string, body sdktypes.SpaceChannelUpdateRequest) (sdktypes.SpacesChannelsUpdateResponse, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/spaces/%s/channels/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesChannelsUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesChannelsUpdateResponse](raw)
}

// Delete spaces channels
func (a *SpacesApi) ChannelsDelete(spaceId string, channelId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/spaces/%s/channels/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// List spaces channels access Rules
func (a *SpacesApi) ChannelsAccessRulesList(spaceId string, channelId string, pageSize *int, cursor *string) (sdktypes.SpacesChannelsAccessRulesListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/spaces/%s/channels/%s/access_rules", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesChannelsAccessRulesListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesChannelsAccessRulesListResponse](raw)
}

// Create spaces channels access Rules
func (a *SpacesApi) ChannelsAccessRulesCreate(spaceId string, channelId string, body sdktypes.SpaceChannelAccessRuleCreateRequest) (sdktypes.SpacesChannelsAccessRulesCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/channels/%s/access_rules", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesChannelsAccessRulesCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SpacesChannelsAccessRulesCreateResponse201](raw)
}

// Delete spaces channels access Rules
func (a *SpacesApi) ChannelsAccessRulesDelete(spaceId string, channelId string, ruleId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/spaces/%s/channels/%s/access_rules/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}), SerializePathParameter(ruleId, PathParameterSpec{Name: "ruleId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// List spaces invites
func (a *SpacesApi) InvitesList(spaceId string, status *string, pageSize *int, cursor *string) (sdktypes.SpacesInvitesListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "status", Value: func() interface{} { if status == nil { return nil }; return *status }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/spaces/%s/invites", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesInvitesListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesInvitesListResponse](raw)
}

// Create spaces invites
func (a *SpacesApi) InvitesCreate(spaceId string, body sdktypes.SpaceInviteCreateRequest) (sdktypes.SpacesInvitesCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/invites", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesInvitesCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SpacesInvitesCreateResponse201](raw)
}

// retrieve spaces invites
func (a *SpacesApi) InvitesRetrieve(spaceId string, inviteCode string) (sdktypes.SpacesInvitesRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/invites/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(inviteCode, PathParameterSpec{Name: "inviteCode", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesInvitesRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesInvitesRetrieveResponse](raw)
}

// Delete spaces invites
func (a *SpacesApi) InvitesDelete(spaceId string, inviteCode string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/spaces/%s/invites/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(inviteCode, PathParameterSpec{Name: "inviteCode", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// Accept spaces invites
func (a *SpacesApi) InvitesAccept(spaceId string, inviteCode string) (sdktypes.SdkWorkCommandResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/invites/%s/accept", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(inviteCode, PathParameterSpec{Name: "inviteCode", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SdkWorkCommandResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkCommandResponse](raw)
}

// List spaces bans
func (a *SpacesApi) BansList(spaceId string, pageSize *int, cursor *string) (sdktypes.SpacesBansListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath(fmt.Sprintf("/spaces/%s/bans", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), query), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesBansListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesBansListResponse](raw)
}

// Create spaces bans
func (a *SpacesApi) BansCreate(spaceId string, body sdktypes.SpaceBanCreateRequest) (sdktypes.SpacesBansCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/bans", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpacesBansCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SpacesBansCreateResponse201](raw)
}

// retrieve spaces bans
func (a *SpacesApi) BansRetrieve(spaceId string, userId string) (sdktypes.SpacesBansRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/bans/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpacesBansRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpacesBansRetrieveResponse](raw)
}

// Delete spaces bans
func (a *SpacesApi) BansDelete(spaceId string, userId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/spaces/%s/bans/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
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
