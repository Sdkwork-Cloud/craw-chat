package types


type SpaceGroupMemberView struct {
	UserId string `json:"userId"`
	Role string `json:"role"`
	Nickname string `json:"nickname"`
	MuteUntil string `json:"muteUntil"`
	JoinedAt string `json:"joinedAt"`
}
