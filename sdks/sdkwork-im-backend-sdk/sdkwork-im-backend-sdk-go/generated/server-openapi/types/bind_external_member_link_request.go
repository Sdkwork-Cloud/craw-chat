package types


type BindExternalMemberLinkRequest struct {
	ConnectionId string `json:"connectionId"`
	EventId string `json:"eventId"`
	ExternalDisplayName string `json:"externalDisplayName"`
	ExternalMemberId string `json:"externalMemberId"`
	LinkId string `json:"linkId"`
	LinkedAt string `json:"linkedAt"`
	LocalActorId string `json:"localActorId"`
	LocalActorKind string `json:"localActorKind"`
}
