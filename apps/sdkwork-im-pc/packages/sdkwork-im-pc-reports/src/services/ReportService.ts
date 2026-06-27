export interface ReportItem {
  id: string;
  type: 'daily' | 'weekly' | 'monthly';
  content: string;
  plan: string;
  author: string;
  date: string;
  hasRead: boolean;
}

const PC_REPORTS_CONTRACT_UNAVAILABLE = 'pc reports contract is not available';

function failClosedReportsMutation(): never {
  throw new Error(PC_REPORTS_CONTRACT_UNAVAILABLE);
}

class SdkworkReportService {
  async getReports(): Promise<ReportItem[]> {
    return [];
  }

  async submitReport(
    _type: 'daily' | 'weekly' | 'monthly',
    _content: string,
    _plan: string,
  ): Promise<ReportItem> {
    failClosedReportsMutation();
  }
}

export const reportService = new SdkworkReportService();
