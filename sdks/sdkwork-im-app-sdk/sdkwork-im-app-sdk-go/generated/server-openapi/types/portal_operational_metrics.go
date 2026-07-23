package types


type PortalOperationalMetrics struct {
	ClientRouteWindowCount PortalInt64Count `json:"clientRouteWindowCount"`
	PendingRealtimeEventCount PortalInt64Count `json:"pendingRealtimeEventCount"`
}
