package types


type QuotaProfileResponse struct {
	MaxConcurrentSessionsPerTenant int `json:"maxConcurrentSessionsPerTenant"`
	MaxInflightMessages int `json:"maxInflightMessages"`
	MaxPayloadBytes int `json:"maxPayloadBytes"`
	MaxSubscriptionsPerSession int `json:"maxSubscriptionsPerSession"`
	ProfileId string `json:"profileId"`
}
