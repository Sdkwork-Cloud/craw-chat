export interface RequestAutomationExecution {
  executionId: string;
  triggerType: string;
  targetKind: string;
  targetRef: string;
  inputPayload?: string;
}
