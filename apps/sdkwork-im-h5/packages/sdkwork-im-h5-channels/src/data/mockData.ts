import { useTranslation } from "react-i18next";
import { CreativeWork } from "../types";

export const CREATIVE_WORKS: CreativeWork[] = [
  {
    id: 1,
    type: "video",
    title: "未来已来：用生成式大模型构建下一个颠覆级应用",
    author: "AI 架构师甲",
    avatar: "https://picsum.photos/seed/a1/100",
    likes: 12400,
    comments: 856,
    shares: 3200,
    remixes: 124,
    bg: "bg-blue-900",
    mediaUrl: "https://www.w3schools.com/html/mov_bbb.mp4"
  },
  {
    id: 2,
    type: "image",
    title: "AIGC 赛博朋克城市设定图初探，MIDJOURNEY V6生成",
    author: "赛博纪元",
    avatar: "https://picsum.photos/seed/a2/100",
    likes: 8900,
    comments: 231,
    shares: 1400,
    remixes: 56,
    bg: "bg-emerald-900",
    mediaUrl: "https://picsum.photos/seed/cybp/800/1200"
  },
  {
    id: 3,
    type: "video",
    title: "Sora 级视频生成原理解析：时空碎片化的力量",
    author: "深度学习漫游",
    avatar: "https://picsum.photos/seed/a3/100",
    likes: 45000,
    comments: 3200,
    shares: 12000,
    remixes: 4000,
    bg: "bg-rose-900",
    mediaUrl: "https://www.w3schools.com/html/mov_bbb.mp4"
  },
  {
    id: 4,
    type: "image",
    title: "AI 生成：超现实主义数字艺术展作品集",
    author: "DigitalArt",
    avatar: "https://picsum.photos/seed/a4/100",
    likes: 3100,
    comments: 112,
    shares: 400,
    remixes: 12,
    bg: "bg-purple-900",
    mediaUrl: "https://picsum.photos/seed/art3/800/1000"
  }
];
