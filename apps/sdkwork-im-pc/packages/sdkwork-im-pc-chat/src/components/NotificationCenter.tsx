import React, { useCallback, useEffect, useState } from 'react';
import { AnimatePresence, motion } from 'motion/react';
import { Bell, X } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { cn } from '@sdkwork/im-pc-commons';
import type { IncomingAppNotification } from '../services/NotificationService';

interface NotificationCenterProps {
  onOpenCall: (notification: Extract<IncomingAppNotification, { kind: 'call' }>) => void;
  onOpenConversation: (conversationId: string) => void;
}

type NotificationItem = IncomingAppNotification & {
  createdAt: number;
  id: string;
};

type NotifyEvent = CustomEvent<IncomingAppNotification>;

const NOTIFICATION_EVENT = 'sdkwork-im-pc:notify-app';
const LEGACY_MESSAGE_NOTIFICATION_EVENT = 'sdkwork-im-pc:notify-message';
const NOTIFICATION_VISIBLE_MS = 6200;
const MAX_VISIBLE_NOTIFICATIONS = 4;

function isNotifyEvent(event: Event): event is NotifyEvent {
  const detail = (event as NotifyEvent).detail;
  return 'detail' in event
    && (
      Boolean(detail?.kind === 'message' && detail.messageId)
      || Boolean(detail?.kind === 'call' && detail.callId)
    );
}

function resolveNotificationItemId(notification: IncomingAppNotification): string {
  return notification.kind === 'call'
    ? `call:${notification.callId}`
    : `message:${notification.messageId}`;
}

export function publishAppNotification(notification: IncomingAppNotification): void {
  if (typeof window === 'undefined') {
    return;
  }
  window.dispatchEvent(new CustomEvent(NOTIFICATION_EVENT, {
    detail: notification,
  }));
}

export function publishMessageNotification(notification: Extract<IncomingAppNotification, { kind: 'message' }>): void {
  publishAppNotification(notification);
}

export const NotificationCenter: React.FC<NotificationCenterProps> = ({
  onOpenCall,
  onOpenConversation,
}) => {
  const { t } = useTranslation();
  const [notifications, setNotifications] = useState<NotificationItem[]>([]);

  const dismissNotification = useCallback((notificationId: string) => {
    setNotifications((currentNotifications) =>
      currentNotifications.filter((notification) => notification.id !== notificationId),
    );
  }, []);

  useEffect(() => {
    const handleNotification = (event: Event) => {
      if (!isNotifyEvent(event)) {
        return;
      }
      const notification = event.detail;
      const notificationId = resolveNotificationItemId(notification);
      setNotifications((currentNotifications) => {
        const withoutDuplicate = currentNotifications.filter(
          (item) => item.id !== notificationId,
        );
        return [
          {
            ...notification,
            createdAt: Date.now(),
            id: notificationId,
          },
          ...withoutDuplicate,
        ].slice(0, MAX_VISIBLE_NOTIFICATIONS);
      });
    };

    window.addEventListener(NOTIFICATION_EVENT, handleNotification);
    window.addEventListener(LEGACY_MESSAGE_NOTIFICATION_EVENT, handleNotification);
    return () => {
      window.removeEventListener(NOTIFICATION_EVENT, handleNotification);
      window.removeEventListener(LEGACY_MESSAGE_NOTIFICATION_EVENT, handleNotification);
    };
  }, []);

  useEffect(() => {
    if (notifications.length === 0) {
      return undefined;
    }
    const timeout = window.setTimeout(() => {
      const expiryTime = Date.now() - NOTIFICATION_VISIBLE_MS;
      setNotifications((currentNotifications) =>
        currentNotifications.filter((notification) => notification.createdAt > expiryTime),
      );
    }, 800);
    return () => window.clearTimeout(timeout);
  }, [notifications]);

  return (
    <div className="fixed bottom-6 right-6 z-[9998] flex w-[360px] max-w-[calc(100vw-32px)] flex-col gap-3 pointer-events-none">
      <AnimatePresence initial={false}>
        {notifications.map((notification) => (
          <motion.div
            key={notification.id}
            initial={{ opacity: 0, y: 18, scale: 0.96 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, x: 24, scale: 0.98 }}
            transition={{ duration: 0.18 }}
            className={cn(
              'group pointer-events-auto overflow-hidden rounded-lg border border-white/10',
              'bg-[#242426]/96 shadow-2xl shadow-black/30 backdrop-blur-xl',
              'ring-1 ring-white/5',
            )}
          >
            <button
              type="button"
              className="flex w-full items-start gap-3 p-3 text-left"
              onClick={() => {
                dismissNotification(notification.id);
                if (notification.kind === 'call') {
                  onOpenCall(notification);
                  return;
                }
                onOpenConversation(notification.conversationId);
              }}
            >
              <div className="flex h-10 w-10 shrink-0 items-center justify-center overflow-hidden rounded-md bg-[#343438] text-white">
                {notification.icon ? (
                  <img
                    alt=""
                    className="h-full w-full object-cover"
                    src={notification.icon}
                  />
                ) : (
                  <Bell size={18} />
                )}
              </div>
              <div className="min-w-0 flex-1 pt-0.5">
                <div className="mb-1 flex items-center justify-between gap-3">
                  <div className="truncate text-[13px] font-semibold text-gray-100">
                    {notification.title}
                  </div>
                  <div className="shrink-0 text-[11px] text-gray-500">
                    {t('chat.notification.center.now')}
                  </div>
                </div>
                <div className="line-clamp-2 text-[12px] leading-5 text-gray-400">
                  {notification.body}
                </div>
              </div>
            </button>
            <button
              type="button"
              aria-label={t('chat.notification.center.dismiss')}
              className="absolute right-2 top-2 flex h-6 w-6 items-center justify-center rounded-md text-gray-500 opacity-0 transition-all hover:bg-white/10 hover:text-gray-100 group-hover:opacity-100"
              onClick={() => dismissNotification(notification.id)}
            >
              <X size={14} />
            </button>
            <div className="h-0.5 w-full bg-white/5">
              <motion.div
                className="h-full bg-blue-500/70"
                initial={{ width: '100%' }}
                animate={{ width: '0%' }}
                transition={{ duration: NOTIFICATION_VISIBLE_MS / 1000, ease: 'linear' }}
              />
            </div>
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
};
