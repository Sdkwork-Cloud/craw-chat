export interface ReportItem {
  id: string;
  type: string;
  reporter: string;
  date: string;
  summary: string;
  isRead: boolean;
}

const STORAGE_KEY = "sdkwork_im_h5_reports";

let mockReports: ReportItem[] = [];

const INITIAL_REPORTS: ReportItem[] = [
  {
    id: "1",
    type: "日报",
    reporter: "张三",
    date: "今天 18:30",
    summary: "完成了首页UI重构，修复了3个P2级别Bug。",
    isRead: false,
  },
  {
    id: "2",
    type: "周报",
    reporter: "李四",
    date: "昨天 17:45",
    summary: "本周主要推进了支付模块的开发，进度达80%。下周计划...",
    isRead: true,
  },
  {
    id: "3",
    type: "月报",
    reporter: "王五",
    date: "10-24 09:30",
    summary: "十月份销售业绩达标，新增客户20家。",
    isRead: true,
  },
];

const loadReports = () => {
  if (mockReports.length > 0) return mockReports;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      mockReports = JSON.parse(data);
    } else {
      mockReports = [...INITIAL_REPORTS];
      saveReports();
    }
  } catch (e) {
    mockReports = [...INITIAL_REPORTS];
  }
  return mockReports;
};

const saveReports = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(mockReports));
  } catch (e) {}
};

export class ReportService {
  static async getReports(): Promise<ReportItem[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve([...loadReports()]);
      }, 300);
    });
  }

  static async submitReport(
    report: Omit<ReportItem, "id" | "isRead">,
  ): Promise<ReportItem> {
    loadReports();
    const newItem: ReportItem = {
      ...report,
      id: Math.random().toString(36).substring(7),
      isRead: true,
    };
    mockReports = [newItem, ...mockReports];
    saveReports();
    return newItem;
  }
}
