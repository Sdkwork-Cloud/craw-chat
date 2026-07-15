import React from 'react';
import { motion } from 'motion/react';
import { useTranslation } from 'react-i18next';
import { Search, Plus, MoreHorizontal, UserMinus, X, Bot, BookOpen, LoaderCircle, Settings2 } from 'lucide-react';
import type { Chat, ChatAgentAssignment, User } from '@sdkwork/im-pc-types';
import { Avatar } from '@sdkwork/im-pc-commons';
import type { GroupMemberListItem, GroupMemberRole } from '../services/GroupService';

export interface ChatRightPanelProps {
  activeChat: Chat;
  currentUserChatId?: string;
  currentUserId?: string;
  groupMembers?: GroupMemberListItem[];
  groupMemberProfiles?: User[];
  currentUserGroupRole?: GroupMemberRole | null;
  canManageGroupMembers?: boolean;
  hasMoreGroupMembers?: boolean;
  isGroupMemberListLoading?: boolean;
  groupMemberListError?: boolean;
  onLoadMoreGroupMembers?: () => void;
  groupAgentAssignments?: ChatAgentAssignment[];
  canManageAgents?: boolean;
  onManageAgents?: () => void;
  canManageKnowledgebase?: boolean;
  isKnowledgebaseActionPending?: boolean;
  knowledgebaseActionLabel?: string;
  onManageKnowledgebase?: () => void;
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
  groupMembers: providedGroupMembers,
  groupMemberProfiles = [],
  currentUserGroupRole = null,
  canManageGroupMembers = false,
  hasMoreGroupMembers = false,
  isGroupMemberListLoading = false,
  groupMemberListError = false,
  onLoadMoreGroupMembers,
  groupAgentAssignments = activeChat.agentAssignments ?? [],
  canManageAgents = false,
  canManageKnowledgebase = false,
  isKnowledgebaseActionPending = false,
  knowledgebaseActionLabel,
  onClose,
  onSetModal,
  onToggleMute,
  onTogglePin,
  onDeleteChat,
  onRemoveGroupMember,
  onManageAgents,
  onManageKnowledgebase,
}) => {
  const { t } = useTranslation();
  const emptyNotice = t('chat.rightPanel.emptyNotice');
  const fallbackMemberName = t('chat.fallback.memberName');
  const fallbackMemberSubtitle = t('chat.fallback.memberSubtitle');
  const groupMembers = providedGroupMembers
    ?? (activeChat.members ?? []).map((id) => ({ id, memberId: id, role: 'unknown' as const }));
  const renderedGroupMembers = activeChat.members?.map((memberId) => groupMembers.find((member) => member.id === memberId))
    .filter((member): member is GroupMemberListItem => Boolean(member))
    ?? groupMembers;
  const groupMemberCount = Math.max(activeChat.memberCount ?? 0, groupMembers.length);
  const groupMemberCountIsLowerBound = hasMoreGroupMembers || activeChat.memberCountIsLowerBound === true;
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
            {activeChat.type === 'group' && canManageGroupMembers && (
              <div className="flex flex-col items-center gap-2 cursor-pointer group" onClick={() => onSetModal('addMember', '')}>
                 <div className="w-10 h-10 rounded-full bg-[#2b2b2d] flex items-center justify-center group-hover:bg-white/10 transition-colors">
                    <Plus size={18} className="text-gray-400 group-hover:text-gray-200" />
                 </div>
                 <span className="text-xs text-gray-400 group-hover:text-gray-200">{t('chat.rightPanel.actions.addMember')}</span>
              </div>
            )}
         </div>
         
         <div className="w-full space-y-4">
            <div
              className={`flex items-center justify-between py-3 border-b border-white/5 px-2 -mx-2 rounded transition-colors group ${activeChat.type !== 'group' || canManageGroupMembers ? 'cursor-pointer hover:bg-white/5' : ''}`}
              onClick={() => {
                if (activeChat.type !== 'group' || canManageGroupMembers) {
                  onSetModal('editName', activeChat.name);
                }
              }}
            >
               <span className="text-sm text-gray-300">{activeChat.type === 'group' ? t('chat.rightPanel.fields.groupName') : t('chat.rightPanel.fields.remark')}</span>
               <div className="flex items-center gap-2 text-gray-500">
                 <span className="text-sm overflow-hidden text-ellipsis whitespace-nowrap max-w-[100px]">{activeChat.name}</span>
                 <MoreHorizontal size={16} className="opacity-0 group-hover:opacity-100 transition-opacity" />
               </div>
            </div>
            {activeChat.type === 'group' && (
              <div
                className={`flex items-center justify-between py-3 border-b border-white/5 px-2 -mx-2 rounded transition-colors group ${canManageGroupMembers ? 'cursor-pointer hover:bg-white/5' : ''}`}
                onClick={() => {
                  if (canManageGroupMembers) {
                    onSetModal('editNotice', activeChat.notice || emptyNotice);
                  }
                }}
              >
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
                   <span className="text-xs text-gray-500">
                     {t(groupMemberCountIsLowerBound
                       ? 'chat.rightPanel.memberCountAtLeast'
                       : 'chat.rightPanel.memberCount', { count: groupMemberCount })}
                   </span>
                 </div>
                 {renderedGroupMembers.map((member) => {
                   const memberId = member.id;
                   const memberProfile = memberProfilesById.get(memberId);
                  const memberName = memberProfile?.name ?? fallbackMemberName;
                   const memberSubtitle = memberProfile?.email ?? memberProfile?.phone ?? fallbackMemberSubtitle;
                   const isCurrentUser = currentUserIdentifiers.has(memberId);
                   const canRemoveMember = canManageGroupMembers
                     && !isCurrentUser
                     && member.role !== 'owner'
                     && (currentUserGroupRole === 'owner'
                       || (currentUserGroupRole === 'admin'
                         && (member.role === 'member' || member.role === 'guest')));
                  return (
                    <div key={memberId} className="flex min-h-[36px] items-center gap-2 rounded px-2 py-1.5 hover:bg-white/5">
                      <Avatar src={memberProfile?.avatar} alt={memberName} className="h-7 w-7 shrink-0 rounded bg-[#2b2b2d]" />
                      <span className="min-w-0 flex-1" title={memberName}>
                        <span className="block truncate text-xs text-gray-300">{memberName}</span>
                        {memberSubtitle !== memberName && (
                          <span className="block truncate text-[11px] text-gray-500">{memberSubtitle}</span>
                        )}
                      </span>
                       {canRemoveMember && (
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
                 {renderedGroupMembers.length === 0 && (
                   <div className="rounded bg-white/5 px-2 py-3 text-center text-xs text-gray-500">
                     {isGroupMemberListLoading
                       ? t('chat.rightPanel.loadingMembers')
                       : groupMemberListError
                         ? t('chat.rightPanel.memberLoadFailed')
                         : t('chat.rightPanel.emptyMembers')}
                   </div>
                 )}
                 {(hasMoreGroupMembers || groupMemberListError) && onLoadMoreGroupMembers && (
                   <button
                     type="button"
                     className="mt-2 flex min-h-8 w-full items-center justify-center gap-2 rounded border border-white/10 px-3 py-1.5 text-xs text-gray-400 transition-colors hover:bg-white/5 hover:text-gray-200 disabled:cursor-not-allowed disabled:opacity-50"
                     disabled={isGroupMemberListLoading}
                     aria-busy={isGroupMemberListLoading}
                     onClick={onLoadMoreGroupMembers}
                   >
                     {isGroupMemberListLoading && <LoaderCircle size={14} className="animate-spin" />}
                     {t(groupMemberListError
                       ? 'chat.rightPanel.actions.retryMemberList'
                       : 'chat.rightPanel.actions.loadMoreMembers')}
                   </button>
                 )}
              </div>
            )}
            {activeChat.type === 'group' && knowledgebaseActionLabel && onManageKnowledgebase && (
              <button
                type="button"
                className="flex w-full items-center justify-between rounded border-b border-white/5 px-2 py-3 text-left transition-colors hover:bg-white/5 disabled:cursor-not-allowed disabled:opacity-50"
                disabled={isKnowledgebaseActionPending}
                onClick={onManageKnowledgebase}
                aria-busy={isKnowledgebaseActionPending}
                aria-label={knowledgebaseActionLabel}
                title={knowledgebaseActionLabel}
              >
                <span className="flex min-w-0 items-center gap-2 text-sm text-gray-300">
                  {canManageKnowledgebase
                    ? <Settings2 size={16} className="shrink-0 text-indigo-400" />
                    : <BookOpen size={16} className="shrink-0 text-indigo-400" />}
                  <span className="truncate">{t('chat.rightPanel.fields.knowledgebase')}</span>
                </span>
                <span className="flex min-w-0 items-center gap-2 text-sm text-gray-500">
                  <span className="truncate text-right">{knowledgebaseActionLabel}</span>
                  {isKnowledgebaseActionPending && <LoaderCircle size={14} className="shrink-0 animate-spin" />}
                </span>
              </button>
            )}
            {activeChat.type === 'group' && (
              <div className="border-b border-white/5 py-3">
                <div className="mb-2 flex items-center justify-between">
                  <span className="text-sm text-gray-300">{t('chat.rightPanel.fields.agents')}</span>
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-gray-500">{t('chat.rightPanel.agentCount', { count: groupAgentAssignments.length })}</span>
                    {canManageAgents && <button
                      type="button"
                      className="flex h-7 w-7 items-center justify-center rounded text-gray-500 transition-colors hover:bg-white/10 hover:text-gray-200"
                      onClick={onManageAgents}
                      aria-label={t('chat.rightPanel.actions.manageAgents')}
                      title={t('chat.rightPanel.actions.manageAgents')}
                    >
                      <Settings2 size={14} />
                    </button>}
                  </div>
                </div>
                <div className="space-y-1">
                  {groupAgentAssignments.map((assignment) => (
                    <div key={`${assignment.agentId}:${assignment.revisionId ?? ''}`} className="flex min-h-[36px] items-center gap-2 rounded px-2 py-1.5 hover:bg-white/5">
                      <Avatar src={assignment.avatar} alt={assignment.name ?? assignment.agentId} className="h-7 w-7 shrink-0 rounded bg-indigo-500/20" />
                      <span className="min-w-0 flex-1 truncate text-xs text-gray-300" title={assignment.agentId}>
                        {assignment.name || assignment.agentId}
                      </span>
                      <Bot size={13} className="shrink-0 text-indigo-400" aria-label={t('chat.rightPanel.agentLabel')} />
                    </div>
                  ))}
                  {groupAgentAssignments.length === 0 && canManageAgents && (
                    <button type="button" onClick={onManageAgents} className="w-full rounded bg-white/5 px-2 py-3 text-center text-xs text-gray-500 transition-colors hover:bg-white/10 hover:text-gray-300">
                      {t('chat.rightPanel.emptyAgents')}
                    </button>
                  )}
                </div>
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
