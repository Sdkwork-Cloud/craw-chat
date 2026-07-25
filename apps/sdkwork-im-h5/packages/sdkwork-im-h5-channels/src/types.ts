export interface CreativeWork {
  id: string | number;
  type: "video" | "image";
  title: string;
  author: string;
  avatar: string;
  likes: number;
  comments: number;
  shares: number;
  remixes: number;
  bg?: string;
  mediaUrl: string;
  heightClass?: string;
}
