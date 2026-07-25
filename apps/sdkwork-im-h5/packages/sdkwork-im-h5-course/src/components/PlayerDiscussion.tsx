import { useTranslation } from "react-i18next";
import React, { useState, useEffect, useRef } from "react";
import { ThumbsUp, MessageSquare, Send } from "lucide-react";
import { CourseService, CourseDiscussion } from "../services/CourseService";
import { showToast } from "@sdkwork/im-h5-commons";

export interface PlayerDiscussionProps {
  courseId: string;
  lessonId?: string;
}

export const PlayerDiscussion: React.FC<PlayerDiscussionProps> = ({ courseId, lessonId }) => {
  const { t } = useTranslation();
const [discussions, setDiscussions] = useState<CourseDiscussion[]>([]);
  const [loading, setLoading] = useState(true);
  const [inputVal, setInputVal] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const fetchDiscussions = async () => {
      setLoading(true);
      try {
        const data = await CourseService.getCourseDiscussions(courseId, lessonId);
        setDiscussions([...data]);
      } catch (error) {
        console.error("Failed to fetch discussions", error);
      } finally {
        setLoading(false);
      }
    };
    fetchDiscussions();
  }, [courseId, lessonId]);

  const handleSubmit = async () => {
    if (!inputVal.trim() || submitting) return;
    setSubmitting(true);
    try {
      const newComment = await CourseService.postDiscussion(courseId, lessonId, inputVal);
      setDiscussions(prev => [newComment, ...prev]);
      setInputVal("");
    } catch (e) {
      showToast(t('course.auto_fn_nb6b6249', '发送失败，请重试'));
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <>
      <div className="p-4 mt-2 bg-white dark:bg-[#1C1C1E] min-h-full" ref={scrollRef}>
         {loading ? (
             <div className="flex items-center justify-center p-8 text-text-sub text-[14px]">{t('course.auto_7f6f37e', '加载中...')}</div>
         ) : discussions.length === 0 ? (
             <div className="flex items-center justify-center p-8 text-text-sub text-[14px]">{t('course.auto_45d0d76', '暂无讨论，来抢沙发吧')}</div>
         ) : (
            <div className="flex flex-col gap-5">
               {discussions.map(item => (
                 <div key={item.id} className="flex items-start gap-3">
                   <img src={item.user.avatar} className="w-8 h-8 rounded-full bg-black/5" alt="avatar" />
                   <div className="flex-1 border-b border-black/5 dark:border-white/5 pb-4">
                      <div className="text-[13px] font-medium text-text-sub mb-1">{item.user.name}</div>
                      <div className="text-[14px] text-text-main leading-relaxed">{item.content}</div>
                      
                      {item.reply && (
                        <div className="mt-3 bg-[#F2F2F7] dark:bg-[#2A2A2D] p-3 rounded-xl flex items-start gap-2">
                          <span className="text-[12px] text-blue-500 font-medium shrink-0">{item.reply.author}:</span>
                          <span className="text-[13px] text-text-main leading-relaxed">{item.reply.content}</span>
                        </div>
                      )}

                      <div className="flex flex-wrap items-center gap-4 mt-3 text-[12px] text-text-sub">
                         <div className="flex items-center gap-1 cursor-pointer"><ThumbsUp className="w-3.5 h-3.5" /> {item.likes || ''}</div>
                         <div className="flex items-center gap-1 cursor-pointer"><MessageSquare className="w-3.5 h-3.5" />{t('course.auto_addef', '回复')}</div>
                         <div className="ml-auto">{item.time}</div>
                      </div>
                   </div>
                 </div>
               ))}
            </div>
         )}
      </div>
      
      {/* Discussion Input box floating at bottom */}
      <div className="sticky bottom-0 left-0 right-0 bg-white dark:bg-[#1C1C1E] border-t border-black/5 dark:border-white/5 p-3 pb-safe flex items-center gap-3 z-20">
         <div className="flex-1 bg-[#F2F2F7] dark:bg-[#2A2A2D] rounded-full px-4 py-2 flex items-center gap-2">
            <input 
              className="flex-1 bg-transparent border-none outline-none text-[14px] text-text-main placeholder:text-text-sub h-6"
              placeholder={t('course.auto_prop_n22e56ef0', '参与讨论...')}
              value={inputVal}
              onChange={(e) => setInputVal(e.target.value)}
              onKeyDown={(e) => {
                 if (e.key === 'Enter') handleSubmit();
              }}
            />
            {inputVal.trim() && (
               <button 
                 onClick={handleSubmit}
                 disabled={submitting}
                 className="w-7 h-7 flex items-center justify-center rounded-full bg-blue-500 text-white shrink-0 active:scale-95 disabled:opacity-50"
               >
                  <Send className="w-3.5 h-3.5" />
               </button>
            )}
         </div>
      </div>
    </>
  );
};
