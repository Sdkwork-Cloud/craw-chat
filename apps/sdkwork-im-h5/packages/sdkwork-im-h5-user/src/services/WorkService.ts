import { useTranslation } from "react-i18next";
export interface Work {
  id: string;
  type: "video" | "article" | "audio" | "ai_image";
  title: string;
  coverUrl: string;
  createdAt: string;
  views: number;
  likes: number;
  comments: number;
}

const INITIAL_WORKS: Work[] = [
  {
    id: "w1",
    type: "video",
    title: "春日露营Vlog",
    coverUrl:
      "https://images.unsplash.com/photo-1523987355523-c7b5b0dd90a7?auto=format&fit=crop&w=400&q=80",
    createdAt: "2026-04-15T10:00:00Z",
    views: 12500,
    likes: 856,
    comments: 42,
  },
  {
    id: "w2",
    type: "ai_image",
    title: "赛博朋克城市设定图",
    coverUrl:
      "https://images.unsplash.com/photo-1515630278258-407f66498911?auto=format&fit=crop&w=400&q=80",
    createdAt: "2026-04-10T14:30:00Z",
    views: 8900,
    likes: 1205,
    comments: 89,
  },
  {
    id: "w3",
    type: "article",
    title: "10个提升工作效率的小技巧",
    coverUrl:
      "https://images.unsplash.com/photo-1499750310107-5fef28a66643?auto=format&fit=crop&w=400&q=80",
    createdAt: "2026-04-05T09:15:00Z",
    views: 45000,
    likes: 3200,
    comments: 156,
  },
  {
    id: "w4",
    type: "audio",
    title: "雨天白噪音放松",
    coverUrl:
      "https://images.unsplash.com/photo-1515694346937-94d85e41e6f0?auto=format&fit=crop&w=400&q=80",
    createdAt: "2026-03-28T22:00:00Z",
    views: 3200,
    likes: 450,
    comments: 12,
  },
];

const STORAGE_KEY = "sdkwork_im_h5_my_works";

let MOCK_WORKS: Work[] = [];

const loadWorks = () => {
  if (MOCK_WORKS.length > 0) return MOCK_WORKS;
  try {
    const data = localStorage.getItem(STORAGE_KEY) ?? localStorage.getItem("clawchat_my_works");
    if (data) {
      MOCK_WORKS = JSON.parse(data);
    } else {
      MOCK_WORKS = [...INITIAL_WORKS];
      saveWorks();
    }
  } catch (e) {
    MOCK_WORKS = [...INITIAL_WORKS];
  }
  return MOCK_WORKS;
};

const saveWorks = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(MOCK_WORKS));
  } catch (e) {
    console.error("Failed to save works", e);
  }
};

export class WorkService {
  static async getMyWorks(): Promise<Work[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...loadWorks()]), 800),
    );
  }

  static async deleteWork(id: string): Promise<boolean> {
    return new Promise((resolve) =>
      setTimeout(() => {
        loadWorks();
        MOCK_WORKS = MOCK_WORKS.filter((w) => w.id !== id);
        saveWorks();
        resolve(true);
      }, 500),
    );
  }
  
  static async updateWork(id: string, updates: Partial<Work>): Promise<Work> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        loadWorks();
        const index = MOCK_WORKS.findIndex(w => w.id === id);
        if (index > -1) {
           MOCK_WORKS[index] = { ...MOCK_WORKS[index], ...updates };
           saveWorks();
           resolve(MOCK_WORKS[index]);
        } else {
           reject(new Error("Work not found"));
        }
      }, 500);
    });
  }

  static async addWork(work: Work): Promise<void> {
    return new Promise((resolve) =>
      setTimeout(() => {
        loadWorks();
        MOCK_WORKS = [work, ...MOCK_WORKS];
        saveWorks();
        resolve();
      }, 500),
    );
  }
}
