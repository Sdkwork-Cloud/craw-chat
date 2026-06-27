package types


type SubmitFriendRequestRequest struct {
	TargetUserId string `json:"targetUserId"`
	RequestMessage string `json:"requestMessage"`
}
