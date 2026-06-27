package types


type ProblemDetail struct {
	Type string `json:"type"`
	Title string `json:"title"`
	Status int `json:"status"`
	Detail string `json:"detail"`
	Code string `json:"code"`
	Message string `json:"message"`
	TraceId string `json:"traceId"`
	Retryable bool `json:"retryable"`
}
