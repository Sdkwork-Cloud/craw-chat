package types


type ProvidersCreateResponse struct {
	Code int `json:"code"`
	Data interface{} `json:"data"`
	TraceId string `json:"traceId"`
}
