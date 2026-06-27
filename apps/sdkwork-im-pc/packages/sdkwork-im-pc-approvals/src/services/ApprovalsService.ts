export interface ApprovalItem {
  id: string;
  title: string;
  type: 'leave' | 'reimbursement' | 'purchase' | 'other';
  status: 'pending' | 'approved' | 'rejected' | 'revoked';
  applicant: { id: string; name: string; avatar?: string };
  submitTime: string;
  amount?: number;
  description: string;
  attachments?: { name: string; size: string }[];
}

const PC_APPROVALS_CONTRACT_UNAVAILABLE = 'pc approvals contract is not available';

function failClosedApprovalsMutation(): never {
  throw new Error(PC_APPROVALS_CONTRACT_UNAVAILABLE);
}

class SdkworkApprovalsService {
  async getPendingApprovals(): Promise<ApprovalItem[]> {
    return [];
  }

  async getHandledApprovals(): Promise<ApprovalItem[]> {
    return [];
  }

  async getMyRequests(): Promise<ApprovalItem[]> {
    return [];
  }

  async approve(_id: string, _comment: string): Promise<void> {
    failClosedApprovalsMutation();
  }

  async reject(_id: string, _comment: string): Promise<void> {
    failClosedApprovalsMutation();
  }

  async submitApproval(_item: ApprovalItem): Promise<ApprovalItem> {
    failClosedApprovalsMutation();
  }
}

export const ApprovalsService = new SdkworkApprovalsService();
