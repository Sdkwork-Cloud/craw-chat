import React, { useCallback, useState, useEffect } from 'react';
import { Plus } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { Avatar } from '@sdkwork/im-pc-commons';
import { toast } from '../Toast';
import { contactService, SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT } from '../../services/ContactService';
import type { FriendRequest } from '../../services/ContactService';

export const NewFriendsContainer: React.FC<{ onAddFriend?: () => void }> = ({ onAddFriend }) => {
  const { t } = useTranslation();
  const [requests, setRequests] = useState<FriendRequest[]>([]);
  const [loading, setLoading] = useState(true);

  const refreshRequests = useCallback((showLoading = false) => {
    if (showLoading) {
      setLoading(true);
    }
    return contactService.getFriendRequests()
      .then((data) => {
        setRequests(data);
      })
      .catch(() => {
        setRequests([]);
        toast(t('contacts.newFriends.toast.loadFailed'), 'error');
      })
      .finally(() => setLoading(false));
  }, [t]);

  useEffect(() => {
    void refreshRequests(true);
    const refreshOnChange = () => {
      void refreshRequests();
    };
    const unsubscribeCount = contactService.subscribePendingFriendRequestCount(refreshOnChange);
    window.addEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshOnChange);
    return () => {
      unsubscribeCount();
      window.removeEventListener(SDKWORK_IM_FRIEND_REQUESTS_CHANGED_EVENT, refreshOnChange);
    };
  }, [refreshRequests]);

  const handleAccept = async (requestId: number) => {
    try {
      await contactService.handleFriendRequest(requestId, 'accept');
      toast(t('contacts.newFriends.toast.acceptSucceeded'), 'success');
      await refreshRequests();
    } catch {
      toast(t('contacts.newFriends.toast.handleFailed'), 'error');
    }
  };

  const handleReject = async (requestId: number) => {
    try {
      await contactService.handleFriendRequest(requestId, 'reject');
      toast(t('contacts.newFriends.toast.rejectSucceeded'), 'success');
      await refreshRequests();
    } catch {
      toast(t('contacts.newFriends.toast.handleFailed'), 'error');
    }
  };

  const handleCancel = async (requestId: number) => {
    try {
      await contactService.cancelFriendRequest(requestId);
      toast(t('contacts.newFriends.toast.cancelSucceeded'), 'success');
      await refreshRequests();
    } catch {
      toast(t('contacts.newFriends.toast.cancelFailed'), 'error');
    }
  };

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 h-full">
      <div className="px-8 py-6 border-b border-white/5 shrink-0 flex items-center justify-between">
        <div>
          <h2 className="text-xl font-medium text-gray-200">{t('contacts.newFriends.title')}</h2>
          <p className="text-sm text-gray-500 mt-1">{t('contacts.newFriends.description')}</p>
        </div>
        <button
          onClick={() => onAddFriend?.()}
          className="flex items-center gap-2 px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white text-sm font-medium rounded-lg transition-colors shadow-lg shadow-indigo-500/20"
        >
          <Plus size={16} /> {t('contacts.newFriends.addFriend')}
        </button>
      </div>
      <div className="flex-1 overflow-y-auto custom-scrollbar p-8">
        <div className="flex flex-col gap-4 max-w-3xl">
          {loading ? (
            <div className="text-sm text-gray-500">{t('contacts.newFriends.loading')}</div>
          ) : requests.map((req) => (
            <div key={req.id} className="flex items-center justify-between p-4 rounded-2xl bg-white/5 hover:bg-white/10 transition-colors border border-white/5">
              <div className="flex items-center gap-4">
                <Avatar src={req.avatar} alt={req.name} fallback={req.name.charAt(0)} className="w-12 h-12 rounded-full" />
                <div>
                  <h4 className="text-md font-medium text-gray-200">{req.name}</h4>
                  <p className="text-sm text-gray-500 mt-0.5">
                    {req.msg || (req.direction === 'outgoing'
                      ? t('contacts.newFriends.outgoingDefaultMsg')
                      : t('contacts.newFriends.incomingDefaultMsg'))}
                  </p>
                </div>
              </div>
              <div>
                {req.status === 'pending' && req.direction === 'incoming' ? (
                  <div className="flex items-center gap-2">
                    <button
                      onClick={() => void handleReject(req.id)}
                      className="px-4 py-1.5 text-sm font-medium bg-white/10 hover:bg-white/20 text-gray-300 rounded-lg transition-colors"
                    >
                      {t('contacts.newFriends.reject')}
                    </button>
                    <button
                      onClick={() => void handleAccept(req.id)}
                      className="px-4 py-1.5 text-sm font-medium bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 rounded-lg transition-colors"
                    >
                      {t('contacts.newFriends.accept')}
                    </button>
                  </div>
                ) : req.status === 'pending' && req.direction === 'outgoing' ? (
                  <button
                    onClick={() => void handleCancel(req.id)}
                    className="px-4 py-1.5 text-sm font-medium bg-white/10 hover:bg-white/20 text-gray-300 rounded-lg transition-colors"
                  >
                    {t('contacts.newFriends.cancel')}
                  </button>
                ) : (
                  <span className="text-xs text-gray-500 px-4">
                    {req.status === 'added'
                      ? t('contacts.newFriends.statusAdded')
                      : req.direction === 'outgoing'
                        ? t('contacts.newFriends.statusCancelled')
                        : t('contacts.newFriends.statusRejected')}
                  </span>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
