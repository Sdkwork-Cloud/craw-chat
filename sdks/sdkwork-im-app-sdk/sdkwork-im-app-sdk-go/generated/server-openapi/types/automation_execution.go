package types


type AutomationExecution struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	PrincipalKind string `json:"principalKind"`
	ExecutionId string `json:"executionId"`
	TriggerType string `json:"triggerType"`
	TargetKind string `json:"targetKind"`
	TargetRef string `json:"targetRef"`
	InputPayload string `json:"inputPayload"`
	OutputPayload string `json:"outputPayload"`
	State AutomationExecutionState `json:"state"`
	RetryCount int `json:"retryCount"`
	RequestedAt string `json:"requestedAt"`
	CompletedAt string `json:"completedAt"`
	FailureReason string `json:"failureReason"`
}
