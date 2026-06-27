package types


type SpaceChannelAccessRuleView struct {
	RuleId string `json:"ruleId"`
	ChannelId string `json:"channelId"`
	RuleType string `json:"ruleType"`
	PrincipalKind string `json:"principalKind"`
	PrincipalId string `json:"principalId"`
	Permission string `json:"permission"`
	CreatedAt string `json:"createdAt"`
}
