package types


type PortalWorkspaceView struct {
	Name string `json:"name"`
	Slug string `json:"slug"`
	Tier string `json:"tier"`
	Region string `json:"region"`
	SupportPlan string `json:"supportPlan"`
	Seats int `json:"seats"`
	ActiveBrands int `json:"activeBrands"`
	Uptime string `json:"uptime"`
}
