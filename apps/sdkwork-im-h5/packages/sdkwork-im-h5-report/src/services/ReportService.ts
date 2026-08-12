/**
 * Report capability — fail-closed (PRD).
 *
 * Audited as a pure client-side mock with no owner backend SDK. The fake
 * in-memory reports, fake submissions and `clawchat_*` storage are removed:
 * every method throws a typed `ReportCapabilityUnavailableError` so any page
 * that reaches this surface shows a typed unavailable state instead of
 * fabricated report data.
 */

export interface ReportItem {
  id: string;
  type: string;
  reporter: string;
  date: string;
  summary: string;
  isRead: boolean;
}

export class ReportCapabilityUnavailableError extends Error {
  constructor(capability: string) {
    super(`${capability} is unavailable because its owner SDK is not composed.`);
    this.name = "ReportCapabilityUnavailableError";
  }
}

export class ReportService {
  static async getReports(): Promise<ReportItem[]> {
    throw new ReportCapabilityUnavailableError("Report list");
  }

  static async submitReport(
    _report: Omit<ReportItem, "id" | "isRead">,
  ): Promise<ReportItem> {
    throw new ReportCapabilityUnavailableError("Report submission");
  }
}
