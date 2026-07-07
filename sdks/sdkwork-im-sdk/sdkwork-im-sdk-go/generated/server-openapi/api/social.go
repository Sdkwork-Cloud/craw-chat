package api

import (
    "encoding/json"
    "fmt"
    "net/url"
    "strings"
    sdktypes "github.com/sdkwork/im-sdk-generated/types"
    sdkhttp "github.com/sdkwork/im-sdk-generated/http"
)

type SocialApi struct {
    client *sdkhttp.Client
}

func NewSocialApi(client *sdkhttp.Client) *SocialApi {
    return &SocialApi{client: client}
}

// Search social users
func (a *SocialApi) UsersList(q *string, pageSize *int, cursor *string) (sdktypes.SocialUsersListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "q", Value: func() interface{} { if q == nil { return nil }; return *q }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath("/social/users"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SocialUsersListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialUsersListResponse](raw)
}

// List friend requests
func (a *SocialApi) FriendRequestsList(direction *string, status *string, pageSize *int, cursor *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "direction", Value: func() interface{} { if direction == nil { return nil }; return *direction }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "status", Value: func() interface{} { if status == nil { return nil }; return *status }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath("/social/friend_requests"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// Create a friend request
func (a *SocialApi) FriendRequestsCreate(body sdktypes.SubmitFriendRequestRequest) (sdktypes.SocialFriendRequestsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/social/friend_requests"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialFriendRequestsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestsCreateResponse201](raw)
}

// Retrieve pending incoming friend request count
func (a *SocialApi) FriendRequestsPendingCountRetrieve() (sdktypes.SocialFriendRequestsPendingCountRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath("/social/friend_requests/pending/count"), nil, nil)
    if err != nil {
        var zero sdktypes.SocialFriendRequestsPendingCountRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestsPendingCountRetrieveResponse](raw)
}

// Accept a friend request
func (a *SocialApi) FriendRequestsAccept(friendRequestId string) (sdktypes.SocialFriendRequestsAcceptResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/social/friend_requests/%s/accept", SerializePathParameter(friendRequestId, PathParameterSpec{Name: "friendRequestId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialFriendRequestsAcceptResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestsAcceptResponse](raw)
}

// Decline a friend request
func (a *SocialApi) FriendRequestsDecline(friendRequestId string) (sdktypes.SocialFriendRequestsDeclineResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/social/friend_requests/%s/decline", SerializePathParameter(friendRequestId, PathParameterSpec{Name: "friendRequestId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialFriendRequestsDeclineResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestsDeclineResponse](raw)
}

// Cancel a friend request
func (a *SocialApi) FriendRequestsCancel(friendRequestId string) (sdktypes.SocialFriendRequestsCancelResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/social/friend_requests/%s/cancel", SerializePathParameter(friendRequestId, PathParameterSpec{Name: "friendRequestId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialFriendRequestsCancelResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendRequestsCancelResponse](raw)
}

// Remove a friendship
func (a *SocialApi) FriendshipsRemove(friendshipId string) (sdktypes.SocialFriendshipsRemoveResponse, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/social/friendships/%s/remove", SerializePathParameter(friendshipId, PathParameterSpec{Name: "friendshipId", Style: "simple", Explode: false}))), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.SocialFriendshipsRemoveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialFriendshipsRemoveResponse](raw)
}

// Block a social user
func (a *SocialApi) UserBlocksCreate(body sdktypes.BlockUserRequest) (sdktypes.SocialUserBlocksCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/social/user_blocks"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialUserBlocksCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialUserBlocksCreateResponse201](raw)
}

// Release a social user block
func (a *SocialApi) UserBlocksDelete(blockId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/social/user_blocks/%s", SerializePathParameter(blockId, PathParameterSpec{Name: "blockId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// List contact tags
func (a *SocialApi) ContactsTagsList(pageSize *int, cursor *string) (sdktypes.SdkWorkListResponse, error) {
    query := BuildQueryString([]QueryParameterSpec{
        {Name: "page_size", Value: func() interface{} { if pageSize == nil { return nil }; return *pageSize }(), Style: "form", Explode: true, AllowReserved: false},
        {Name: "cursor", Value: func() interface{} { if cursor == nil { return nil }; return *cursor }(), Style: "form", Explode: true, AllowReserved: false},
    })
    raw, err := a.client.Get(AppendQueryString(ImApiPath("/social/contacts/tags"), query), nil, nil)
    if err != nil {
        var zero sdktypes.SdkWorkListResponse
        return zero, err
    }
    return decodeResult[sdktypes.SdkWorkListResponse](raw)
}

// Create a contact tag
func (a *SocialApi) ContactsTagsCreate(body sdktypes.CreateContactTagRequest) (sdktypes.SocialContactsTagsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath("/social/contacts/tags"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialContactsTagsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialContactsTagsCreateResponse201](raw)
}

// Update a contact tag
func (a *SocialApi) ContactsTagsUpdate(tagId string, body sdktypes.UpdateContactTagRequest) (sdktypes.SocialContactsTagsUpdateResponse, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/social/contacts/tags/%s", SerializePathParameter(tagId, PathParameterSpec{Name: "tagId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialContactsTagsUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialContactsTagsUpdateResponse](raw)
}

// Delete a contact tag
func (a *SocialApi) ContactsTagsDelete(tagId string) (struct{}, error) {
    raw, err := a.client.Delete(ImApiPath(fmt.Sprintf("/social/contacts/tags/%s", SerializePathParameter(tagId, PathParameterSpec{Name: "tagId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero struct{}
        return zero, err
    }
    return decodeResult[struct{}](raw)
}

// Create a contact recommendation
func (a *SocialApi) ContactsRecommendationsCreate(targetUserId string, body sdktypes.CreateContactRecommendationRequest) (sdktypes.SocialContactsRecommendationsCreateResponse201, error) {
    raw, err := a.client.Post(ImApiPath(fmt.Sprintf("/social/contacts/%s/recommendations", SerializePathParameter(targetUserId, PathParameterSpec{Name: "targetUserId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialContactsRecommendationsCreateResponse201
        return zero, err
    }
    return decodeResult[sdktypes.SocialContactsRecommendationsCreateResponse201](raw)
}

// Retrieve contact preferences
func (a *SocialApi) ContactsPreferencesRetrieve(targetUserId string) (sdktypes.SocialContactsPreferencesRetrieveResponse, error) {
    raw, err := a.client.Get(ImApiPath(fmt.Sprintf("/social/contacts/%s/preferences", SerializePathParameter(targetUserId, PathParameterSpec{Name: "targetUserId", Style: "simple", Explode: false}))), nil, nil)
    if err != nil {
        var zero sdktypes.SocialContactsPreferencesRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialContactsPreferencesRetrieveResponse](raw)
}

// Update contact preferences
func (a *SocialApi) ContactsPreferencesUpdate(targetUserId string, body sdktypes.UpdateContactPreferencesRequest) (sdktypes.SocialContactsPreferencesUpdateResponse, error) {
    raw, err := a.client.Patch(ImApiPath(fmt.Sprintf("/social/contacts/%s/preferences", SerializePathParameter(targetUserId, PathParameterSpec{Name: "targetUserId", Style: "simple", Explode: false}))), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SocialContactsPreferencesUpdateResponse
        return zero, err
    }
    return decodeResult[sdktypes.SocialContactsPreferencesUpdateResponse](raw)
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
