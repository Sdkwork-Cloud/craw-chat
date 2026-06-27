import React, { useState } from 'react';
import { Loader2, Search, UserPlus } from 'lucide-react';
import { motion } from 'motion/react';
import { useTranslation } from 'react-i18next';
import { Avatar } from '@sdkwork/im-pc-commons';
import { toast } from './Toast';
import { contactService } from '../services/ContactService';
import { ModalWrapper } from './ModalWrapper';
import type { User } from '@sdkwork/im-pc-types';

interface AddFriendSearchResult {
  avatar?: string;
  desc: string;
  id: string;
  name: string;
}

function buildSearchResultDescription(user: User): string {
  return user.chatId
    || user.email
    || user.phone
    || user.position
    || user.company
    || user.location
    || user.motto
    || user.name;
}

export const AddFriendModal: React.FC<{ isOpen: boolean; onClose: () => void }> = ({ isOpen, onClose }) => {
  const { t } = useTranslation();
  const [searchQuery, setSearchQuery] = useState('');
  const [isSearching, setIsSearching] = useState(false);
  const [searchNotice, setSearchNotice] = useState<{ type: 'loading' | 'empty'; message: string } | null>(null);
  const [result, setResult] = useState<AddFriendSearchResult | null>(null);

  const handleSearch = async () => {
    const normalizedQuery = searchQuery.trim();
    if (!normalizedQuery) {
      return;
    }

    setIsSearching(true);
    setResult(null);
    setSearchNotice({ type: 'loading', message: t('contacts.addFriend.notice.searching') });
    try {
      const results = await contactService.searchContacts(normalizedQuery);
      if (results.length > 0) {
        setSearchNotice(null);
        setResult({
          id: results[0].id,
          name: results[0].name,
          avatar: results[0].avatar,
          desc: buildSearchResultDescription(results[0]),
        });
      } else {
        setSearchNotice({ type: 'empty', message: t('contacts.addFriend.notice.noResults', { query: normalizedQuery }) });
      }
    } catch {
      setSearchNotice(null);
      toast(t('contacts.addFriend.toast.searchFailed'), 'error');
    } finally {
      setIsSearching(false);
    }
  };

  React.useEffect(() => {
    if (!isOpen) {
      setSearchQuery('');
      setResult(null);
      setSearchNotice(null);
    }
  }, [isOpen]);

  return (
    <ModalWrapper
      isOpen={isOpen}
      onClose={onClose}
      title={t('contacts.addFriend.title')}
      width="w-[640px]"
      height="h-[520px]"
    >
      <div className="flex h-full min-h-0 flex-col">
        <div className="mb-4 flex shrink-0 gap-2">
          <div className="relative flex-1">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
            <input
              type="text"
              placeholder={t('contacts.addFriend.placeholder')}
              value={searchQuery}
              onChange={(event) => {
                setSearchQuery(event.target.value);
                setSearchNotice(null);
              }}
              onKeyDown={(event) => {
                if (event.key === 'Enter') {
                  void handleSearch();
                }
              }}
              className="h-10 w-full rounded-lg border border-white/5 bg-[#181818] py-2 pl-9 pr-4 text-sm text-gray-200 outline-none transition-colors focus:border-white/20"
            />
          </div>
          <button
            onClick={handleSearch}
            disabled={!searchQuery.trim() || isSearching}
            className="flex h-10 shrink-0 items-center gap-2 rounded-lg bg-[#00b42a] px-4 text-sm text-white transition-colors hover:bg-[#009a24] disabled:cursor-not-allowed disabled:bg-[#00b42a]/50"
          >
            {isSearching && <Loader2 size={14} className="animate-spin" />}
            {isSearching ? t('contacts.addFriend.searchingAction') : t('contacts.addFriend.searchAction')}
          </button>
        </div>

        {searchNotice && (
          <motion.div
            initial={{ opacity: 0, y: -4 }}
            animate={{ opacity: 1, y: 0 }}
            className={`mb-4 flex shrink-0 items-center gap-2 rounded-lg border px-3 py-2 text-xs ${
              searchNotice.type === 'loading'
                ? 'border-white/5 bg-white/5 text-gray-400'
                : 'border-amber-500/20 bg-amber-500/10 text-amber-300'
            }`}
            aria-live="polite"
          >
            {searchNotice.type === 'loading' && <Loader2 size={14} className="animate-spin" />}
            <span>{searchNotice.message}</span>
          </motion.div>
        )}

        <div className="min-h-0 flex-1 overflow-y-auto custom-scrollbar">
          {result && (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              className="flex items-center justify-between rounded-lg border border-white/5 bg-[#181818] p-4"
            >
              <div className="flex min-w-0 items-center gap-3">
                <Avatar src={result.avatar} alt={result.name} className="h-11 w-11 rounded bg-[#2b2b2d]" />
                <div className="min-w-0">
                  <div className="truncate text-sm font-medium text-gray-200">{result.name}</div>
                  <div className="mt-0.5 truncate text-xs text-gray-500">{result.desc}</div>
                </div>
              </div>
              <button
                className="flex h-8 shrink-0 items-center gap-1 rounded bg-white/10 px-3 text-sm text-blue-400 transition-colors hover:bg-white/20"
                onClick={async () => {
                  try {
                    await contactService.addFriend(result.id);
                    toast(t('contacts.addFriend.toast.requestSent', { name: result.name }), 'success');
                    onClose();
                  } catch (error) {
                    const message = error instanceof Error ? error.message : '';
                    if (message.includes('already a friend')) {
                      toast(t('contacts.addFriend.toast.alreadyFriend'), 'info');
                      return;
                    }
                    if (message.includes('already pending')) {
                      toast(t('contacts.addFriend.toast.requestPending'), 'info');
                      return;
                    }
                    toast(t('contacts.addFriend.toast.requestFailed'), 'error');
                  }
                }}
              >
                <UserPlus size={14} />
                {t('contacts.addFriend.add')}
              </button>
            </motion.div>
          )}
        </div>
      </div>
    </ModalWrapper>
  );
};
