package types


type ProblemDetail struct {
	Type string `json:"type"`
	Title string `json:"title"`
	Status int `json:"status"`
	Detail string `json:"detail"`
	Instance string `json:"instance"`
	Code SdkWorkPlatformErrorCode `json:"code"`
	TraceId string `json:"traceId"`
	I18nKey string `json:"i18nKey"`
	Locale string `json:"locale"`
	Errors []FieldError `json:"errors"`
}
