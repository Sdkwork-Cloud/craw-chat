export interface ReportItem {
  id: string;
  type: string;
  reporter: string;
  date: string;
  summary: string;
  isRead: boolean;
}

export class ReportCapabilityUnavailableError extends Error {
  constructor() {
    super("Report is unavailable because its owner SDK is not composed.");
    this.name = "ReportCapabilityUnavailableError";
  }
}

export class ReportService {
  static async getReports(): Promise<ReportItem[]> {
    throw new ReportCapabilityUnavailableError();
  }

  static async submitReport(
    _report: Omit<ReportItem, "id" | "isRead">,
  ): Promise<ReportItem> {
    throw new ReportCapabilityUnavailableError();
  }
}
