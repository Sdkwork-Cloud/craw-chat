import { useTranslation } from "react-i18next";
export interface VoiceSummaryRecord {
  id: string;
  title: string;
  date: string;
  duration: string;
  summary: string;
  keywords: string[];
}

const STORAGE_KEY = "sdkwork_im_h5_voice_summaries";

let mockSummaries: VoiceSummaryRecord[] = [];

const INITIAL_SUMMARIES: VoiceSummaryRecord[] = [
  {
    id: "1",
    title: "产品设计评审会",
    date: "昨天 14:00",
    duration: "45:20",
    summary:
      "本次评审主要对产品主打功能进行了讨论，决定削减部分边缘需求，聚焦核心体验。需要设计团队本周五前完成高保真UI调整。",
    keywords: ["设计评审", "功能削减", "周五交付"],
  },
  {
    id: "2",
    title: "客户随访录音",
    date: "2023-10-20",
    duration: "12:05",
    summary:
      "客户对新版本的响应速度表示满意，但提出了价格偏高的疑虑。销售团队需要跟进促销政策。",
    keywords: ["响应速度", "价格疑虑", "促销"],
  },
];

const loadSummaries = () => {
  if (mockSummaries.length > 0) return mockSummaries;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      mockSummaries = JSON.parse(data);
    } else {
      mockSummaries = [...INITIAL_SUMMARIES];
      saveSummaries();
    }
  } catch (e) {
    mockSummaries = [...INITIAL_SUMMARIES];
  }
  return mockSummaries;
};

const saveSummaries = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(mockSummaries));
  } catch (e) {}
};

export class VoiceSummaryService {
  static async getSummaries(): Promise<VoiceSummaryRecord[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...loadSummaries()]), 300),
    );
  }

  static async addSummary(
    summary: Omit<VoiceSummaryRecord, "id">,
  ): Promise<VoiceSummaryRecord> {
    loadSummaries();
    const newSummary: VoiceSummaryRecord = {
      ...summary,
      id: Math.random().toString(36).substr(2, 9),
    };
    mockSummaries = [newSummary, ...mockSummaries];
    saveSummaries();
    return newSummary;
  }
}
