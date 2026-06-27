package types


type SpaceChannelAccessRuleCreateRequest struct {
	RuleType string `json:"ruleType"`
	PrincipalKind string `json:"principalKind"`
	PrincipalId string `json:"principalId"`
	Permission string `json:"permission"`
}
