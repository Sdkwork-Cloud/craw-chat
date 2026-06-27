package types


type SpaceGroupMemberUpdateRequest struct {
	Role string `json:"role"`
	Nickname string `json:"nickname"`
	MuteUntil string `json:"muteUntil"`
}
