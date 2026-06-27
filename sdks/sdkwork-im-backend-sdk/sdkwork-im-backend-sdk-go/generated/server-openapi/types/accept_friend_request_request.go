package types


type AcceptFriendRequestRequest struct {
	AcceptedAt string `json:"acceptedAt"`
	AcceptedByUserId string `json:"acceptedByUserId"`
	EventId string `json:"eventId"`
}
