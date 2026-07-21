package types


type RouteNodeLifecycle struct {
	DrainStatus string `json:"drainStatus"`
	NodeId string `json:"nodeId"`
	OwnedRouteCount string `json:"ownedRouteCount"`
	RebalanceState string `json:"rebalanceState"`
}
