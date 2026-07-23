package types


type PortalConversationOperationalMetrics struct {
	LaggingScopeCount PortalInt64Count `json:"laggingScopeCount"`
	MaxOperationalLag PortalInt64Count `json:"maxOperationalLag"`
	PendingOutboxEventCount PortalInt64Count `json:"pendingOutboxEventCount"`
	FailedOutboxAttemptCount PortalInt64Count `json:"failedOutboxAttemptCount"`
}
