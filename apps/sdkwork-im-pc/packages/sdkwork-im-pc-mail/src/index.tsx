import React, { useState, useEffect, useRef } from 'react';
import { Mail, Search, Edit3, Inbox, Send, Star, Trash2, Clock, MoreVertical, Archive, Share, CornerUpLeft, CornerUpRight, Paperclip, X, Maximize2, Image, Bold, Italic, Underline } from 'lucide-react';
import { cn, sanitizeHtmlForDisplay } from '@sdkwork/im-pc-commons';
import { mailService, MailItem } from './services/MailService';
import { toast } from '@sdkwork/im-pc-chat';
import { ComposeMailModal } from './components/ComposeMailModal';

export const MailView: React.FC = () => {
  const [mails, setMails] = useState<MailItem[]>([]);
  const [selectedMailId, setSelectedMailId] = useState<string | null>(null);
  const [currentFolder, setCurrentFolder] = useState<string>('inbox');
  const [isComposing, setIsComposing] = useState(false);
  const [isRichText, setIsRichText] = useState(false);
  const [composeData, setComposeData] = useState({ to: '', subject: '', body: '' });
  const [attachments, setAttachments] = useState<File[]>([]);
  const [quickReplyText, setQuickReplyText] = useState('');
  const [quickReplyAttachments, setQuickReplyAttachments] = useState<File[]>([]);
  const [isComposeMaximized, setIsComposeMaximized] = useState(false);
  const [showMailMoreMenu, setShowMailMoreMenu] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const quickReplyFileInputRef = useRef<HTMLInputElement>(null);

  const handleAttachment = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setAttachments([...attachments, ...Array.from(e.target.files)]);
    }
    e.target.value = '';
  };

  const handleQuickReplyAttachment = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setQuickReplyAttachments([...quickReplyAttachments, ...Array.from(e.target.files)]);
    }
    e.target.value = '';
  };

  const loadMails = () => {
    mailService.getMails(currentFolder).then(fetchedMails => {
      setMails(fetchedMails);
      if (fetchedMails.length > 0 && (!selectedMailId || !fetchedMails.find(m => m.id === selectedMailId))) {
        setSelectedMailId(fetchedMails[0].id);
      } else if (fetchedMails.length === 0) {
        setSelectedMailId(null);
      }
    });
  };

  useEffect(() => {
    loadMails();
  }, [currentFolder]); // We ignore selectedMailId to prevent re-fetching on select

  const selectedMail = mails.find(m => m.id === selectedMailId);

  useEffect(() => {
    setQuickReplyText('');
  }, [selectedMailId]);

  const handleQuickReply = async () => {
    if (!selectedMail || (!quickReplyText.trim() && quickReplyAttachments.length === 0)) return;
    try {
      await mailService.sendMail({
        subject: `Re: ${selectedMail.subject}`,
        bodyHtml: `<p>${quickReplyText}</p><br/><hr/><p>On ${selectedMail.time}, ${selectedMail.senderName} wrote:</p><div>${selectedMail.bodyHtml || selectedMail.previewText}</div>`,
      });
      toast('回复发送成功', 'success');
      setQuickReplyText('');
      setQuickReplyAttachments([]);
      if (currentFolder === 'sent') loadMails();
    } catch (e) {
      toast('发送失败', 'error');
    }
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

  const handleDelete = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    await mailService.deleteMail(id);
    toast('邮件已移至垃圾箱', 'success');
    loadMails();
  };

  return (
    <div className="flex-1 flex bg-[#1e1e1e] min-w-0 min-h-0 relative">
      {/* Mail Sidebar */}
      <div className="w-[240px] flex-shrink-0 bg-[#232325] border-r border-white/5 flex flex-col h-full">
        <div className="p-4 shrink-0">
          <button 
            onClick={() => setIsComposing(true)}
            className="w-full bg-indigo-600 hover:bg-indigo-500 text-white font-medium py-2.5 rounded-lg flex items-center justify-center gap-2 transition-all shadow-lg shadow-indigo-500/20 active:scale-95"
          >
            <Edit3 size={18} /> 写邮件
          </button>
        </div>
        
        <div className="flex-1 overflow-y-auto px-3 py-2 flex flex-col gap-1.5 mt-2">
          <h3 className="text-xs font-semibold text-gray-500 uppercase tracking-wider px-3 mb-2">邮件夹</h3>
          <div onClick={() => setCurrentFolder('inbox')} className={cn("px-3 py-2.5 rounded-lg flex items-center justify-between cursor-pointer font-medium text-sm transition-colors", currentFolder === 'inbox' ? "bg-indigo-500/15 text-indigo-400" : "text-gray-400 hover:bg-white/5 hover:text-gray-200")}>
             <div className="flex items-center gap-3">
               <Inbox size={18} /> 收件箱
             </div>
             {currentFolder === 'inbox' && <span className="bg-indigo-600 text-white text-[10px] px-2 py-0.5 rounded-full">{mails.length}</span>}
          </div>
          <div onClick={() => setCurrentFolder('starred')} className={cn("px-3 py-2.5 rounded-lg flex items-center gap-3 cursor-pointer text-sm transition-colors font-medium", currentFolder === 'starred' ? "bg-indigo-500/15 text-indigo-400" : "text-gray-400 hover:bg-white/5 hover:text-gray-200")}>
            <Star size={18} /> 星标邮件
          </div>
          <div onClick={() => setCurrentFolder('drafts')} className={cn("px-3 py-2.5 rounded-lg flex items-center gap-3 cursor-pointer text-sm transition-colors font-medium", currentFolder === 'drafts' ? "bg-indigo-500/15 text-indigo-400" : "text-gray-400 hover:bg-white/5 hover:text-gray-200")}>
            <Clock size={18} /> 稍后处理
          </div>
          <div onClick={() => setCurrentFolder('sent')} className={cn("px-3 py-2.5 rounded-lg flex items-center gap-3 cursor-pointer text-sm transition-colors font-medium", currentFolder === 'sent' ? "bg-indigo-500/15 text-indigo-400" : "text-gray-400 hover:bg-white/5 hover:text-gray-200")}>
            <Send size={18} /> 已发送
          </div>
          <div onClick={() => setCurrentFolder('trash')} className={cn("px-3 py-2.5 rounded-lg flex items-center gap-3 cursor-pointer text-sm transition-colors font-medium", currentFolder === 'trash' ? "bg-indigo-500/15 text-indigo-400" : "text-gray-400 hover:bg-white/5 hover:text-gray-200")}>
            <Trash2 size={18} /> 已删除
          </div>
        </div>
      </div>
      
      {/* Mail List */}
      <div className="w-[340px] flex-shrink-0 bg-[#181818] border-r border-white/5 flex flex-col h-full z-10">
        <div className="p-4 border-b border-white/5 shrink-0 bg-[#181818]/80 backdrop-blur-md">
          <div className="relative group">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 group-focus-within:text-indigo-400 transition-colors" size={16} />
            <input 
              type="text" 
              placeholder="搜索邮件..." 
              className="w-full bg-[#2b2b2d] border border-transparent focus:border-indigo-500/50 rounded-lg pl-9 pr-4 py-2 text-sm text-gray-200 outline-none transition-all placeholder:text-gray-500 focus:bg-[#1e1e1e] focus:shadow-[0_0_0_2px_rgba(99,102,241,0.2)]"
            />
          </div>
        </div>
        
        <div className="flex-1 overflow-y-auto w-full custom-scrollbar">
          {mails.map((mail) => (
            <div 
              key={mail.id} 
              onClick={() => {
                setSelectedMailId(mail.id);
                if (!mail.isRead) {
                   mailService.markAsRead(mail.id).then(() => {
                      const newMails = [...mails];
                      const idx = newMails.findIndex(m => m.id === mail.id);
                      if (idx > -1) newMails[idx].isRead = true;
                      setMails(newMails);
                   });
                }
              }}
              className={cn("p-4 border-b border-white/5 cursor-pointer transition-all relative group", 
                selectedMailId === mail.id ? "bg-indigo-500/10 border-l-2 border-l-indigo-500" : "hover:bg-white/5 border-l-2 border-l-transparent",
                !mail.isRead ? "after:content-[''] after:absolute after:left-2 after:top-1/2 after:-translate-y-1/2 after:w-2 after:h-2 after:bg-indigo-500 after:rounded-full pl-6" : ""
              )}>
               <div className="flex justify-between items-start mb-1.5">
                 <span className={cn("text-sm transition-colors truncate pr-2", !mail.isRead ? "font-bold text-gray-100" : "font-semibold text-gray-300")}>{mail.senderName}</span>
                 <span className="text-xs text-gray-500 shrink-0 mt-0.5">{mail.time}</span>
               </div>
               <div className={cn("text-sm mb-1.5 truncate transition-colors", !mail.isRead ? "text-indigo-300 font-bold" : (selectedMailId === mail.id ? "text-indigo-400 font-medium" : "text-gray-400"))}>{mail.subject}</div>
               <div className="text-xs text-gray-500 line-clamp-2 leading-relaxed">{mail.previewText}</div>

               {/* Hover Actions */}
               <div className="absolute right-4 top-1/2 -translate-y-1/2 flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity bg-[#2b2b2d] p-1 rounded-lg border border-white/10 shadow-lg">
                  <button onClick={(e) => handleDelete(e, mail.id)} className="p-1.5 text-gray-400 hover:text-red-400 hover:bg-white/10 rounded-md transition-colors" title="删除">
                    <Trash2 size={14} />
                  </button>
                  <button className="p-1.5 text-gray-400 hover:text-indigo-400 hover:bg-white/10 rounded-md transition-colors" title="延后处理" onClick={(e) => { 
                    e.stopPropagation(); 
                    setMails(prev => prev.filter(m => m.id !== mail.id));
                    if (selectedMailId === mail.id) setSelectedMailId(null);
                    toast('邮件已延后处理', 'success'); 
                  }}>
                    <Clock size={14} />
                  </button>
                  <button className="p-1.5 text-gray-400 hover:text-indigo-400 hover:bg-white/10 rounded-md transition-colors" title="归档" onClick={(e) => { 
                    e.stopPropagation(); 
                    setMails(prev => prev.filter(m => m.id !== mail.id));
                    if (selectedMailId === mail.id) setSelectedMailId(null);
                    toast('邮件已归档', 'success'); 
                  }}>
                    <Archive size={14} />
                  </button>
               </div>
            </div>
          ))}
          {mails.length === 0 && (
             <div className="flex flex-col items-center justify-center h-40 text-sm text-gray-500">
                <Inbox size={32} className="mb-3 opacity-20" />
                当前文件夹为空
             </div>
          )}
        </div>
      </div>
      
      {/* Mail Content */}
      <div className="flex-1 flex flex-col h-full bg-[#1e1e1e] min-w-0">
         {selectedMail ? (
           <>
             {/* Mail Header Area */}
             <div className="p-6 md:p-8 border-b border-white/5 shrink-0 bg-[#1e1e1e]">
                <div className="flex items-center justify-between gap-4 mb-6">
                   <div className="flex gap-2">
                     <button className="p-2 border border-white/10 text-gray-300 hover:bg-white/5 rounded-lg transition-colors flex items-center justify-center bg-[#2b2b2d]" onClick={() => document.getElementById('quick-reply-box')?.focus()}>
                       <CornerUpLeft size={16} />
                     </button>
                     <button className="p-2 border border-white/10 text-gray-300 hover:bg-white/5 rounded-lg transition-colors flex items-center justify-center bg-[#2b2b2d]" onClick={() => {
                        setIsComposing(true);
                        toast('回复邮件（带附件）', 'success');
                     }}>
                       <CornerUpRight size={16} />
                     </button>
                     <button onClick={(e) => handleDelete(e as any, selectedMail.id)} className="p-2 border border-white/10 text-gray-300 hover:bg-red-500/10 hover:text-red-400 hover:border-red-500/20 rounded-lg transition-colors flex items-center justify-center bg-[#2b2b2d] ml-2">
                       <Trash2 size={16} />
                     </button>
                   </div>
                   <div className="flex items-center gap-2">
                      <button className="text-gray-500 hover:text-gray-300 transition-colors p-2" onClick={() => {
                         setMails(prev => prev.filter(m => m.id !== selectedMail.id));
                         setSelectedMailId(null);
                         toast('邮件已归档', 'success');
                      }}><Archive size={18} /></button>
                      <div className="relative">
                        <button className="text-gray-500 hover:text-gray-300 transition-colors p-2" onClick={() => setShowMailMoreMenu(!showMailMoreMenu)}><MoreVertical size={18} /></button>
                        {showMailMoreMenu && (
                           <>
                             <div className="fixed inset-0 z-40" onClick={() => setShowMailMoreMenu(false)} />
                             <div className="absolute top-10 right-0 w-48 bg-[#282828] border border-white/10 rounded-xl shadow-xl z-50 p-1.5 animate-in fade-in zoom-in-95">
                               <button className="w-full text-left px-3 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/10 rounded-lg transition-colors" onClick={() => { 
                                 setShowMailMoreMenu(false); 
                                 mailService.markAsUnread(selectedMail.id).then(() => {
                                    const newMails = [...mails];
                                    const idx = newMails.findIndex(m => m.id === selectedMail.id);
                                    if (idx > -1) newMails[idx].isRead = false;
                                    setMails(newMails);
                                    toast('已标记为未读', 'success');
                                 });
                               }}>标记为未读</button>
                               <button className="w-full text-left px-3 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/10 rounded-lg transition-colors" onClick={() => { setShowMailMoreMenu(false); toast('已添加到任务', 'success'); }}>添加到任务</button>
                               <div className="h-px bg-white/10 my-1 mx-2" />
                               <button className="w-full text-left px-3 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/10 rounded-lg transition-colors" onClick={() => { 
                                 setShowMailMoreMenu(false); 
                                 setMails(prev => prev.filter(m => m.senderEmail !== selectedMail.senderEmail));
                                 setSelectedMailId(null);
                                 toast('已静音此邮件发件人', 'success'); 
                               }}>不再接收此发件人邮件</button>
                               <button className="w-full text-left px-3 py-2 text-sm text-red-500 hover:bg-red-500/10 rounded-lg transition-colors" onClick={() => { 
                                 setShowMailMoreMenu(false); 
                                 setMails(prev => prev.filter(m => m.id !== selectedMail.id));
                                 setSelectedMailId(null);
                                 toast('已举报垃圾邮件', 'success'); 
                               }}>举报为垃圾邮件</button>
                             </div>
                           </>
                        )}
                      </div>
                   </div>
                </div>

                <div className="flex items-start justify-between mb-6">
                  <h1 className="text-2xl font-semibold text-gray-100 leading-snug pr-8">{selectedMail.subject}</h1>
                  <button 
                    onClick={() => {
                       mailService.toggleStar(selectedMail.id).then(() => {
                          const newMails = [...mails];
                          const idx = newMails.findIndex(m => m.id === selectedMail.id);
                          if (idx > -1) newMails[idx].isStarred = !newMails[idx].isStarred;
                          setMails(newMails);
                       });
                    }}
                    className={cn("p-2 rounded-lg transition-colors shrink-0", selectedMail.isStarred ? "text-yellow-400 hover:bg-yellow-400/10" : "text-gray-500 hover:text-gray-300 hover:bg-white/10")}
                  >
                    <Star size={24} fill={selectedMail.isStarred ? "currentColor" : "none"} />
                  </button>
                </div>

                <div className="flex items-center justify-between text-sm">
                   <div className="flex items-center gap-4">
                      <div className="w-12 h-12 rounded-full bg-gradient-to-br from-indigo-500/20 to-purple-500/20 border border-indigo-500/20 text-indigo-400 text-xl flex items-center justify-center font-bold">
                         {selectedMail.senderName.charAt(0)}
                      </div>
                      <div>
                        <div className="text-gray-200 font-semibold mb-0.5">{selectedMail.senderName} <span className="text-gray-500 font-normal ml-1">&lt;{selectedMail.senderEmail}&gt;</span></div>
                        <div className="text-gray-500 text-xs flex items-center gap-2">
                           发送至 <span className="bg-white/5 px-2 py-0.5 rounded border border-white/10">me@sdkwork.com</span>
                        </div>
                      </div>
                   </div>
                   <div className="text-gray-400 font-mono text-xs">{selectedMail.time}</div>
                </div>
             </div>

             {/* Mail Body */}
             <div className="flex-1 overflow-y-auto p-6 md:p-8 text-[15px] leading-relaxed text-gray-300 custom-scrollbar flex flex-col items-center">
                <div className="max-w-[800px] w-full">
                  <div
                    className="mail-content prose prose-invert prose-indigo w-full max-w-none"
                    dangerouslySetInnerHTML={{
                      __html: sanitizeHtmlForDisplay(selectedMail.bodyHtml || selectedMail.previewText),
                    }}
                  />
                  
                  {selectedMail.attachments && selectedMail.attachments.length > 0 && (
                     <div className="mt-12 border-t border-white/5 pt-6">
                        <div className="text-sm font-medium text-gray-400 mb-4 flex items-center gap-2">
                           <Paperclip size={16} />
                           附件 ({selectedMail.attachments.length})
                        </div>
                        <div className="flex flex-wrap gap-4">
                        {selectedMail.attachments.map((att, idx) => (
                           <div key={idx} className="border border-white/10 rounded-xl p-4 bg-[#2b2b2d] flex items-center gap-4 w-64 cursor-pointer hover:bg-[#343438] hover:border-white/20 transition-all group">
                              <div className={cn("w-12 h-12 flex items-center justify-center rounded-lg uppercase font-bold text-sm shrink-0",
                                 att.type === 'pdf' ? "bg-red-500/10 text-red-400" :
                                 att.type === 'word' ? "bg-blue-500/10 text-blue-400" :
                                 "bg-gray-500/10 text-gray-400"
                              )}>
                                {att.type}
                              </div>
                              <div className="min-w-0 flex-1">
                                <div className="text-gray-200 font-medium truncate mb-1">{att.name}</div>
                                <div className="text-gray-500 text-xs font-mono">{att.size}</div>
                              </div>
                           </div>
                        ))}
                        </div>
                     </div>
                  )}

                  {/* Quick Reply Box */}
                  <div className="mt-12 border border-white/10 rounded-xl bg-[#2b2b2d] p-4 focus-within:border-indigo-500/50 focus-within:shadow-[0_0_0_2px_rgba(99,102,241,0.2)] transition-all relative overflow-hidden group">
                     <div className="absolute top-0 left-0 w-1 h-full bg-indigo-500 transform -translate-x-full group-focus-within:translate-x-0 transition-transform"></div>
                     <textarea 
                       id="quick-reply-box"
                       value={quickReplyText}
                       onChange={e => setQuickReplyText(e.target.value)}
                       placeholder={`快速回复给 ${selectedMail.senderName}...`} 
                       className="w-full bg-transparent outline-none resize-none text-sm text-gray-200 min-h-[80px]"
                     ></textarea>
                     {quickReplyAttachments.length > 0 && (
                       <div className="mt-2 flex flex-wrap gap-2 p-3 bg-white/5 rounded-xl border border-white/5">
                         {quickReplyAttachments.map((file, i) => (
                           <div key={i} className="flex items-center gap-2 bg-[#2b2b2d] px-3 py-1.5 rounded-lg border border-white/10 text-xs text-gray-300">
                             <Paperclip size={12} className="text-gray-500" />
                             <span className="max-w-[120px] truncate">{file.name}</span>
                             <span className="text-gray-500">{(file.size / 1024).toFixed(1)}KB</span>
                             <button onClick={() => setQuickReplyAttachments(quickReplyAttachments.filter((_, idx) => idx !== i))} className="ml-1 text-gray-500 hover:text-red-400">
                               <X size={12} />
                             </button>
                           </div>
                         ))}
                       </div>
                     )}
                     <div className="flex justify-between items-center mt-2">
                        <div className="flex gap-2 text-gray-500 relative">
                           <input type="file" multiple className="hidden" ref={quickReplyFileInputRef} onChange={handleQuickReplyAttachment} />
                           <button className="p-1.5 hover:text-gray-300 hover:bg-white/5 hover:text-indigo-400 rounded-md transition-colors tooltip" title="添加附件" onClick={() => quickReplyFileInputRef.current?.click()}><Paperclip size={18} /></button>
                           <button className="p-1.5 hover:text-gray-300 hover:bg-white/5 hover:text-indigo-400 rounded-md transition-colors tooltip" title="插入图片" onClick={() => {
                              quickReplyFileInputRef.current!.accept = "image/*";
                              quickReplyFileInputRef.current?.click();
                           }}><Image size={18} /></button>
                        </div>
                        <button 
                          onClick={handleQuickReply}
                          disabled={!quickReplyText.trim() && quickReplyAttachments.length === 0}
                          className="bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 disabled:hover:bg-indigo-600 text-white px-5 py-2 rounded-lg text-sm font-medium transition-colors shadow-lg shadow-indigo-500/20 active:scale-95 flex items-center gap-2"
                        >
                           发送 <Send size={14} />
                        </button>
                     </div>
                  </div>
                </div>
             </div>
           </>
         ) : (
           <div className="flex-1 flex justify-center mt-32">
             <div className="flex flex-col items-center justify-center text-gray-500 max-w-sm text-center">
                <div className="w-24 h-24 bg-white/5 rounded-full flex items-center justify-center mb-6">
                  <Mail size={40} className="opacity-40" />
                </div>
                <h3 className="text-lg font-medium text-gray-300 mb-2">未选择邮件</h3>
                <p className="text-sm">从左侧列表中选择一封邮件进行阅读，或者点击写邮件开始新的通信。</p>
             </div>
           </div>
         )}
      </div>

      {/* Compose Modal */}
      <ComposeMailModal
        isComposing={isComposing}
        setIsComposing={setIsComposing}
        isComposeMaximized={isComposeMaximized}
        setIsComposeMaximized={setIsComposeMaximized}
        isRichText={isRichText}
        setIsRichText={setIsRichText}
        composeData={composeData}
        setComposeData={setComposeData}
        attachments={attachments}
        setAttachments={setAttachments}
        currentFolder={currentFolder}
        loadMails={loadMails}
      />
    </div>
  );
};
