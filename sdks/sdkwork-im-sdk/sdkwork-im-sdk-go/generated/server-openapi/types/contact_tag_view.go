package types


type ContactTagView struct {
	TenantId string `json:"tenantId"`
	OwnerUserId string `json:"ownerUserId"`
	TagId string `json:"tagId"`
	Name string `json:"name"`
	Color string `json:"color"`
	Count int `json:"count"`
	Bg string `json:"bg"`
	Border string `json:"border"`
	CreatedAt string `json:"createdAt"`
	UpdatedAt string `json:"updatedAt"`
}
