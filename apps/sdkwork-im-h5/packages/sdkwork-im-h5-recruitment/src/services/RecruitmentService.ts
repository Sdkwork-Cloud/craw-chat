import { useTranslation } from "react-i18next";
export interface CandidateRecord {
  id: string;
  name: string;
  jobTitle: string;
  stage: string;
  date: string;
  avatar?: string;
  experience: string;
  education: string;
}

const STORAGE_KEY = "sdkwork_im_h5_recruitment_candidates";

const INITIAL_CANDIDATES: CandidateRecord[] = [
  {
    id: "1",
    name: "刘备",
    jobTitle: "高级前端工程师",
    stage: "一面安排中",
    date: "今天 14:00",
    experience: "5年",
    education: "本科",
    avatar: "https://picsum.photos/seed/1/200",
  },
  {
    id: "2",
    name: "曹操",
    jobTitle: "产品经理",
    stage: "待复试",
    date: "明天 10:30",
    experience: "8年",
    education: "硕士",
    avatar: "https://picsum.photos/seed/2/200",
  },
  {
    id: "3",
    name: "孙权",
    jobTitle: "UI/UX 设计师",
    stage: "已录用",
    date: "本周入职",
    experience: "3年",
    education: "本科",
    avatar: "https://picsum.photos/seed/3/200",
  },
];

let mockCandidates: CandidateRecord[] = [];

function loadData() {
  if (mockCandidates.length > 0) return mockCandidates;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      mockCandidates = JSON.parse(data);
    } else {
      mockCandidates = [...INITIAL_CANDIDATES];
    }
  } catch (e) {
    mockCandidates = [...INITIAL_CANDIDATES];
  }
  return mockCandidates;
}

function saveData() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(mockCandidates));
}

export class RecruitmentService {
  static async getCandidates(): Promise<CandidateRecord[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...loadData()]), 300),
    );
  }
  static async updateCandidateStage(id: string, stage: string): Promise<void> {
    return new Promise((resolve) => {
      loadData();
      const index = mockCandidates.findIndex((c) => c.id === id);
      if (index >= 0) {
        mockCandidates[index].stage = stage;
        saveData();
      }
      setTimeout(resolve, 300);
    });
  }
  static async deleteCandidate(id: string): Promise<void> {
    return new Promise((resolve) => {
      loadData();
      mockCandidates = mockCandidates.filter((c) => c.id !== id);
      saveData();
      setTimeout(resolve, 300);
    });
  }
}
