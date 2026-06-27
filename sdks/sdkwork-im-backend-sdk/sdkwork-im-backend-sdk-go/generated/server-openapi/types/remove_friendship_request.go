package types


type RemoveFriendshipRequest struct {
	EventId string `json:"eventId"`
	RemovedAt string `json:"removedAt"`
	RemovedByUserId string `json:"removedByUserId"`
}
