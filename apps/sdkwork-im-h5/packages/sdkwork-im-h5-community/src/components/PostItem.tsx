import React from "react";
import { Post } from "../types";
import { cn } from "@sdkwork/im-h5-commons";
import { Heart, MessageCircle } from "lucide-react";

interface PostItemProps {
  post: Post;
  onLike: (postId: string) => void;
  onCommentClick: (postId: string) => void;
}

export const PostItem: React.FC<PostItemProps> = ({ post, onLike, onCommentClick }) => {
  return (
    <div className="bg-white dark:bg-[#1C1C1E] px-4 pt-4 pb-3">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-3">
          <img src={post.authorAvatar} alt="" className="w-10 h-10 rounded-full border border-black/5 dark:border-white/5" />
          <div className="flex flex-col">
            <span className="text-[15px] font-bold text-text-main">{post.authorName}</span>
            <span className="text-[12px] text-text-sub">{new Date(post.createdAt).toLocaleString()}</span>
          </div>
        </div>
      </div>
      <p className="text-[15px] text-text-main leading-relaxed mb-3 whitespace-pre-wrap">{post.content}</p>
      {post.images && post.images.length > 0 && (
        <div className="grid grid-cols-3 gap-2 mb-3">
          {post.images.map((img, idx) => (
            <img key={idx} src={img} alt="" className="w-full aspect-square object-cover rounded-xl border border-black/5 dark:border-white/5" />
          ))}
        </div>
      )}
      
      <div className="flex items-center gap-6 mt-2 pt-3 border-t border-black/5 dark:border-white/5 text-text-sub">
        <div 
          className="flex items-center gap-1.5 cursor-pointer active:opacity-70 transition-opacity"
          onClick={() => onLike(post.id)}
        >
          <Heart className={cn("w-5 h-5", post.isLiked ? "text-rose-500 fill-rose-500" : "")} />
          <span className={cn("text-[13px]", post.isLiked && "text-rose-500")}>{post.likes > 0 ? post.likes : '赞'}</span>
        </div>
        <div 
          className="flex items-center gap-1.5 cursor-pointer active:opacity-70 transition-opacity"
          onClick={() => onCommentClick(post.id)}
        >
          <MessageCircle className="w-5 h-5" />
          <span className="text-[13px]">{post.comments > 0 ? post.comments : '评论'}</span>
        </div>
      </div>

      {post.commentsList && post.commentsList.length > 0 && (
        <div className="mt-3 bg-chat-active-bg dark:bg-white/5 rounded-xl p-3 flex flex-col gap-1.5">
          {post.commentsList.slice(0, 3).map(comment => (
            <span key={comment.id} className="text-[13px] text-text-main">
              <span className="font-semibold text-blue-500">{comment.authorName}</span>: {comment.content}
            </span>
          ))}
          {post.commentsList.length > 3 && (
            <span className="text-[13px] text-blue-500 font-medium cursor-pointer">查看全部 {post.comments} 条评论</span>
          )}
        </div>
      )}
    </div>
  );
};
