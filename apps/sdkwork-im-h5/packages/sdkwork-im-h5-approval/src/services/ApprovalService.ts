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
  type: string;
  applicantId: string;
  applicant: string;
  avatar?: string;
  department?: string;
  date: string;
  content: string;
  attachments?: { name: string; url: string }[];
  status: ApprovalStatus;
  currentStep?: string;
  history: ApprovalActor[];
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
  constructor() {
    super("Approval is unavailable because its owner SDK is not composed.");
    this.name = "ApprovalCapabilityUnavailableError";
  }
}

export class ApprovalService {
  static async getApprovals(
    _filter: "my-requests" | "pending-my-approval" | "handled" = "pending-my-approval",
  ): Promise<ApprovalItem[]> {
    throw new ApprovalCapabilityUnavailableError();
  }

  static async getApprovalDetail(_id: string): Promise<ApprovalItem> {
    throw new ApprovalCapabilityUnavailableError();
  }

  static async submitApproval(_request: SubmitApprovalRequest): Promise<ApprovalItem> {
    throw new ApprovalCapabilityUnavailableError();
  }

  static async handleApproval(_request: HandleApprovalRequest): Promise<boolean> {
    throw new ApprovalCapabilityUnavailableError();
  }
}
