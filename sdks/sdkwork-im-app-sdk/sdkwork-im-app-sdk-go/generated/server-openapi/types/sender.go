package types


type Sender struct {
	Id string `json:"id"`
	Kind string `json:"kind"`
	MemberId string `json:"memberId"`
	DeviceId string `json:"deviceId"`
	SessionId string `json:"sessionId"`
	Metadata StringMap `json:"metadata"`
}
