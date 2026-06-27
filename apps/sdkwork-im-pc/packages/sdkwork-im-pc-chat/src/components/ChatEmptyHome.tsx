import React from 'react';
import { motion } from 'motion/react';
import { useTranslation } from 'react-i18next';
import {
  Bot,
  Contact,
  MessageSquarePlus,
  RefreshCw,
  Sparkles,
  UserPlus,
} from 'lucide-react';

interface ChatEmptyHomeProps {
  assistantAvailable: boolean;
  isStartupLoading?: boolean;
  onAddFriend: () => void;
  onCreateAgent: () => void;
  onCreateGroup: () => void;
  onOpenAssistant: () => void;
  onOpenContacts: () => void;
  onRetryStartup?: () => void;
  startupError?: string | null;
}

interface EmptyHomeAction {
  description: string;
  disabled?: boolean;
  icon: React.ReactNode;
  onClick: () => void;
  title: string;
}

export const ChatEmptyHome: React.FC<ChatEmptyHomeProps> = ({
  assistantAvailable,
  isStartupLoading = false,
  onAddFriend,
  onCreateAgent,
  onCreateGroup,
  onOpenAssistant,
  onOpenContacts,
  onRetryStartup,
  startupError,
}) => {
  const { t } = useTranslation();
  const actions: EmptyHomeAction[] = [
    {
      description: t('chat.emptyHome.actions.addFriend.description'),
      icon: <UserPlus size={18} />,
      onClick: onAddFriend,
      title: t('chat.emptyHome.actions.addFriend.title'),
    },
    {
      description: t('chat.emptyHome.actions.createGroup.description'),
      icon: <MessageSquarePlus size={18} />,
      onClick: onCreateGroup,
      title: t('chat.emptyHome.actions.createGroup.title'),
    },
    {
      description: t('chat.emptyHome.actions.openAssistant.description'),
      disabled: !assistantAvailable,
      icon: <Bot size={18} />,
      onClick: onOpenAssistant,
      title: t('chat.emptyHome.actions.openAssistant.title'),
    },
    {
      description: t('chat.emptyHome.actions.openContacts.description'),
      icon: <Contact size={18} />,
      onClick: onOpenContacts,
      title: t('chat.emptyHome.actions.openContacts.title'),
    },
    {
      description: t('chat.emptyHome.actions.createAgent.description'),
      icon: <Sparkles size={18} />,
      onClick: onCreateAgent,
      title: t('chat.emptyHome.actions.createAgent.title'),
    },
  ];
  const actionDescription = (action: EmptyHomeAction): string => (
    action.disabled && action.onClick === onOpenAssistant
      ? t('chat.emptyHome.actions.openAssistant.unavailableDescription')
      : action.description
  );

  return (
    <motion.div
      initial={{ opacity: 0, y: 12 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.24 }}
      className="flex-1 overflow-y-auto bg-[#1e1e1e] px-8 py-8 text-gray-200"
    >
      <div className="mx-auto flex min-h-full w-full max-w-3xl flex-col justify-center gap-7">
        <section className="flex flex-col gap-3">
          <div className="flex h-10 w-10 items-center justify-center text-[#48e06d]">
            <Sparkles size={24} />
          </div>
          <div className="text-[12px] font-medium uppercase text-gray-500">
            {t('chat.emptyHome.eyebrow')}
          </div>
          <h1 className="text-[28px] font-semibold leading-tight text-white">
            {t('chat.emptyHome.title')}
          </h1>
          <p className="max-w-2xl text-sm leading-6 text-gray-400">
            {t('chat.emptyHome.description')}
          </p>
        </section>

        <div className="flex flex-col gap-1">
          {actions.map((action) => (
            <button
              key={action.title}
              type="button"
              onClick={action.onClick}
              disabled={action.disabled}
              className="group flex w-full items-center gap-4 px-1 py-4 text-left transition-colors hover:bg-white/[0.03] disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:bg-transparent"
            >
              <span className="flex h-9 w-9 shrink-0 items-center justify-center text-gray-400 transition-colors group-hover:text-[#48e06d]">
                {action.icon}
              </span>
              <span className="min-w-0 flex-1">
                <span className="block text-sm font-medium text-gray-100">
                  {action.title}
                </span>
                <span className="mt-1 block text-[12px] leading-5 text-gray-500">
                  {actionDescription(action)}
                </span>
              </span>
            </button>
          ))}
        </div>

        {(isStartupLoading || startupError) && (
          <div className="flex flex-wrap items-center justify-between gap-3 bg-white/[0.03] px-3 py-3 text-sm text-gray-300">
            <span>
              {isStartupLoading ? t('chat.emptyHome.status.preparing') : startupError ?? t('chat.emptyHome.status.fallbackLoadError')}
            </span>
            {!isStartupLoading && onRetryStartup && (
              <button
                type="button"
                onClick={onRetryStartup}
                className="inline-flex h-8 items-center gap-2 bg-white/[0.04] px-3 text-[12px] font-medium text-gray-100 transition-colors hover:bg-white/[0.08]"
              >
                <RefreshCw size={14} />
                {t('chat.emptyHome.status.retry')}
              </button>
            )}
          </div>
        )}
      </div>
    </motion.div>
  );
};
