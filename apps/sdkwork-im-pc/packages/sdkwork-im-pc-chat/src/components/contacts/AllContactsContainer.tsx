import React, { useState, useEffect } from 'react';
import { UserPlus, ChevronRight } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { Avatar } from '@sdkwork/im-pc-commons';
import { cn } from '@sdkwork/im-pc-commons';
import { toast } from '../Toast';
import { contactService, SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT } from '../../services/ContactService';
import type { User as UserType } from '@sdkwork/im-pc-types';

export const AllContactsContainer: React.FC<{
  onAddFriend?: () => void;
  onUserSelect: (user: UserType, deptName: string) => void;
  searchQuery: string;
  selectedUserId: string | null;
}> = ({ onAddFriend, onUserSelect, selectedUserId, searchQuery }) => {
  const { t } = useTranslation();
  const [contacts, setContacts] = useState<UserType[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let isMounted = true;

    const loadContacts = () => {
      setLoading(true);
      return contactService.getContacts()
        .then((data) => {
          if (isMounted) {
            setContacts(data);
          }
        })
        .catch(() => {
          if (isMounted) {
            setContacts([]);
            toast(t('contacts.allContacts.toast.loadFailed'), 'error');
          }
        })
        .finally(() => {
          if (isMounted) {
            setLoading(false);
          }
        });
    };

    void loadContacts();

    const refreshContacts = () => {
      void loadContacts();
    };
    window.addEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshContacts);

    return () => {
      isMounted = false;
      window.removeEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshContacts);
    };
  }, [t]);

  const filteredContacts = contacts.filter(user => {
    if (!searchQuery.trim()) return true;
    return user.name.toLowerCase().includes(searchQuery.toLowerCase()) || user.email?.toLowerCase().includes(searchQuery.toLowerCase());
  });

  const groupedContacts = filteredContacts.reduce((acc, user) => {
    let firstChar = user.name.charAt(0).toUpperCase();
    if (user.py) {
      firstChar = user.py.charAt(0).toUpperCase();
    }
    const groupKey = /[A-Z]/.test(firstChar) ? firstChar : '#';
    !acc[groupKey] && (acc[groupKey] = []);
    acc[groupKey].push(user);
    return acc;
  }, {} as Record<string, UserType[]>);
  
  const letters = Object.keys(groupedContacts).sort();
  
  const scrollToLetter = (letter: string) => {
     const element = document.getElementById(`letter-${letter}`);
     if (element) {
        element.scrollIntoView({ behavior: 'smooth' });
     }
  };

  return (
     <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 border-r border-white/5 relative">
        <div className="px-6 py-5 border-b border-white/5 shrink-0 flex items-center justify-between bg-[#1e1e1e] z-10">
           <div>
             <h2 className="text-xl font-medium text-gray-200">{t('contacts.allContacts.title')}</h2>
             <p className="text-sm text-gray-500 mt-1">{t('contacts.allContacts.count', { count: filteredContacts.length })}</p>
           </div>
           <button 
             onClick={() => onAddFriend?.()}
             className="flex items-center gap-2 px-3 py-1.5 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 text-sm font-medium rounded-lg transition-colors border border-indigo-500/20 shadow-sm"
           >
             <UserPlus size={16} /> {t('contacts.allContacts.addContact')}
           </button>
        </div>
        
        <div className="flex-1 overflow-y-auto custom-scrollbar relative" id="all-contacts-scroll-container">
           {loading ? (
             <div className="p-8 text-center text-gray-500 text-sm">{t('contacts.allContacts.loading')}</div>
           ) : (
             letters.map(letter => (
                <div key={letter} id={`letter-${letter}`}>
                   <div className="px-6 py-1.5 bg-[#181818] text-xs text-gray-500 font-medium sticky top-0 z-10 border-y border-white/5">
                      {letter}
                   </div>
                   <div className="flex flex-col py-1">
                      {groupedContacts[letter].map(user => (
                         <div 
                           key={user.id}
                           onClick={() => onUserSelect(user, t('contacts.allContacts.groupLabel'))}
                           className={cn(
                             "flex items-center px-6 py-3 cursor-pointer transition-colors hover:bg-white/5 gap-3",
                             selectedUserId === user.id && "bg-white/10 hover:bg-white/10"
                           )}
                         >
                            <Avatar src={user.avatar} alt={user.name} className="w-10 h-10 rounded-xl bg-[#2b2b2d]" />
                            <div className="flex-1 min-w-0">
                               <div className="text-[15px] text-gray-200 truncate">{user.name}</div>
                            </div>
                            <ChevronRight size={16} className="text-gray-600 opacity-0 group-hover:opacity-100 transition-opacity" />
                         </div>
                      ))}
                   </div>
                </div>
             ))
           )}
        </div>

        <div className="absolute right-1 top-1/2 -translate-y-1/2 flex flex-col gap-[2px] z-20">
           {['↑', ...letters, '#'].map(l => (
              <button 
                 key={l}
                 onClick={() => l === '↑' ? document.getElementById('all-contacts-scroll-container')?.scrollTo({top:0, behavior:'smooth'}) : scrollToLetter(l)}
                 className="w-5 h-5 flex items-center justify-center text-[10px] text-gray-400 hover:bg-indigo-500 hover:text-white rounded-md transition-colors"
                 title={l === '↑' ? t('contacts.allContacts.scrollTop') : l}
              >
                 {l}
              </button>
           ))}
        </div>
     </div>
  );
}
