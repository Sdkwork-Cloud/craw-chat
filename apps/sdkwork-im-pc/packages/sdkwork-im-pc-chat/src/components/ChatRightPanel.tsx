import React from 'react';
import { motion } from 'motion/react';
import { useTranslation } from 'react-i18next';
import { Search, Plus, MoreHorizontal, UserMinus, X } from 'lucide-react';
import type { Chat, User } from '@sdkwork/im-pc-types';
import { Avatar } from '@sdkwork/im-pc-commons';

export interface ChatRightPanelProps {
  activeChat: Chat;
  currentUserChatId?: string;
  currentUserId?: string;
  groupMemberProfiles?: User[];
  onClose: () => void;
  onSetModal: (modal: 'search'|'editName'|'editNotice'|'addMember'|null, inputVal: string) => void;
  onToggleMute: () => Promise<void>;
  onTogglePin: () => Promise<void>;
  onDeleteChat: () => Promise<void>;
  onRemoveGroupMember: (memberId: string) => Promise<void>;
}

export const ChatRightPanel: React.FC<ChatRightPanelProps> = ({
  activeChat,
  currentUserChatId,
  currentUserId,
  groupMemberProfiles = [],
  onClose,
  onSetModal,
  onToggleMute,
  onTogglePin,
  onDeleteChat,
  onRemoveGroupMember
}) => {
  const { t } = useTranslation();
  const emptyNotice = t('chat.rightPanel.emptyNotice');
  const fallbackMemberName = t('chat.fallback.memberName');
  const fallbackMemberSubtitle = t('chat.fallback.memberSubtitle');
  const groupMembers = activeChat.members ?? [];
  const groupMemberCount = activeChat.memberCount ?? groupMembers.length;
  const currentUserIdentifiers = new Set(
    [currentUserId, currentUserChatId].filter((value): value is string => Boolean(value)),
  );
  const memberProfilesById = new Map<string, User>();
  for (const profile of groupMemberProfiles) {
    memberProfilesById.set(profile.id, profile);
    if (profile.chatId) {
      memberProfilesById.set(profile.chatId, profile);
    }
  }

  return (
    <motion.div
      initial={{ width: 0, opacity: 0 }}
      animate={{ width: 300, opacity: 1 }}
      exit={{ width: 0, opacity: 0 }}
      transition={{ duration: 0.2 }}
      className="h-full border-l border-white/5 bg-[#181818] overflow-y-auto custom-scrollbar flex-shrink-0"
    >
      <div className="sticky top-0 z-10 flex h-14 items-center justify-between border-b border-white/5 bg-[#181818]/95 px-5 backdrop-blur">
        <h2 className="truncate text-sm font-medium text-gray-200">
          {t('chat.rightPanel.title')}
        </h2>
        <button
          type="button"
          aria-label={t('chat.rightPanel.actions.close')}
          title={t('chat.rightPanel.actions.close')}
          className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg text-gray-400 transition-colors hover:bg-white/10 hover:text-gray-100"
          onClick={onClose}
        >
          <X size={18} />
        </button>
      </div>
      <div className="flex flex-col items-center px-6 pb-6 pt-7">
         <Avatar src={activeChat.avatar} alt={activeChat.name} className="w-20 h-20 rounded-2xl bg-[#2b2b2d] mb-4 shadow-lg" />
         <h3 className="mb-6 max-w-full truncate text-lg font-medium text-gray-200">{activeChat.name}</h3>
         
         <div className="w-full flex justify-center gap-6 mb-8">
            <div className="flex flex-col items-center gap-2 cursor-pointer group" onClick={() => onSetModal('search', '')}>
               <div className="w-10 h-10 rounded-full bg-[#2b2b2d] flex items-center justify-center group-hover:bg-white/10 transition-colors">
                  <Search size={18} className="text-gray-400 group-hover:text-gray-200" />
               </div>
               <span className="text-xs text-gray-400 group-hover:text-gray-200">{t('chat.rightPanel.actions.searchChat')}</span>
            </div>
            {activeChat.type === 'group' && (
              <div className="flex flex-col items-center gap-2 cursor-pointer group" onClick={() => onSetModal('addMember', '')}>
                 <div className="w-10 h-10 rounded-full bg-[#2b2b2d] flex items-center justify-center group-hover:bg-white/10 transition-colors">
                    <Plus size={18} className="text-gray-400 group-hover:text-gray-200" />
                 </div>
                 <span className="text-xs text-gray-400 group-hover:text-gray-200">{t('chat.rightPanel.actions.addMember')}</span>
              </div>
            )}
         </div>
         
         <div className="w-full space-y-4">
            <div className="flex items-center justify-between py-3 border-b border-white/5 cursor-pointer hover:bg-white/5 px-2 -mx-2 rounded transition-colors group" onClick={() => onSetModal('editName', activeChat.name)}>
               <span className="text-sm text-gray-300">{activeChat.type === 'group' ? t('chat.rightPanel.fields.groupName') : t('chat.rightPanel.fields.remark')}</span>
               <div className="flex items-center gap-2 text-gray-500">
                 <span className="text-sm overflow-hidden text-ellipsis whitespace-nowrap max-w-[100px]">{activeChat.name}</span>
                 <MoreHorizontal size={16} className="opacity-0 group-hover:opacity-100 transition-opacity" />
               </div>
            </div>
            {activeChat.type === 'group' && (
              <div className="flex items-center justify-between py-3 border-b border-white/5 cursor-pointer hover:bg-white/5 px-2 -mx-2 rounded transition-colors group" onClick={() => onSetModal('editNotice', activeChat.notice || emptyNotice)}>
                 <span className="text-sm text-gray-300">{t('chat.rightPanel.fields.groupNotice')}</span>
                 <div className="flex items-center gap-2 text-gray-500">
                   <span className="text-sm overflow-hidden text-ellipsis whitespace-nowrap max-w-[100px]">{activeChat.notice || emptyNotice}</span>
                   <MoreHorizontal size={16} className="opacity-0 group-hover:opacity-100 transition-opacity" />
                 </div>
              </div>
            )}
            {activeChat.type === 'group' && (
              <div className="border-b border-white/5 py-3">
                <div className="mb-2 flex items-center justify-between">
                  <span className="text-sm text-gray-300">{t('chat.rightPanel.fields.members')}</span>
                  <span className="text-xs text-gray-500">{t('chat.rightPanel.memberCount', { count: groupMemberCount })}</span>
                </div>
                {activeChat.members?.map((memberId) => {
                  const memberProfile = memberProfilesById.get(memberId);
                  const memberName = memberProfile?.name ?? fallbackMemberName;
                  const memberSubtitle = memberProfile?.email ?? memberProfile?.phone ?? fallbackMemberSubtitle;
                  const isCurrentUser = currentUserIdentifiers.has(memberId);
                  return (
                    <div key={memberId} className="flex min-h-[36px] items-center gap-2 rounded px-2 py-1.5 hover:bg-white/5">
                      <Avatar src={memberProfile?.avatar} alt={memberName} className="h-7 w-7 shrink-0 rounded bg-[#2b2b2d]" />
                      <span className="min-w-0 flex-1" title={memberName}>
                        <span className="block truncate text-xs text-gray-300">{memberName}</span>
                        {memberSubtitle !== memberName && (
                          <span className="block truncate text-[11px] text-gray-500">{memberSubtitle}</span>
                        )}
                      </span>
                      {!isCurrentUser && (
                        <button
                          type="button"
                          aria-label={t('chat.rightPanel.actions.removeMember')}
                          title={t('chat.rightPanel.actions.removeMember')}
                          className="flex h-7 w-7 shrink-0 items-center justify-center rounded text-gray-500 transition-colors hover:bg-red-500/10 hover:text-red-400"
                          onClick={() => void onRemoveGroupMember(memberId)}
                        >
                          <UserMinus size={14} />
                        </button>
                      )}
                    </div>
                  );
                })}
                {groupMembers.length === 0 && (
                  <div className="rounded bg-white/5 px-2 py-3 text-center text-xs text-gray-500">
                    {t('chat.rightPanel.emptyMembers')}
                  </div>
                )}
              </div>
            )}
            <div className="flex items-center justify-between py-3 border-b border-white/5">
               <span className="text-sm text-gray-300">{t('chat.rightPanel.fields.mute')}</span>
               <div 
                 className={`w-10 h-5 rounded-full relative cursor-pointer pt-0.5 px-0.5 transition-colors ${activeChat.isMuted ? 'bg-[#00b42a]' : 'bg-[#2b2b2d] hover:bg-white/10'}`}
                 onClick={() => void onToggleMute()}
               >
                  <div className={`w-4 h-4 bg-white rounded-full absolute top-0.5 transition-all ${activeChat.isMuted ? 'right-0.5' : 'left-0.5'}`} />
               </div>
            </div>
            <div className="flex items-center justify-between py-3 border-b border-white/5">
               <span className="text-sm text-gray-300">{t('chat.rightPanel.fields.pin')}</span>
               <div 
                 className={`w-10 h-5 rounded-full relative cursor-pointer pt-0.5 px-0.5 transition-colors ${activeChat.isPinned ? 'bg-[#00b42a]' : 'bg-[#2b2b2d] hover:bg-white/10'}`}
                 onClick={() => void onTogglePin()}
               >
                  <div className={`w-4 h-4 bg-white rounded-full absolute top-0.5 transition-all ${activeChat.isPinned ? 'right-0.5' : 'left-0.5'}`} />
               </div>
            </div>
         </div>
         
         <button 
           className="w-full py-3 mt-8 text-red-500 text-sm font-medium hover:bg-red-500/10 rounded-lg transition-colors border border-transparent hover:border-red-500/20"
           onClick={() => void onDeleteChat()}
         >
            {activeChat.type === 'group' ? t('chat.rightPanel.actions.leaveGroup') : t('chat.rightPanel.actions.deleteChat')}
         </button>
      </div>
    </motion.div>
  );
};
