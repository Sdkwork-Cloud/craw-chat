import { useTranslation } from "react-i18next";
import React from "react";
import { Heart, MessageCircle, Send } from "lucide-react";
import { Avatar, cn } from "@sdkwork/im-h5-commons";
import { motion, AnimatePresence } from "motion/react";
import type { Moment } from "../../services/MomentService";

interface MomentItemCardProps {
  moment: Moment;
  activePopoverId: string | null;
  activeCommentId: string | null;
  commentText: string;
  setCommentText: (text: string) => void;
  formatTime: (ts: number) => string;
  setPreviewState: (state: any) => void;
  handleDelete: (id: string, e?: React.MouseEvent) => void;
  togglePopover: (id: string, e: React.MouseEvent) => void;
  handleLike: (id: string, e: React.MouseEvent) => void;
  openComment: (id: string, e: React.MouseEvent) => void;
  openReply: (id: string, author: string, e: React.MouseEvent) => void;
  submitComment: (id: string) => void;
}

export const MomentItemCard: React.FC<MomentItemCardProps> = ({
  moment,
  activePopoverId,
  activeCommentId,
  commentText,
  setCommentText,
  formatTime,
  setPreviewState,
  handleDelete,
  togglePopover,
  handleLike,
  openComment,
  openReply,
  submitComment,
}) => {
  const { t } = useTranslation();
return (
    <div className="flex gap-3 px-4 py-4 border-b border-black/5 dark:border-white/5 bg-white dark:bg-[#1C1C1E] active:bg-gray-50 dark:active:bg-white/5 transition-colors">
      <Avatar
        src={moment.author.avatar}
        size="md"
        className="w-10 h-10 rounded-md shrink-0 cursor-pointer active:opacity-70 border border-black/5 dark:border-white/5"
      />
      <div className="flex-1 min-w-0">
        <h3 className="text-[#576B95] dark:text-[#7d90a9] font-medium text-[16px] mb-1.5 cursor-pointer active:opacity-70 inline-block leading-none">
          {moment.author.name}
        </h3>
        {moment.content && (
          <p className="text-text-main text-[15px] leading-relaxed mb-2.5 break-words whitespace-pre-wrap">
            {moment.content}
          </p>
        )}

        {moment.images && moment.images.length > 0 && (
          <div
            className={cn(
              "mb-3",
              moment.images.length === 1
                ? "w-[65%] max-h-[220px]"
                : "grid grid-cols-3 gap-1 w-[85%]"
            )}
          >
            {moment.images.map((img, i) => (
              <div 
                 key={i} 
                 className={cn(
                   "overflow-hidden bg-gray-100 dark:bg-gray-800", 
                   moment.images!.length === 1 ? "w-auto max-w-full max-h-[220px]" : "w-full aspect-square"
                 )}
                 onClick={() => setPreviewState({ type: 'images', images: moment.images!, index: i })}
              >
                <img
                  src={img}
                  alt="Moment"
                  className="w-full h-full object-cover cursor-pointer active:opacity-80 transition-opacity"
                />
              </div>
            ))}
          </div>
        )}
        
        {moment.video && (
          <div className="mb-3 w-[65%] max-h-[220px] relative rounded overflow-hidden bg-black cursor-pointer active:opacity-90 transition-opacity"
               onClick={() => setPreviewState({ type: 'video', url: moment.video! })}
          >
             <video 
                src={moment.video} 
                className="w-full h-full object-cover max-h-[220px]"
                playsInline
                muted
             />
             <div className="absolute inset-0 flex items-center justify-center bg-black/20 pointer-events-none">
                <div className="w-10 h-10 rounded-full bg-black/40 border border-white/40 backdrop-blur-sm flex items-center justify-center">
                   <div className="w-0 h-0 border-t-[6px] border-t-transparent border-l-[10px] border-l-white border-b-[6px] border-b-transparent ml-1" />
                </div>
             </div>
          </div>
        )}

        <div className="flex items-center justify-between mt-1 mb-1 relative h-6">
          <div className="flex items-center gap-3">
            <span className="text-text-sub text-[13px]">
              {formatTime(moment.timestamp)}
            </span>
            {moment.author.id === "u1" && (
              <button 
                className="text-[#576B95] dark:text-[#7d90a9] text-[13px] active:opacity-50"
                onClick={(e) => handleDelete(moment.id, e)}
              >删除</button>
            )}
          </div>

          {/* Interaction popover container */}
          <div className="relative">
            <button
              onClick={(e) => togglePopover(moment.id, e)}
              className="w-8 h-5 bg-[#F1F1F2] dark:bg-[#2A2A2D] rounded flex items-center justify-center shrink-0 active:bg-gray-200 dark:active:bg-white/10 transition-colors"
            >
              <span className="w-1 h-1 bg-text-sub rounded-full mx-[1.5px]" />
              <span className="w-1 h-1 bg-text-sub rounded-full mx-[1.5px]" />
            </button>

            <AnimatePresence>
              {activePopoverId === moment.id && (
                <motion.div
                  initial={{ opacity: 0, scale: 0.9, x: 10 }}
                  animate={{ opacity: 1, scale: 1, x: 0 }}
                  exit={{ opacity: 0, scale: 0.9, x: 10 }}
                  transition={{ duration: 0.15 }}
                  className="absolute right-10 top-1/2 -translate-y-1/2 bg-[#4C4C4C] dark:bg-[#333333] rounded overflow-hidden flex items-center text-white h-[38px] shadow-xl divide-x divide-white/20 origin-right whitespace-nowrap z-20"
                  onClick={(e) => e.stopPropagation()}
                >
                  <button 
                    className="flex items-center justify-center gap-1.5 px-6 h-full active:bg-[#3C3C3C] dark:active:bg-[#222222] transition-colors min-w-[76px]"
                    onClick={(e) => handleLike(moment.id, e)}
                  >
                    <Heart className={cn("w-4 h-4", moment.likes?.includes("u1") && "fill-current text-white")} />
                    <span className="text-[13px] font-medium">{moment.likes?.includes("u1") ? "取消" : "赞"}</span>
                  </button>
                  <button 
                    className="flex items-center justify-center gap-1.5 px-6 h-full active:bg-[#3C3C3C] dark:active:bg-[#222222] transition-colors min-w-[76px]"
                    onClick={(e) => openComment(moment.id, e)}
                  >
                    <MessageCircle className="w-4 h-4" />
                    <span className="text-[13px] font-medium">{t('user.auto_117876', `评论`)}</span>
                  </button>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>

        {/* Likes and Comments Area */}
        {(moment.likes?.length > 0 ||
          moment.comments?.length > 0) && (
          <div className="bg-[#F3F3F5] dark:bg-[#202022] rounded text-[13px] relative mt-2.5 overflow-visible">
            {/* Up arrow triangle */}
            <div className="absolute -top-1.5 left-3 w-3 h-3 bg-[#F3F3F5] dark:bg-[#202022] rotate-45 transform origin-center" />
            
            <div className="relative z-10 p-2.5 space-y-1.5">
              {moment.likes?.length > 0 && (
                <div className="flex items-start gap-1.5 text-[#576B95] dark:text-[#7d90a9] font-medium leading-relaxed">
                  <Heart className="w-[14px] h-[14px] mt-0.5 shrink-0 fill-current" />
                  <span className="break-words">Alex Chen, 以及 {moment.likes.length} 人</span>
                </div>
              )}

              {moment.likes?.length > 0 &&
                moment.comments?.length > 0 && (
                  <div className="h-[1px] bg-black/5 dark:bg-white/5 my-1" />
                )}

              {moment.comments?.length > 0 && (
                <div className="flex flex-col gap-1 text-[13.5px] text-text-main leading-relaxed">
                  {moment.comments.map((c) => (
                    <div key={c.id} className="break-words" onClick={(e) => openReply(moment.id, c.authorName, e)}>
                      <span className="text-[#576B95] dark:text-[#7d90a9] font-medium cursor-pointer active:opacity-70">
                        {c.authorName}
                      </span>{c.content.startsWith("回复 ") && c.content.includes(":") ? (<>
                          <span className="text-text-main mx-1">{t('user.auto_addef', `回复`)}</span>
                          <span className="text-[#576B95] dark:text-[#7d90a9] font-medium cursor-pointer active:opacity-70">{c.content.split(":")[0].replace("回复 ", "")}</span>
                          <span>: {c.content.substring(c.content.indexOf(":") + 1)}</span>
                        </>
                      ) : (
                        <span>: {c.content}</span>
                      )}
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}

        {/* Inline Comment Input */}
        <AnimatePresence>
          {activeCommentId === moment.id && (
            <motion.div
              initial={{ opacity: 0, height: 0, marginTop: 0 }}
              animate={{ opacity: 1, height: "auto", marginTop: 12 }}
              exit={{ opacity: 0, height: 0, marginTop: 0 }}
              className="overflow-hidden"
            >
              <div className="flex items-center gap-2 bg-[#F3F3F5] dark:bg-[#2A2A2D] rounded-lg px-3 py-2">
                <textarea
                  placeholder={t('user.auto_prop_eee21c8', "评论...")}
                  className="bg-transparent flex-1 outline-none text-[14px] text-text-main resize-none min-h-[20px] max-h-[80px]"
                  value={commentText}
                  rows={1}
                  onChange={(e) => setCommentText(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && !e.shiftKey && (e.preventDefault(), submitComment(moment.id))}
                  autoFocus
                />
                <button
                  className="text-[#576B95] dark:text-[#7d90a9] disabled:opacity-30 p-1 shrink-0 self-end"
                  disabled={!commentText.trim()}
                  onClick={() => submitComment(moment.id)}
                >
                  <Send className="w-5 h-5" />
                </button>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
};
