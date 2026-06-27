import type { AutomationExecutionDeliveryStatus } from './automation-execution-delivery-status';
import type { AutomationExecutionState } from './automation-execution-state';

export interface AutomationExecutionRequestResponse {
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
  requestKey: string;
  deliveryStatus: AutomationExecutionDeliveryStatus;
  proofVersion: string;
}
