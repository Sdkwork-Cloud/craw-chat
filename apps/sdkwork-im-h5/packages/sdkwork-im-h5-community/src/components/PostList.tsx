import React from "react";
import { Post } from "../types";
import { PostItem } from "./PostItem";

interface PostListProps {
  posts: Post[];
  onLike: (postId: string) => void;
  onCommentClick: (postId: string) => void;
}

export const PostList: React.FC<PostListProps> = ({ posts, onLike, onCommentClick }) => {
  return (
    <div className="pb-24 flex flex-col gap-2 bg-chat-btn-bg dark:bg-black/50">
      {posts.map(post => (
        <PostItem
          key={post.id}
          post={post}
          onLike={onLike}
          onCommentClick={onCommentClick}
        />
      ))}
      {posts.length === 0 && (
        <div className="h-40 flex items-center justify-center text-text-sub">暂无动态</div>
      )}
    </div>
  );
};
