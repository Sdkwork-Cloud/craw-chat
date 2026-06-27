package types


type DriveReference struct {
	DriveUri string `json:"driveUri"`
	SpaceId string `json:"spaceId"`
	NodeId string `json:"nodeId"`
	NodeVersion string `json:"nodeVersion"`
}
