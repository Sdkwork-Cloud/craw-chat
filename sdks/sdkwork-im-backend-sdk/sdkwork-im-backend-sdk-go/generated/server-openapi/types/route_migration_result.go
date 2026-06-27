package types


type RouteMigrationResult struct {
	MigratedRouteCount int `json:"migratedRouteCount"`
	SourceDrainStatus string `json:"sourceDrainStatus"`
	SourceNodeId string `json:"sourceNodeId"`
	SourceRebalanceState string `json:"sourceRebalanceState"`
	TargetDrainStatus string `json:"targetDrainStatus"`
	TargetNodeId string `json:"targetNodeId"`
	TargetRebalanceState string `json:"targetRebalanceState"`
}
