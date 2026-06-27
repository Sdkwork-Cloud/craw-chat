import type { AutomationExecutionState } from './automation-execution-state';

export interface AutomationExecution {
  tenantId: string;
  principalId: string;
  principalKind: string;
  executionId: string;
  triggerType: string;
  targetKind: string;
  targetRef: string;
  inputPayload?: string;
  outputPayload?: string;
  state: AutomationExecutionState;
  retryCount: number;
  requestedAt: string;
  completedAt?: string;
  failureReason?: string;
}
