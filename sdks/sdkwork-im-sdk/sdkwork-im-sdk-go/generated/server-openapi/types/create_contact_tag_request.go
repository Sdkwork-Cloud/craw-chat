package types


type CreateContactTagRequest struct {
	Name string `json:"name"`
	Color string `json:"color"`
	Count int `json:"count"`
	Bg string `json:"bg"`
	Border string `json:"border"`
}
