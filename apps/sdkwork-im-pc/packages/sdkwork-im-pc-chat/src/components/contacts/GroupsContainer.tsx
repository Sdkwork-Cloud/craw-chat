import React, { useState, useEffect } from 'react';
import { Plus, Users } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { toast } from '../Toast';
import { groupService } from '../../services/GroupService';
import type { Chat } from '@sdkwork/im-pc-types';
import { CreateGroupModal } from '../CreateGroupModal';

export const GroupsContainer: React.FC<{
  searchQuery?: string;
  onOpenGroup?: (group: Chat) => void;
}> = ({ searchQuery = '', onOpenGroup }) => {
  const { t } = useTranslation();
  const [groups, setGroups] = useState<Chat[]>([]);
  const [loading, setLoading] = useState(true);
  const [isCreateGroupOpen, setIsCreateGroupOpen] = useState(false);

  useEffect(() => {
    groupService.getGroups()
      .then(data => {
        setGroups(data);
      })
      .catch(() => {
        setGroups([]);
        toast(t('contacts.groups.toast.loadFailed'), 'error');
      })
      .finally(() => setLoading(false));
  }, [t]);

  const filteredGroups = groups.filter(group => {
    if (!searchQuery.trim()) return true;
    return group.name.toLowerCase().includes(searchQuery.toLowerCase());
  });

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 h-full">
      <div className="px-8 py-6 border-b border-white/5 shrink-0 flex items-center justify-between">
        <div>
          <h2 className="text-xl font-medium text-gray-200">{t('contacts.groups.title')}</h2>
          <p className="text-sm text-gray-500 mt-1">{t('contacts.groups.description')}</p>
        </div>
        <button 
          onClick={() => {
            setIsCreateGroupOpen(true);
          }}
          className="flex items-center gap-2 px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white text-sm font-medium rounded-lg transition-colors shadow-lg shadow-indigo-500/20"
        >
          <Plus size={16} /> {t('contacts.groups.startGroup')}
        </button>
      </div>
      <div className="flex-1 overflow-y-auto custom-scrollbar p-8">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {loading ? (
            <div className="text-sm text-gray-500 col-span-full">{t('contacts.groups.loading')}</div>
          ) : filteredGroups.map(group => (
            <div
              key={group.id}
              className="p-5 rounded-2xl bg-white/5 hover:bg-white/10 transition-colors border border-white/5 cursor-pointer flex flex-col gap-4"
              onClick={() => onOpenGroup?.(group)}
            >
              <div className="flex items-center gap-4">
                <div className="w-12 h-12 rounded-xl bg-indigo-500/10 border border-indigo-500/20 flex items-center justify-center text-indigo-400 overflow-hidden">
                  {group.avatar ? (
                    <img src={group.avatar} alt={group.name} className="w-full h-full object-cover" />
                  ) : (
                    <Users size={24} />
                  )}
                </div>
                <div>
                  <h4 className="text-md font-medium text-gray-200">{group.name}</h4>
                  <p className="text-xs text-gray-500 mt-1">
                    {t('contacts.groups.memberStats', {
                      activeCount: group.activeCount || 0,
                      count: group.memberCount || 0,
                    })}
                  </p>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
      <CreateGroupModal
        isOpen={isCreateGroupOpen}
        onClose={() => setIsCreateGroupOpen(false)}
        onCreated={async (group) => {
          setGroups((previousGroups) => [group, ...previousGroups]);
          onOpenGroup?.(group);
        }}
      />
    </div>
  );
};
