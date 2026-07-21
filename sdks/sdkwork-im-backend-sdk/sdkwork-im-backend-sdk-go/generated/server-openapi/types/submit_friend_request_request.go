package types


type SubmitFriendRequestRequest struct {
	EventId string `json:"eventId"`
	RequestMessage string `json:"requestMessage"`
	RequestedAt string `json:"requestedAt"`
	RequesterUserId string `json:"requesterUserId"`
	TargetUserId string `json:"targetUserId"`
}
