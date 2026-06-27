package types


type ContactPreferencesView struct {
	TenantId string `json:"tenantId"`
	OwnerUserId string `json:"ownerUserId"`
	TargetUserId string `json:"targetUserId"`
	IsStarred bool `json:"isStarred"`
	Remark string `json:"remark"`
	IsBlocked bool `json:"isBlocked"`
	UpdatedAt string `json:"updatedAt"`
}
