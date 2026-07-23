package types


type PortalWorkspaceView struct {
	Name string `json:"name"`
	Slug string `json:"slug"`
	Environment string `json:"environment"`
	Tier string `json:"tier"`
	Region string `json:"region"`
	SupportPlan string `json:"supportPlan"`
	Seats PortalInt64Count `json:"seats"`
	ActiveBrands PortalInt64Count `json:"activeBrands"`
}
