package types


type RtcParticipantCredential struct {
	TenantId string `json:"tenantId"`
	RtcSessionId string `json:"rtcSessionId"`
	ParticipantId string `json:"participantId"`
	Credential string `json:"credential"`
	ExpiresAt string `json:"expiresAt"`
}
