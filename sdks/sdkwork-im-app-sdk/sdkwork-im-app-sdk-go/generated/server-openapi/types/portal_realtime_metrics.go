package types


type PortalRealtimeMetrics struct {
	ClientRouteWindowCount PortalInt64Count `json:"clientRouteWindowCount"`
	PendingEventCount PortalInt64Count `json:"pendingEventCount"`
	MaxClientRouteWindowEventCount PortalInt64Count `json:"maxClientRouteWindowEventCount"`
	ClientRouteWindowCapacity PortalInt64Count `json:"clientRouteWindowCapacity"`
	MaxClientRouteWindowUsagePermille int `json:"maxClientRouteWindowUsagePermille"`
	CapacityTrimmedEventCount PortalInt64Count `json:"capacityTrimmedEventCount"`
	OldestPendingOccurredAt string `json:"oldestPendingOccurredAt"`
}
