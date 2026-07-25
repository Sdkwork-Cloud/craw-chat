import { useTranslation } from "react-i18next";
import React, { useState, useRef, useEffect } from "react";
import { ChevronLeft, Info, MoreHorizontal, FileText, File, Video, Paperclip, Send } from "lucide-react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import { useNavigate, useParams } from "react-router";

export const NotarySessionChat: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const [messages, setMessages] = useState<any[]>([
    {
      id: "m1",
      sender: "notary",
      type: "text",
      content: t('notary.auto_mock_msg_1', "您好，我是为您办理本次公证的陈公证员，请问有什么可以帮您？"),
      time: "10:00",
    },
    {
      id: "m2",
      sender: "system",
      type: "system",
      content: t('notary.auto_mock_msg_2', "公证员已将状态更新为【材料审核中】"),
      time: "10:05",
    },
    {
      id: "m3",
      sender: "notary",
      type: "action",
      content: t('notary.auto_mock_msg_3', "请补充提交双方居住证明"),
      actionLabel: t('notary.auto_mock_msg_act', "前往上传"),
      time: "10:06",
    }
  ]);

  const [inputVal, setInputVal] = useState("");
  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSend = () => {
  if (!inputVal.trim()) return;
    setMessages((prev) => [
      ...prev,
      {
        id: `c_${Date.now()}`,
        sender: "user",
        type: "text",
        content: inputVal,
        time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false }),
      }
    ]);
    setInputVal("");
    
    // Mock reply
    setTimeout(() => {
       setMessages((prev) => [
         ...prev,
         {
           id: `c_${Date.now()}_r`,
           sender: "notary",
           type: "text",
           content: t('notary.auto_mock_msg_repl', "好的，收到您的消息。如有其他问题请随时留言。"),
           time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false }),
         }
       ]);
    }, 1500);
  };

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black fixed inset-0 z-50">
      <header className="h-[56px] flex items-center justify-between px-1 glass-header shrink-0 pt-safe relative border-b border-border-color">
        <div className="flex items-center z-10 w-16">
          <IconButton icon={<ChevronLeft />} onClick={() => navigate(-1)} />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center flex-col pointer-events-none">
          <h1 className="text-[17px] font-medium text-text-main">{t('notary.auto_45934bdb', "陈公证员")}</h1>
          <span className="text-[12px] text-green-500">{t('notary.auto_b0c97', "在线")}</span>
        </div>
        <div className="w-16 flex items-center justify-end px-2 z-10 gap-1">
           <IconButton icon={<MoreHorizontal className="w-5 h-5" />} />
        </div>
      </header>

      <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-4 pb-[80px]">
        {messages.map((m) => {
          if (m.type === "system") {
            return (
              <div key={m.id} className="flex justify-center w-full my-2">
                <span className="text-[12px] text-text-sub bg-black/5 dark:bg-white/10 px-3 py-1 rounded-full">
                  {m.content}
                </span>
              </div>
            );
          }

          const isMe = m.sender === "user";
          return (
            <div key={m.id} className={cn("flex w-full gap-3", isMe ? "justify-end" : "justify-start")}>
              {!isMe && (
                <div className="w-10 h-10 rounded-full bg-primary-blue/10 flex items-center justify-center shrink-0 border border-primary-blue/20">
                  <span className="font-bold text-primary-blue text-[14px]">{t('notary.auto_chen', "陈")}</span>
                </div>
              )}
              
              <div className={cn("flex flex-col max-w-[70%]", isMe ? "items-end" : "items-start")}>
                <div className="text-[11px] text-text-sub mb-1 px-1">{m.time}</div>
                <div 
                  className={cn(
                    "px-4 py-3 text-[15px] leading-relaxed relative",
                    isMe 
                      ? "bg-primary-blue text-white rounded-2xl rounded-tr-sm shadow-sm" 
                      : "bg-white dark:bg-[#1c1c1e] text-text-main rounded-2xl rounded-tl-sm shadow-sm border border-border-color/50"
                  )}
                >
                  {m.content}
                  
                  {m.type === "action" && (
                    <div className="mt-3 pt-3 border-t border-border-color/50 w-full flex justify-end">
                      <button 
                        onClick={() => navigate('/notary/files')}
                        className="text-[13px] font-medium text-primary-blue active:opacity-70 transition-opacity"
                      >
                        {m.actionLabel} &gt;
                      </button>
                    </div>
                  )}
                </div>
              </div>

              {isMe && (
                <div className="w-10 h-10 rounded-full bg-border-color overflow-hidden shrink-0">
                  <img src="https://picsum.photos/seed/me/200/200" alt="me" className="w-full h-full object-cover" />
                </div>
              )}
            </div>
          );
        })}
        <div ref={endRef} />
      </div>

      <div className="absolute bottom-0 left-0 right-0 bg-bg-color border-t border-border-color pb-safe px-3 pt-3 flex flex-col gap-2 shadow-[0_-5px_20px_rgba(0,0,0,0.02)]">
        <div className="flex gap-4 px-2 overflow-x-auto no-scrollbar py-1">
          <div className="text-[12px] bg-primary-blue/5 text-primary-blue border border-primary-blue/20 px-3 py-1.5 rounded-full whitespace-nowrap active:scale-95 transition-transform" onClick={() =>setInputVal(t('notary.auto_q1v', "请问视频公证需要准备什么？"))}>{t('notary.auto_27057f01', "询问注意事项")}</div>
          <div className="text-[12px] bg-primary-blue/5 text-primary-blue border border-primary-blue/20 px-3 py-1.5 rounded-full whitespace-nowrap active:scale-95 transition-transform" onClick={() =>setInputVal(t('notary.auto_q2v', "公证目前进展如何？"))}>{t('notary.auto_31593788', "查询进度")}</div>
        </div>
        <div className="flex items-center gap-2 mb-2">
           <IconButton icon={<Paperclip className="w-6 h-6 text-text-sub" />} />
           <div className="flex-1 min-h-[44px] bg-black/5 dark:bg-white/10 rounded-2xl px-4 py-2.5 flex items-center">
             <input 
               type="text" 
               className="w-full bg-transparent outline-none text-[15px] text-text-main"
               placeholder={t('notary.auto_prop_13864c35', "输入消息...")}
               value={inputVal}
               onChange={(e) => setInputVal(e.target.value)}
               onKeyDown={(e) => {
                 if (e.key === "Enter") handleSend();
               }}
             />
           </div>
           {inputVal.trim() ? (
             <button 
               className="w-10 h-10 rounded-full bg-primary-blue flex items-center justify-center text-white active:scale-95 transition-transform shadow-md shrink-0"
               onClick={handleSend}
             >
               <Send className="w-5 h-5 -ml-0.5" />
             </button>
           ) : (
             <IconButton icon={<Video className="w-6 h-6 text-text-sub" />} onClick={() => navigate(`/call/video-notary/${id || "c1"}`)} />
           )}
        </div>
      </div>
    </div>
  );
};
