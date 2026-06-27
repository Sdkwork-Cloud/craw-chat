package types


type RequestAutomationExecution struct {
	ExecutionId string `json:"executionId"`
	TriggerType string `json:"triggerType"`
	TargetKind string `json:"targetKind"`
	TargetRef string `json:"targetRef"`
	InputPayload string `json:"inputPayload"`
}
