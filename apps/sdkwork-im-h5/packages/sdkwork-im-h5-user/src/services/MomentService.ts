import { useTranslation } from "react-i18next";
import type { User } from "@sdkwork/im-h5-types";

export interface Moment {
  id: string;
  author: User;
  content: string;
  images?: string[];
  video?: string;
  timestamp: number;
  likes: string[];
  comments: { id: string; authorName: string; content: string }[];
}

const INITIAL_MOMENTS: Moment[] = [
  {
    id: "mom_video1",
    author: {
      id: "u4",
      name: "Jacky",
      avatar: "https://picsum.photos/seed/jacky/200/200",
    },
    content: "分享一段好看的视频！",
    video: "https://www.w3schools.com/html/mov_bbb.mp4",
    timestamp: Date.now() - 1000 * 60 * 10,
    likes: ["u1"],
    comments: [],
  },
  {
    id: "mom1",
    author: {
      id: "u2",
      name: "Sarah Jenkins",
      avatar: "https://picsum.photos/seed/sarah/200/200",
      status: "online",
    },
    content: "今天天气真好！出去走走~ 🌞",
    images: [
      "https://picsum.photos/seed/mom1/400/300",
      "https://picsum.photos/seed/mom2/400/300",
    ],
    timestamp: Date.now() - 1000 * 60 * 30,
    likes: ["u3", "u4"],
    comments: [
      { id: "com1", authorName: "David Lee", content: "风景不错啊！" },
    ],
  },
  {
    id: "mom2",
    author: {
      id: "u3",
      name: "David Lee",
      avatar: "https://picsum.photos/seed/david/200/200",
    },
    content: "刚看完一本好书，推荐给大家《设计心理学》。",
    timestamp: Date.now() - 1000 * 60 * 60 * 2,
    likes: ["u1"],
    comments: [],
  },
];

const STORAGE_KEY = "sdkwork_im_h5_moments";

export let MOCK_MOMENTS: Moment[] = [];

const loadMoments = () => {
  if (MOCK_MOMENTS.length > 0) return MOCK_MOMENTS;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      MOCK_MOMENTS = JSON.parse(data);
    } else {
      MOCK_MOMENTS = [...INITIAL_MOMENTS];
      saveMoments();
    }
  } catch (e) {
    MOCK_MOMENTS = [...INITIAL_MOMENTS];
  }
  return MOCK_MOMENTS;
};

const saveMoments = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(MOCK_MOMENTS));
  } catch (e) {
    console.error("Failed to save moments", e);
  }
};

export const MomentService = {
  async getMoments(): Promise<Moment[]> {
    return [...loadMoments()];
  },

  async addMoment(
    content: string,
    images?: string[],
    video?: string,
    authorProfile?: Partial<User>,
  ): Promise<Moment> {
    loadMoments();
    const newMoment: Moment = {
      id: `mom${Date.now()}`,
      author: (authorProfile as User) || {
        id: "u1",
        name: "Alex Chen",
        avatar: "https://picsum.photos/seed/alex/200/200",
        status: "online",
      },
      content,
      images,
      video,
      timestamp: Date.now(),
      likes: [],
      comments: [],
    };
    MOCK_MOMENTS = [newMoment, ...MOCK_MOMENTS];
    saveMoments();
    return newMoment;
  },

  async toggleLike(momentId: string, userId: string): Promise<void> {
    loadMoments();
    const index = MOCK_MOMENTS.findIndex((m) => m.id === momentId);
    if (index !== -1) {
      const likes = [...MOCK_MOMENTS[index].likes];
      if (likes.includes(userId)) {
        MOCK_MOMENTS[index].likes = likes.filter((id) => id !== userId);
      } else {
        MOCK_MOMENTS[index].likes = [...likes, userId];
      }
      saveMoments();
    }
  },

  async addComment(
    momentId: string,
    authorName: string,
    content: string,
  ): Promise<void> {
    loadMoments();
    const index = MOCK_MOMENTS.findIndex((m) => m.id === momentId);
    if (index !== -1) {
      MOCK_MOMENTS[index].comments = [
        ...MOCK_MOMENTS[index].comments,
        { id: `com${Date.now()}`, authorName, content },
      ];
      saveMoments();
    }
  },

  async deleteMoment(momentId: string): Promise<void> {
    loadMoments();
    MOCK_MOMENTS = MOCK_MOMENTS.filter((m) => m.id !== momentId);
    saveMoments();
  },
};
