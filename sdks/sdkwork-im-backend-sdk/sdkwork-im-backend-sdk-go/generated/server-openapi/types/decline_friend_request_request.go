package types


type DeclineFriendRequestRequest struct {
	DeclinedAt string `json:"declinedAt"`
	DeclinedByUserId string `json:"declinedByUserId"`
	EventId string `json:"eventId"`
}
