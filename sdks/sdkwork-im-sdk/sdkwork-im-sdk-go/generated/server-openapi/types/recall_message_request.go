package types


type RecallMessageRequest struct {
	IdempotencyKey string `json:"idempotencyKey"`
}
