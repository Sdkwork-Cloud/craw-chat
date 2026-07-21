package types


type LagPageData struct {
	Items []LagItem `json:"items"`
	PageInfo PageInfo `json:"pageInfo"`
}
