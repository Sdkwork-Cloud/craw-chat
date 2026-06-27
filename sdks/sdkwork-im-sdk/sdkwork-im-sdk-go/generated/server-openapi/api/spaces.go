package api

import (
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
func (a *SpacesApi) Create(body sdktypes.SpaceCreateRequest) (sdktypes.SpaceView, error) {
    raw, err := a.client.Post(ImApiPath("/spaces"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceView](raw)
}

// List spaces
func (a *SpacesApi) List() (sdktypes.SpaceListResponse, error) {
    raw, err := a.client.Get(ImApiPath("/spaces"), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpaceListResponse](raw)
}

// Get a space
func (a *SpacesApi) Get(spaceId string) (sdktypes.SpaceView, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceView](raw)
}

// Update a space
func (a *SpacesApi) Update(spaceId string, body sdktypes.SpaceUpdateRequest) (sdktypes.SpaceView, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/spaces/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceView](raw)
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
func (a *SpacesApi) MembersList(spaceId string) (sdktypes.SpaceMemberListResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/members", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceMemberListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpaceMemberListResponse](raw)
}

// Create spaces members
func (a *SpacesApi) MembersCreate(spaceId string, body sdktypes.SpaceMemberCreateRequest) (sdktypes.SpaceMemberView, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/members", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceMemberView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceMemberView](raw)
}

// Get spaces members
func (a *SpacesApi) MembersGet(spaceId string, userId string) (sdktypes.SpaceMemberView, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/members/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceMemberView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceMemberView](raw)
}

// Update spaces members
func (a *SpacesApi) MembersUpdate(spaceId string, userId string, body sdktypes.SpaceMemberUpdateRequest) (sdktypes.SpaceMemberView, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/spaces/%s/members/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceMemberView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceMemberView](raw)
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
func (a *SpacesApi) GroupsList(spaceId string) (sdktypes.SpaceGroupListResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/groups", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceGroupListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpaceGroupListResponse](raw)
}

// Create spaces groups
func (a *SpacesApi) GroupsCreate(spaceId string, body sdktypes.SpaceGroupCreateRequest) (sdktypes.SpaceGroupView, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/groups", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceGroupView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceGroupView](raw)
}

// Get spaces groups
func (a *SpacesApi) GroupsGet(spaceId string, groupId string) (sdktypes.SpaceGroupView, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceGroupView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceGroupView](raw)
}

// Update spaces groups
func (a *SpacesApi) GroupsUpdate(spaceId string, groupId string, body sdktypes.SpaceGroupUpdateRequest) (sdktypes.SpaceGroupView, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceGroupView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceGroupView](raw)
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
func (a *SpacesApi) GroupsMembersList(spaceId string, groupId string) (sdktypes.SpaceGroupMemberListResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s/members", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceGroupMemberListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpaceGroupMemberListResponse](raw)
}

// Create spaces groups members
func (a *SpacesApi) GroupsMembersCreate(spaceId string, groupId string, body sdktypes.SpaceGroupMemberCreateRequest) (sdktypes.SpaceGroupMemberView, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s/members", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceGroupMemberView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceGroupMemberView](raw)
}

// Get spaces groups members
func (a *SpacesApi) GroupsMembersGet(spaceId string, groupId string, userId string) (sdktypes.SpaceGroupMemberView, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s/members/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceGroupMemberView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceGroupMemberView](raw)
}

// Update spaces groups members
func (a *SpacesApi) GroupsMembersUpdate(spaceId string, groupId string, userId string, body sdktypes.SpaceGroupMemberUpdateRequest) (struct{}, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/spaces/%s/groups/%s/members/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(groupId, PathParameterSpec{Name: "groupId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
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
func (a *SpacesApi) ChannelsList(spaceId string) (sdktypes.SpaceChannelListResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/channels", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceChannelListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpaceChannelListResponse](raw)
}

// Create spaces channels
func (a *SpacesApi) ChannelsCreate(spaceId string, body sdktypes.SpaceChannelCreateRequest) (sdktypes.SpaceChannelView, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/channels", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceChannelView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceChannelView](raw)
}

// Get spaces channels
func (a *SpacesApi) ChannelsGet(spaceId string, channelId string) (sdktypes.SpaceChannelView, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/channels/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceChannelView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceChannelView](raw)
}

// Update spaces channels
func (a *SpacesApi) ChannelsUpdate(spaceId string, channelId string, body sdktypes.SpaceChannelUpdateRequest) (sdktypes.SpaceChannelView, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/spaces/%s/channels/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceChannelView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceChannelView](raw)
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
func (a *SpacesApi) ChannelsAccessRulesList(spaceId string, channelId string) (sdktypes.SpaceChannelAccessRuleListResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/channels/%s/access_rules", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceChannelAccessRuleListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpaceChannelAccessRuleListResponse](raw)
}

// Create spaces channels access Rules
func (a *SpacesApi) ChannelsAccessRulesCreate(spaceId string, channelId string, body sdktypes.SpaceChannelAccessRuleCreateRequest) (sdktypes.SpaceChannelAccessRuleView, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/channels/%s/access_rules", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(channelId, PathParameterSpec{Name: "channelId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceChannelAccessRuleView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceChannelAccessRuleView](raw)
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
func (a *SpacesApi) InvitesList(spaceId string) (sdktypes.SpaceInviteListResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/invites", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceInviteListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpaceInviteListResponse](raw)
}

// Create spaces invites
func (a *SpacesApi) InvitesCreate(spaceId string, body sdktypes.SpaceInviteCreateRequest) (sdktypes.SpaceInviteView, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/invites", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceInviteView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceInviteView](raw)
}

// Get spaces invites
func (a *SpacesApi) InvitesGet(spaceId string, inviteCode string) (sdktypes.SpaceInviteView, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/invites/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(inviteCode, PathParameterSpec{Name: "inviteCode", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceInviteView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceInviteView](raw)
}

// Revoke spaces invites
func (a *SpacesApi) InvitesRevoke(spaceId string, inviteCode string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/spaces/%s/invites/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(inviteCode, PathParameterSpec{Name: "inviteCode", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// Accept spaces invites
func (a *SpacesApi) InvitesAccept(spaceId string, inviteCode string) (struct{}, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/invites/%s/accept", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(inviteCode, PathParameterSpec{Name: "inviteCode", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// List spaces bans
func (a *SpacesApi) BansList(spaceId string) (sdktypes.SpaceBanListResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/bans", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceBanListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SpaceBanListResponse](raw)
}

// Create spaces bans
func (a *SpacesApi) BansCreate(spaceId string, body sdktypes.SpaceBanCreateRequest) (sdktypes.SpaceBanView, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/spaces/%s/bans", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SpaceBanView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceBanView](raw)
}

// Get spaces bans
func (a *SpacesApi) BansGet(spaceId string, userId string) (sdktypes.SpaceBanView, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/spaces/%s/bans/%s", SerializePathParameter(spaceId, PathParameterSpec{Name: "spaceId", Style: "simple", Explode: false}), SerializePathParameter(userId, PathParameterSpec{Name: "userId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SpaceBanView
        return zero, err
    }
    return decodeResult[sdktypes.SpaceBanView](raw)
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
