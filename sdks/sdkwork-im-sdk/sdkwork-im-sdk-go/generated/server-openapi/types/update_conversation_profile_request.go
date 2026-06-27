package types


type UpdateConversationProfileRequest struct {
	DisplayName string `json:"displayName"`
	AvatarUrl string `json:"avatarUrl"`
	Notice string `json:"notice"`
}
