package types


type SocialFriendshipsRetrieveResponse struct {
	Code int `json:"code"`
	Data interface{} `json:"data"`
	TraceId string `json:"traceId"`
}
