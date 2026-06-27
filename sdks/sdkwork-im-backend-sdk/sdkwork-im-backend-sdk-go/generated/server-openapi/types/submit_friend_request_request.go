package types


type SubmitFriendRequestRequest struct {
	EventId string `json:"eventId"`
	RequestId string `json:"requestId"`
	RequestMessage string `json:"requestMessage"`
	RequestedAt string `json:"requestedAt"`
	RequesterUserId string `json:"requesterUserId"`
	TargetUserId string `json:"targetUserId"`
}
