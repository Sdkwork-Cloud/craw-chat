package types


type UpdateContactPreferencesRequest struct {
	IsStarred bool `json:"isStarred"`
	Remark string `json:"remark"`
	IsBlocked bool `json:"isBlocked"`
}
