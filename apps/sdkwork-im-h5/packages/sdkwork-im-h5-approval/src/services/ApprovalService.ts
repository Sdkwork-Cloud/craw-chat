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

const STORAGE_KEY = "sdkwork_im_h5_approvals";

let mockApprovals: ApprovalItem[] = [];

const INITIAL_APPROVALS: ApprovalItem[] = [
  {
    id: "1",
    title: "年假申请",
    type: "请假",
    applicantId: "u1",
    applicant: "张三",
    department: "研发部",
    date: "2023-10-24",
    content: "因个人事务申请年假3天，从10月25日至10月27日。",
    status: "pending",
    history: [
      { id: "u1", name: "张三", avatar: "", actionTime: "2023-10-24 10:00:00" },
    ],
  },
  {
    id: "2",
    title: "差旅费报销",
    type: "报销",
    applicantId: "u2",
    applicant: "李四",
    department: "销售部",
    date: "2023-10-23",
    content: "10月份前往北京出差高铁票与住宿费报销，共计2400元。",
    status: "approved",
    history: [],
  },
  {
    id: "3",
    title: "物资采购申请",
    type: "采购",
    applicantId: "u3",
    applicant: "王五",
    department: "行政部",
    date: "2023-10-22",
    content: "申请采购新员工办公电脑两台。",
    status: "rejected",
    history: [],
  },
];

const loadApprovals = () => {
  if (mockApprovals.length > 0) return mockApprovals;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      mockApprovals = JSON.parse(data);
    } else {
      mockApprovals = [...INITIAL_APPROVALS];
      saveApprovals();
    }
  } catch (e) {
    mockApprovals = [...INITIAL_APPROVALS];
  }
  return mockApprovals;
};

const saveApprovals = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(mockApprovals));
  } catch (e) {}
};

export class ApprovalService {
  /**
   * Fetch a list of approvals (e.g. pending ones for the current user)
   * @param filter Can be 'my-requests', 'pending-my-approval', 'handled'
   */
  static async getApprovals(
    filter:
      | "my-requests"
      | "pending-my-approval"
      | "handled" = "pending-my-approval",
  ): Promise<ApprovalItem[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const items = loadApprovals();
        resolve(
          items.filter((item) =>
            filter === "handled"
              ? item.status !== "pending"
              : filter === "my-requests"
                ? item.applicantId === "me"
                : item.status === "pending",
          ),
        );
      }, 300);
    });
  }

  /**
   * Get detailed info for a single approval
   */
  static async getApprovalDetail(id: string): Promise<ApprovalItem> {
    const items = loadApprovals();
    const item = items.find((i) => i.id === id) || items[0];
    return { ...item };
  }

  /**
   * Submit a new approval request
   */
  static async submitApproval(
    request: SubmitApprovalRequest,
  ): Promise<ApprovalItem> {
    loadApprovals();
    const newItem: ApprovalItem = {
      id: Math.random().toString(36).substring(7),
      title: request.title,
      type: request.type,
      content: request.content,
      applicantId: "me",
      applicant: "我",
      date: new Date().toISOString(),
      status: "pending",
      history: [],
    };
    mockApprovals = [newItem, ...mockApprovals];
    saveApprovals();
    return newItem;
  }

  /**
   * Hande an approval request (approve or reject)
   */
  static async handleApproval(
    request: HandleApprovalRequest,
  ): Promise<boolean> {
    loadApprovals();
    const index = mockApprovals.findIndex((a) => a.id === request.id);
    if (index !== -1) {
      mockApprovals[index].status =
        request.action === "approve" ? "approved" : "rejected";
      mockApprovals[index].history.push({
        id: "me",
        name: "我",
        avatar: "",
        action: request.action,
        comment: request.comment,
        actionTime: new Date().toISOString(),
      });
      saveApprovals();
    }
    return true;
  }
}
