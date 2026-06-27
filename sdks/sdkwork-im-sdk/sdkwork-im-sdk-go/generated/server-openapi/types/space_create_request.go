package types


type SpaceCreateRequest struct {
	SpaceName string `json:"spaceName"`
	SpaceType string `json:"spaceType"`
	Description string `json:"description"`
}
