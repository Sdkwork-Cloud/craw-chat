import React, { useRef } from 'react';
import { Maximize2, X, Bold, Italic, Underline, Paperclip, Image as ImageIcon, Trash2, Send, Edit3 } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { toast } from '@sdkwork/im-pc-chat';
import { mailService } from '../services/MailService';

interface ComposeMailModalProps {
  isComposing: boolean;
  setIsComposing: (v: boolean) => void;
  isComposeMaximized: boolean;
  setIsComposeMaximized: (v: boolean) => void;
  isRichText: boolean;
  setIsRichText: (v: boolean) => void;
  composeData: { to: string; subject: string; body: string };
  setComposeData: (data: any) => void;
  attachments: File[];
  setAttachments: (files: File[]) => void;
  currentFolder: string;
  loadMails: () => void;
}

export const ComposeMailModal: React.FC<ComposeMailModalProps> = ({
  isComposing, setIsComposing, isComposeMaximized, setIsComposeMaximized,
  isRichText, setIsRichText, composeData, setComposeData, attachments, setAttachments,
  currentFolder, loadMails
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  if (!isComposing) return null;

  const handleAttachment = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setAttachments([...attachments, ...Array.from(e.target.files)]);
    }
    e.target.value = '';
  };

  const handleSend = async () => {
    if (!composeData.to || !composeData.subject) {
      toast('请输入收件人和主题', 'error');
      return;
    }
    try {
      await mailService.sendMail({
        subject: composeData.subject,
        bodyHtml: composeData.body,
        previewText: composeData.body.substring(0, 50),
      });
      toast('邮件发送成功', 'success');
      setIsComposing(false);
      setComposeData({ to: '', subject: '', body: '' });
      if (currentFolder === 'sent') loadMails();
    } catch (e) {
      toast('发送失败', 'error');
    }
  };

  return (
    <div className={cn(
      "absolute bg-[#1e1e1e] border border-white/10 rounded-xl shadow-2xl flex flex-col overflow-hidden animate-in slide-in-from-bottom-8 duration-300 z-50 transition-all",
      isComposeMaximized ? "inset-4" : "right-8 bottom-8 w-[500px] h-[600px]"
    )}>
       <div className="h-12 bg-[#2b2b2d] border-b border-white/10 flex items-center justify-between px-4 shrink-0 cursor-move">
          <span className="text-sm font-medium text-gray-200">新邮件</span>
          <div className="flex gap-2">
             <button className="text-gray-500 hover:text-white p-1 rounded-md transition-colors" onClick={() => setIsComposeMaximized(!isComposeMaximized)}>
               <Maximize2 size={16} />
             </button>
             <button onClick={() => { setIsComposing(false); setIsComposeMaximized(false); }} className="text-gray-500 hover:text-white p-1 rounded-md transition-colors"><X size={16} /></button>
          </div>
       </div>
       
       <div className="flex-1 flex flex-col p-4 bg-[#1e1e1e]">
          <div className="border-b border-white/5 py-2 flex items-center">
             <span className="text-gray-500 text-sm w-16 shrink-0">收件人</span>
             <input 
               type="text" 
               value={composeData.to}
               onChange={e => setComposeData({...composeData, to: e.target.value})}
               className="flex-1 bg-transparent border-none outline-none text-sm text-gray-200" 
             />
          </div>
          <div className="border-b border-white/5 py-2 flex items-center">
             <span className="text-gray-500 text-sm w-16 shrink-0">主题</span>
             <input 
               type="text" 
               value={composeData.subject}
               onChange={e => setComposeData({...composeData, subject: e.target.value})}
               className="flex-1 bg-transparent border-none outline-none text-sm text-gray-200 font-medium" 
             />
          </div>
          
          <div className="flex-1 py-4 flex flex-col relative">
             {isRichText && (
               <div className="flex gap-1 pb-2 mb-2 border-b border-white/5 text-gray-400">
                 <button className="p-1 hover:bg-white/10 rounded"><Bold size={16}/></button>
                 <button className="p-1 hover:bg-white/10 rounded"><Italic size={16}/></button>
                 <button className="p-1 hover:bg-white/10 rounded"><Underline size={16}/></button>
               </div>
             )}
             {isRichText ? (
               <div 
                 contentEditable 
                 onInput={e => setComposeData({...composeData, body: e.currentTarget.textContent || ''})}
                 className="w-full flex-1 bg-transparent border-none outline-none text-sm text-gray-300 min-h-[100px] custom-scrollbar overflow-y-auto"
                 suppressContentEditableWarning
               >
                 {composeData.body}
               </div>
             ) : (
               <textarea 
                 value={composeData.body}
                 onChange={e => setComposeData({...composeData, body: e.target.value})}
                 className="w-full flex-1 bg-transparent border-none outline-none text-sm text-gray-300 resize-none min-h-[100px] custom-scrollbar"
                 placeholder="撰写邮件正文..."
               />
             )}
             {attachments.length > 0 && (
               <div className="mt-4 flex flex-wrap gap-2 p-3 bg-white/5 rounded-xl border border-white/5">
                 {attachments.map((file, i) => (
                   <div key={i} className="flex items-center gap-2 bg-[#2b2b2d] px-3 py-1.5 rounded-lg border border-white/10 text-xs text-gray-300">
                     <Paperclip size={12} className="text-gray-500" />
                     <span className="max-w-[120px] truncate">{file.name}</span>
                     <span className="text-gray-500">{(file.size / 1024).toFixed(1)}KB</span>
                     <button onClick={() => setAttachments(attachments.filter((_, idx) => idx !== i))} className="ml-1 text-gray-500 hover:text-red-400">
                       <X size={12} />
                     </button>
                   </div>
                 ))}
               </div>
             )}
          </div>
       </div>
       
       <div className="h-16 bg-[#2b2b2d] border-t border-white/10 flex items-center justify-between px-4 shrink-0">
          <div className="flex gap-2 text-gray-500 border-none outline-none relative">
             <input type="file" multiple className="hidden" ref={fileInputRef} onChange={handleAttachment} />
             <button className={`p-2 rounded-lg transition-colors ${isRichText ? 'text-indigo-400 bg-white/10' : 'hover:text-gray-300 hover:bg-white/5'}`} onClick={() => setIsRichText(!isRichText)}><Edit3 size={18} /></button>
             <button className="p-2 hover:text-gray-300 hover:bg-white/5 rounded-lg transition-colors" onClick={() => fileInputRef.current?.click()}><Paperclip size={18} /></button>
             <button className="p-2 hover:text-gray-300 hover:bg-white/5 rounded-lg transition-colors" onClick={() => {
               fileInputRef.current!.accept = "image/*";
               fileInputRef.current?.click();
             }}><ImageIcon size={18} /></button>
          </div>
          <div className="flex gap-3 items-center">
             <button onClick={() => setIsComposing(false)} className="text-gray-400 hover:text-white px-3 py-1.5 text-sm transition-colors"><Trash2 size={18} /></button>
             <button onClick={handleSend} className="bg-indigo-600 hover:bg-indigo-500 text-white px-6 py-2 rounded-lg text-sm font-medium transition-colors shadow-lg shadow-indigo-500/20 active:scale-95 flex items-center gap-2">
               发送 <Send size={14} />
             </button>
          </div>
       </div>
    </div>
  );
};
