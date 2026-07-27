import { useTranslation } from "react-i18next";
import React, { useState, useEffect, useRef } from "react";
import { useParams, useNavigate } from "react-router";
import { ChevronLeft, Users, Heart, Share2, Send, X, Video } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { CourseService, CourseData } from "../services/CourseService";

export const CourseLiveRoom: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [course, setCourse] = useState<CourseData | null>(null);
  const [messages, setMessages] = useState<{ id: string; user: string; text: string; isSystem?: boolean; type?: string }[]>([
    { id: "msg_1", user: "系统", text: "欢迎来到直播间！请大家文明发言。", isSystem: true },
    { id: "msg_2", user: "飞奔的蜗牛", text: "老师晚上好！" },
    { id: "msg_3", user: "前端小菜鸟", text: "终于开播啦，期待！" },
    { id: "msg_4", user: "用户9527", text: "签到签到！" },
  ]);
  const [inputText, setInputText] = useState("");
  const chatRef = useRef<HTMLDivElement>(null);
  const [likes, setLikes] = useState(1280);

  useEffect(() => {
    if (id) {
      CourseService.getCourseDetail(id).then(data => {
        if (data) setCourse(data);
      });
    }

    // Auto add some fake messages
    const interval = setInterval(() => {
       const msgs = ["支持老师！", "666", "这里的原理没太听懂", "太有用了", "感谢分享！", "这个架构设计很不错", "学到了学到了", "哈哈哈哈"];
       const users = ["李四", "Alice", "Bob", "码农小王", "架构师之路", "用户889"];
       setMessages(prev => [
           ...prev,
           {
               id: `msg_${Date.now()}`,
               user: users[Math.floor(Math.random() * users.length)],
               text: msgs[Math.floor(Math.random() * msgs.length)],
           }
       ]);
       setLikes(prev => prev + Math.floor(Math.random() * 5));
    }, 4000);

    return () => clearInterval(interval);
  }, [id]);

  useEffect(() => {
    if (chatRef.current) {
      chatRef.current.scrollTop = chatRef.current.scrollHeight;
    }
  }, [messages]);

  const handleSend = () => {
  if (!inputText.trim()) return;
    setMessages(prev => [
      ...prev,
      { id: `msg_${Date.now()}`, user: "我", text: inputText.trim(), type: "me" }
    ]);
    setInputText("");
  };

  const handleLike = () => {
  setLikes(prev => prev + 1);
  };

  if (!course) {
    return (
      <div className="flex flex-col h-[100dvh] bg-black items-center justify-center">
         <span className="text-white/50 text-[14px]">{t('course.auto_7f6f37e', '加载中...')}</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-[100dvh] bg-[#F2F2F7] dark:bg-black overflow-hidden relative pt-[env(safe-area-inset-top)]">
      {/* Video Area */}
      <div className="relative w-full aspect-video bg-black shrink-0 z-10 group">
         <video 
            src="https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/elephants-dream.mp4"
            className="w-full h-full object-contain"
            autoPlay 
            muted 
            loop 
            playsInline
         />
         
         {/* Live Overlay Header: Only Back Button */}
         <div className="absolute top-0 left-0 p-3 pointer-events-none z-20">
            <IconButton
              icon={<ChevronLeft className="w-6 h-6 text-white drop-shadow-md shrink-0" />}
              className="bg-black/30 backdrop-blur-md w-9 h-9 border border-white/10 hover:bg-black/50 transition-colors shrink-0 pointer-events-auto"
              onClick={() => navigate(-1)}
            />
         </div>
      </div>

      {/* Live Info Bar - Below Video */}
      <div className="bg-white dark:bg-[#1C1C1E] px-4 py-3 flex items-center justify-between shrink-0 shadow-sm z-10 relative border-b border-black/5 dark:border-white/5">
         <div className="flex items-center gap-3 flex-1 min-w-0">
             <div className="w-10 h-10 rounded-full overflow-hidden shrink-0 border border-black/5 dark:border-white/5 bg-gray-100 dark:bg-gray-800">
                <img src={course.cover} alt="avatar" className="w-full h-full object-cover" />
             </div>
             <div className="flex flex-col flex-1 min-w-0 justify-center">
                <div className="flex items-center gap-1.5 mb-0.5">
                   <span className="text-[14px] font-bold text-text-main truncate max-w-[120px]">{course.instructor}</span>
                   <div className="flex items-center gap-1 bg-red-50 text-red-500 dark:bg-red-500/10 text-[10px] px-1.5 py-[2px] rounded font-medium shrink-0">
                      <span className="w-1 h-1 rounded-full bg-red-500 animate-pulse" />LIVE
                   </div>
                </div>
                <span className="text-[12px] text-text-sub truncate leading-none">{course.title}</span>
             </div>
         </div>
         <div className="flex items-center gap-4 shrink-0 pl-2">
             <div className="flex flex-col items-end justify-center">
                <div className="flex items-center gap-1 text-text-sub">
                   <Users className="w-[14px] h-[14px]" />
                   <span className="text-[12px] font-medium leading-none mt-0.5">{course.students + 120}</span>
                </div>
                <span className="text-[10px] text-text-sub/70 mt-1">{t('course.auto_2c94fb71', '当前在线')}</span>
             </div>
             <button className="bg-red-500 hover:bg-red-600 text-white text-[13px] px-4 py-1.5 rounded-full font-medium transition-colors shadow-sm shadow-red-500/20 active:scale-95 shrink-0">{t('course.auto_1e4dea', '+ 关注')}</button>
         </div>
      </div>

      {/* Chat Area */}
      <div className="flex-1 flex flex-col bg-[#F2F2F7] dark:bg-[#121212] overflow-hidden relative">
         {/* Title Bar */}
         <div className="bg-white dark:bg-[#1C1C1E] p-3 border-b border-black/5 dark:border-white/5 shrink-0 flex items-center justify-between">
            <h1 className="text-[15px] font-bold text-text-main line-clamp-1">{course.title}</h1>
            <div className="bg-red-50 text-red-500 dark:bg-red-500/10 text-[11px] px-2 py-0.5 rounded font-medium shrink-0 ml-2 border border-red-500/20">{t('course.auto_7ae30252', '互动讨论区')}</div>
         </div>

         {/* Messages */}
         <div 
            className="flex-1 overflow-y-auto p-4 space-y-3 pb-4"
            ref={chatRef}
         >
            {messages.map((msg) => (
               <div key={msg.id} className={`flex items-start text-[14px] ${msg.isSystem ? 'justify-center' : ''}`}>
                  {msg.isSystem ? (
                     <div className="bg-black/5 dark:bg-white/10 text-text-sub px-3 py-1 rounded-full text-[12px] text-center max-w-[80%]">
                        {msg.text}
                     </div>
                  ) : (
                     <div className={`flex gap-2 max-w-[85%] ${msg.type === 'me' ? 'ml-auto flex-row-reverse' : ''}`}>
                        <div className={`flex flex-col ${msg.type === 'me' ? 'items-end' : 'items-start'}`}>
                           <span className="text-[11px] text-text-sub mb-0.5 px-1">{msg.user}</span>
                           <div className={`px-3 py-2 rounded-2xl ${msg.type === 'me' ? 'bg-blue-600 text-white rounded-tr-sm' : 'bg-white dark:bg-[#2A2A2D] text-text-main rounded-tl-sm border border-black/5 dark:border-white/5 shadow-sm'}`}>
                              {msg.text}
                           </div>
                        </div>
                     </div>
                  )}
               </div>
            ))}
         </div>

         {/* Input Area */}
         <div className="bg-white dark:bg-[#1C1C1E] border-t border-black/5 dark:border-white/5 px-3 pt-3 pb-[calc(env(safe-area-inset-bottom)+12px)] shrink-0 flex items-center gap-2">
            <div className="flex-1 bg-gray-100 dark:bg-[#2A2A2D] rounded-full h-10 flex items-center px-4">
               <input 
                  type="text" 
                  className="bg-transparent w-full h-full text-[14px] text-text-main outline-none placeholder:text-text-sub"
                  placeholder={t('course.auto_prop_22c9d37a', '参与互动讨论...')}
                  value={inputText}
                  onChange={(e) => setInputText(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleSend()}
               />
               <button 
                  className={`ml-2 w-7 h-7 flex items-center justify-center rounded-full transition-colors ${inputText.trim() ? 'bg-blue-600' : 'bg-transparent'}`}
                  onClick={handleSend}
               >
                  <Send className={`w-4 h-4 ${inputText.trim() ? 'text-white ml-[-1px] mb-[-1px]' : 'text-text-sub/50'}`} />
               </button>
            </div>
            <IconButton
               icon={<Share2 className="w-5 h-5 text-text-main" />}
               className="bg-gray-100 dark:bg-[#2A2A2D] w-10 h-10"
               onClick={() => {}}
            />
            <div className="relative">
               <IconButton
                  icon={<Heart className="w-5 h-5 text-red-500 fill-red-500" />}
                  className="bg-red-50 dark:bg-red-500/10 w-10 h-10"
                  onClick={handleLike}
               />
               <div className="absolute -top-1 -right-1 bg-red-500 text-white text-[9px] px-1 rounded-full font-bold shadow-sm pointer-events-none scale-90">
                  {likes >= 1000 ? `${(likes/1000).toFixed(1)}k` : likes}
               </div>
            </div>
         </div>
      </div>
    </div>
  );
};
