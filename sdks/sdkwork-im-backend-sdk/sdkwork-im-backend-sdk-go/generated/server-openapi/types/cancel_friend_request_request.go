package types


type CancelFriendRequestRequest struct {
	CanceledAt string `json:"canceledAt"`
	CanceledByUserId string `json:"canceledByUserId"`
	EventId string `json:"eventId"`
}
