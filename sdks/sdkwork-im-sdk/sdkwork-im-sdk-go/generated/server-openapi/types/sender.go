package types


type Sender struct {
	Id string `json:"id"`
	Kind string `json:"kind"`
	PrincipalId string `json:"principalId"`
	PrincipalKind string `json:"principalKind"`
	DisplayName string `json:"displayName"`
	AvatarUrl string `json:"avatarUrl"`
}
