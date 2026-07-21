package types


type QuotaProfileResponse struct {
	MaxConcurrentSessionsPerTenant string `json:"maxConcurrentSessionsPerTenant"`
	MaxInflightMessages string `json:"maxInflightMessages"`
	MaxPayloadBytes string `json:"maxPayloadBytes"`
	MaxSubscriptionsPerSession string `json:"maxSubscriptionsPerSession"`
	ProfileId string `json:"profileId"`
}
