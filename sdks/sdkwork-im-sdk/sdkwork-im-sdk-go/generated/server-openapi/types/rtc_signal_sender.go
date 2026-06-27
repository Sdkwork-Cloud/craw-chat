package types


type RtcSignalSender struct {
	Id string `json:"id"`
	Kind string `json:"kind"`
	MemberId string `json:"memberId"`
	DeviceId string `json:"deviceId"`
	SessionId string `json:"sessionId"`
	Metadata map[string]interface{} `json:"metadata"`
}
