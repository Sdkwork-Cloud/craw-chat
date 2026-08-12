/**
 * Approval capability — fail-closed (PRD).
 *
 * Audited as a pure client-side mock with no owner backend SDK. The fake
 * in-memory data, fake submissions and `clawchat_*` storage are removed:
 * every method throws a typed `ApprovalCapabilityUnavailableError` so any
 * page that reaches this surface (e.g. via an opt-in module selection) shows
 * a typed unavailable state instead of fabricated approvals.
 */

export type ApprovalStatus = "pending" | "approved" | "rejected" | "withdrawn";

export interface ApprovalActor {
  id: string;
  name: string;
  avatar: string;
  action?: "approve" | "reject";
  comment?: string;
  actionTime?: string;
}

export interface ApprovalItem {
  id: string;
  title: string;
  type: string; // "请假", "报销", etc.
  applicantId: string;
  applicant: string;
  avatar?: string;
  department?: string;
  date: string;
  content: string; // Detail description
  attachments?: { name: string; url: string }[];
  status: ApprovalStatus;
  currentStep?: string;
  history: ApprovalActor[]; // Workflow history
}

export interface SubmitApprovalRequest {
  title: string;
  type: string;
  content: string;
  attachments?: string[];
  approverIds: string[];
}

export interface HandleApprovalRequest {
  id: string;
  action: "approve" | "reject";
  comment: string;
}

export class ApprovalCapabilityUnavailableError extends Error {
  constructor(capability: string) {
    super(`${capability} is unavailable because its owner SDK is not composed.`);
    this.name = "ApprovalCapabilityUnavailableError";
  }
}

export class ApprovalService {
  /**
   * Fetch a list of approvals (e.g. pending ones for the current user)
   * @param filter Can be 'my-requests', 'pending-my-approval', 'handled'
   */
  static async getApprovals(
    _filter:
      | "my-requests"
      | "pending-my-approval"
      | "handled" = "pending-my-approval",
  ): Promise<ApprovalItem[]> {
    throw new ApprovalCapabilityUnavailableError("Approval list");
  }

  /**
   * Get detailed info for a single approval
   */
  static async getApprovalDetail(_id: string): Promise<ApprovalItem> {
    throw new ApprovalCapabilityUnavailableError("Approval detail");
  }

  /**
   * Submit a new approval request
   */
  static async submitApproval(
    _request: SubmitApprovalRequest,
  ): Promise<ApprovalItem> {
    throw new ApprovalCapabilityUnavailableError("Approval submission");
  }

  /**
   * Handle an approval request (approve or reject)
   */
  static async handleApproval(
    _request: HandleApprovalRequest,
  ): Promise<boolean> {
    throw new ApprovalCapabilityUnavailableError("Approval handling");
  }
}
