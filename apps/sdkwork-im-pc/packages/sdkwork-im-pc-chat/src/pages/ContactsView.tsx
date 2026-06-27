import React, { useState, useEffect } from 'react';
import { User, Users, Building2, Search, Tag, UserPlus, Star } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { useTranslation } from 'react-i18next';
import { Avatar } from '@sdkwork/im-pc-commons';
import { cn } from '@sdkwork/im-pc-commons';
import { toast } from '../components/Toast';
import { contactService, SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT } from '../services/ContactService';
import type { Chat, User as UserType } from '@sdkwork/im-pc-types';

import { TagsContainer } from '../components/contacts/TagsContainer';
import { NewFriendsContainer } from '../components/contacts/NewFriendsContainer';
import { GroupsContainer } from '../components/contacts/GroupsContainer';
import { ContactDetailPane } from '../components/contacts/ContactDetailPane';
import { OrgContainer } from '../components/contacts/OrgContainer';
import { AllContactsContainer } from '../components/contacts/AllContactsContainer';

const menuItems = [
  { id: 'all', labelKey: 'contacts.menu.all', icon: <User size={18} />, color: 'bg-indigo-500' },
  { id: 'new', labelKey: 'contacts.menu.new', icon: <UserPlus size={18} />, color: 'bg-orange-500' },
  { id: 'groups', labelKey: 'contacts.menu.groups', icon: <Users size={18} />, color: 'bg-green-500' },
  { id: 'org', labelKey: 'contacts.menu.organization', icon: <Building2 size={18} />, color: 'bg-blue-500' },
  { id: 'tags', labelKey: 'contacts.menu.tags', icon: <Tag size={18} />, color: 'bg-purple-500' },
];

export const ContactsView: React.FC<{
  onSendMessage?: (user: UserType) => void;
  onStartCall?: (type: 'voice' | 'video', user: UserType) => void;
  onAddFriend?: () => void;
  onAppSelect?: (appId: string) => void;
  onOpenGroup?: (group: Chat) => void;
  searchQuery?: string;
}> = ({ onSendMessage, onStartCall, onAddFriend, onAppSelect, onOpenGroup, searchQuery = '' }) => {
  const { t } = useTranslation();
  const [activeId, setActiveId] = useState('all');
  const [starredContacts, setStarredContacts] = useState<UserType[]>([]);
  const [loading, setLoading] = useState(true);
  
  // Global selected user state for the right panel across different views
  const [selectedUser, setSelectedUser] = useState<{ user: UserType, deptName: string } | null>(null);

  // Clear selected user when active tab changes to a main tab
  useEffect(() => {
    if (activeId !== '_starred') {
      setSelectedUser(null);
    }
  }, [activeId]);

  useEffect(() => {
    let isMounted = true;

    const loadStarredContacts = () => {
      setLoading(true);
      return contactService.getStarredContacts()
        .then((contacts) => {
          if (isMounted) {
            setStarredContacts(contacts);
          }
        })
        .catch(() => {
          if (isMounted) {
            toast(t('contacts.starred.loadFailed'), 'error');
          }
        })
        .finally(() => {
          if (isMounted) {
            setLoading(false);
          }
        });
    };

    void loadStarredContacts();

    const refreshStarredContacts = () => {
      void loadStarredContacts();
    };

    const refreshSelectedUser = () => {
      setSelectedUser((current) => {
        if (!current) {
          return null;
        }
        void contactService.getContacts()
          .then((contacts) => {
            if (!isMounted) {
              return;
            }
            const updated = contacts.find((contact) => contact.id === current.user.id);
            if (!updated) {
              setSelectedUser(null);
              return;
            }
            if (updated.name !== current.user.name || updated.avatar !== current.user.avatar) {
              setSelectedUser({ user: updated, deptName: current.deptName });
            }
          })
          .catch(() => undefined);
        return current;
      });
    };

    window.addEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshStarredContacts);
    window.addEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshSelectedUser);

    return () => {
      isMounted = false;
      window.removeEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshStarredContacts);
      window.removeEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshSelectedUser);
    };
  }, [t]);

  return (
    <div className="flex flex-1 min-h-0">
      {/* Left List */}
      <div className="flex w-[280px] shrink-0 flex-col bg-[#202020] border-r border-white/5 min-h-0">
        <div className="flex-1 overflow-y-auto custom-scrollbar py-2">
          {menuItems.map(item => (
            <div 
              key={item.id}
              onClick={() => setActiveId(item.id)}
              className={cn(
                "flex items-center px-4 py-3 cursor-pointer transition-colors hover:bg-white/5",
                activeId === item.id && "bg-white/10 hover:bg-white/10"
              )}
            >
              <div className={cn("w-[36px] h-[36px] rounded-lg flex items-center justify-center text-white shrink-0 mr-3", item.color)}>
                {item.icon}
              </div>
              <span className="text-[14px] text-gray-200">{t(item.labelKey)}</span>
            </div>
          ))}
          
          <div className="px-4 py-2 mt-4 text-xs text-gray-500 font-medium flex items-center justify-between">
             {t('contacts.starred.title')}
          </div>
          {loading ? (
            <div className="px-4 py-3 text-sm text-gray-500">{t('contacts.starred.loading')}</div>
          ) : (
            starredContacts.map(contact => (
              <div 
                key={contact.id} 
                className={cn(
                  "flex items-center px-4 py-3 cursor-pointer transition-colors hover:bg-white/5",
                  selectedUser?.user.id === contact.id && activeId === '_starred' && "bg-white/10"
                )}
                onClick={() => {
                  setActiveId('_starred');
                  setSelectedUser({ user: contact, deptName: t('contacts.starred.title') });
                }}
              >
                <Avatar src={contact.avatar} alt={contact.name} className="w-[36px] h-[36px] rounded-lg mr-3 bg-[#2b2b2d]" />
                <span className="text-[14px] text-gray-200 truncate">{contact.name}</span>
              </div>
            ))
          )}
        </div>
      </div>
      
      {/* Main Content Area */}
      {activeId !== '_starred' && (
        <AnimatePresence mode="wait">
          <motion.div
            key={activeId}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            transition={{ duration: 0.2 }}
            className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0"
          >
            {activeId === 'org' ? (
               <OrgContainer onUserSelect={(u, d) => setSelectedUser({user: u, deptName: d})} selectedUserId={selectedUser?.user.id || null} onSendMessage={onSendMessage} searchQuery={searchQuery} />
            ) : activeId === 'all' ? (
               <AllContactsContainer onAddFriend={onAddFriend} onUserSelect={(u, d) => setSelectedUser({user: u, deptName: d})} selectedUserId={selectedUser?.user.id || null} searchQuery={searchQuery} />
            ) : activeId === 'tags' ? (
               <TagsContainer searchQuery={searchQuery} />
            ) : activeId === 'new' ? (
               <NewFriendsContainer onAddFriend={onAddFriend} />
            ) : activeId === 'groups' ? (
               <GroupsContainer searchQuery={searchQuery} onOpenGroup={onOpenGroup} />
            ) : (
              <div className="flex-1 flex flex-col min-w-0 items-center justify-center">
                <Building2 size={64} className="text-gray-600 mb-4" />
                <h3 className="text-xl text-gray-300 font-medium mb-2">
                  {menuItems.find(m => m.id === activeId)?.labelKey
                    ? t(menuItems.find(m => m.id === activeId)?.labelKey as string)
                    : t('contacts.unknownContent')}
                </h3>
                <p className="text-gray-500 text-sm">{t('contacts.missingView')}</p>
              </div>
            )}
          </motion.div>
        </AnimatePresence>
      )}

      {/* Global Right Panel for selected user across Left Sidebar options */}
      {selectedUser ? (
        <ContactDetailPane user={selectedUser.user} departmentName={selectedUser.deptName} fullWidth={activeId === '_starred'} onSendMessage={onSendMessage} onStartCall={onStartCall} onAppSelect={onAppSelect} />
      ) : activeId === '_starred' ? (
         <motion.div 
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.3 }}
            className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 items-center justify-center"
         >
            <User size={64} className="text-gray-600 mb-4 opacity-50" />
            <h3 className="text-xl text-gray-300 font-medium mb-2">{t('contacts.starred.emptyTitle')}</h3>
            <p className="text-gray-500 text-sm">{t('contacts.starred.emptyDescription')}</p>
         </motion.div>
      ) : activeId === 'org' || activeId === 'all' ? (
         <motion.div 
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.3 }}
            className="w-[360px] lg:w-[420px] flex-shrink-0 bg-[#1e1e1e] flex flex-col items-center justify-center border-l border-white/5 border-dashed hidden md:flex"
         >
            <User size={64} className="text-gray-700 mx-auto mb-4 opacity-50" />
            <p className="text-gray-500 text-sm font-medium">{t('contacts.emptyDetail')}</p>
         </motion.div>
      ) : null}
    </div>
  );
};
