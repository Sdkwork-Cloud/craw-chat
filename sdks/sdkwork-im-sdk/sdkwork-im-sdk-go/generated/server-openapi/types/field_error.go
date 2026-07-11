package types


type FieldError struct {
	Field string `json:"field"`
	Message string `json:"message"`
	Code int `json:"code"`
	I18nKey string `json:"i18nKey"`
	Params map[string]string `json:"params"`
}
