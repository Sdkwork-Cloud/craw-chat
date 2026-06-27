package types


type KillSwitchResponse struct {
	Active bool `json:"active"`
	DisabledBindings []string `json:"disabledBindings"`
	DisabledCapabilities []string `json:"disabledCapabilities"`
	DisabledCodecs []string `json:"disabledCodecs"`
	Reason string `json:"reason"`
	RuleId string `json:"ruleId"`
}
