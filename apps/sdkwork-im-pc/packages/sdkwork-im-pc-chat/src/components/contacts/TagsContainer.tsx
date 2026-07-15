import React, { useState, useEffect } from 'react';
import { Hash, ChevronRight, Plus, Edit2, Trash2 } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { useTranslation } from 'react-i18next';
import { cn } from '@sdkwork/im-pc-commons';
import { toast } from '../Toast';
import { contactService } from '../../services/ContactService';
import type { ContactTag } from '../../services/ContactService';
import { PromptModal, usePrompt } from '../PromptModal';

export const TagsContainer: React.FC<{ searchQuery?: string }> = ({ searchQuery = '' }) => {
  const { t } = useTranslation();
  const [selectedTag, setSelectedTag] = useState<string | null>(null);
  const [tags, setTags] = useState<ContactTag[]>([]);
  const [loading, setLoading] = useState(true);

  const { promptConfig, customPrompt, closePrompt } = usePrompt();

  useEffect(() => {
    contactService.getTags()
      .then((data) => {
        setTags(data);
      })
      .catch(() => {
        setTags([]);
        toast(t('contacts.tags.toast.loadFailed'), 'error');
      })
      .finally(() => setLoading(false));
  }, [t]);

  const filteredTags = tags.filter((tag) => {
    if (!searchQuery.trim()) {
      return true;
    }
    return tag.name.toLowerCase().includes(searchQuery.toLowerCase());
  });

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 h-full">
      <div className="px-8 py-6 border-b border-white/5 shrink-0 flex items-center justify-between">
        <div>
          <h2 className="text-xl font-medium text-gray-200">{t('contacts.tags.title')}</h2>
          <p className="text-sm text-gray-500 mt-1">{t('contacts.tags.description')}</p>
        </div>
        <button
          onClick={() => {
            customPrompt(t('contacts.tags.prompt.createName'), '', async (name) => {
              try {
                if (name && name.trim()) {
                  const newTag = await contactService.addTag({
                    name: name.trim(),
                    color: 'bg-indigo-500',
                    count: 0,
                    bg: 'bg-indigo-500/10',
                    border: 'border-indigo-500/20',
                  });
                  setTags([...tags, newTag]);
                  toast(t('contacts.tags.toast.createSucceeded'), 'success');
                }
              } catch {
                toast(t('contacts.tags.toast.createFailed'), 'error');
              } finally {
                closePrompt();
              }
            });
          }}
          className="flex items-center gap-2 px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white text-sm font-medium rounded-lg transition-colors shadow-lg shadow-indigo-500/20"
        >
          <Plus size={16} /> {t('contacts.tags.create')}
        </button>
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar p-8">
        {loading ? (
          <div className="text-sm text-gray-500">{t('contacts.starred.loading')}</div>
        ) : (
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
            {filteredTags.map((tag) => (
              <motion.div
                layoutId={`tag-${tag.id}`}
                key={tag.id}
                onClick={() => setSelectedTag(tag.id)}
                className={cn(
                  'group relative p-5 rounded-2xl border cursor-pointer transition-all',
                  tag.bg, tag.border,
                  'hover:scale-[1.02] hover:shadow-xl hover:shadow-black/20',
                )}
              >
                <div className="flex flex-col h-full gap-4 relative z-10">
                  <div className="flex items-center justify-between">
                    <div className={cn('w-10 h-10 rounded-full flex items-center justify-center text-white shadow-inner', tag.color)}>
                      <Hash size={20} />
                    </div>
                    <button
                      onClick={(event) => {
                        event.stopPropagation();
                        customPrompt(t('contacts.tags.prompt.renameName'), tag.name, async (name) => {
                          try {
                            if (name && name.trim() && name !== tag.name) {
                              await contactService.updateTag(tag.id, { name: name.trim() });
                              setTags(tags.map((entry) => entry.id === tag.id ? { ...entry, name: name.trim() } : entry));
                              toast(t('contacts.tags.toast.renameSucceeded'), 'success');
                            }
                          } catch {
                            toast(t('contacts.tags.toast.renameFailed'), 'error');
                          } finally {
                            closePrompt();
                          }
                        });
                      }}
                      className="w-8 h-8 rounded-full flex items-center justify-center text-gray-400 hover:bg-white/10 hover:text-white transition-colors opacity-0 group-hover:opacity-100"
                      title={t('contacts.tags.rename')}
                    >
                      <Edit2 size={16} />
                    </button>
                  </div>
                  <div>
                    <h3 className="text-lg font-medium text-gray-200 group-hover:text-white transition-colors">{tag.name}</h3>
                    <p className="text-sm text-gray-500 mt-1">{t('contacts.tags.contactCount', { count: tag.count })}</p>
                  </div>
                </div>
              </motion.div>
            ))}
          </div>
        )}
      </div>

      <AnimatePresence>
        {selectedTag && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 z-50 bg-[#1e1e1e] flex flex-col"
          >
            {(() => {
              const tag = tags.find((entry) => entry.id === selectedTag);
              if (!tag) {
                return null;
              }
              return (
                <>
                  <div className="px-8 py-6 border-b border-white/5 shrink-0 flex items-center justify-between">
                    <div className="flex items-center gap-4">
                      <button
                        onClick={() => setSelectedTag(null)}
                        className="w-8 h-8 rounded-full flex items-center justify-center text-gray-400 hover:bg-white/10 hover:text-white transition-colors"
                      >
                        <ChevronRight size={20} className="rotate-180" />
                      </button>
                      <div className="flex items-center gap-3">
                        <div className={cn('w-8 h-8 rounded-full flex items-center justify-center text-white', tag.color)}>
                          <Hash size={16} />
                        </div>
                        <div>
                          <h2 className="text-xl font-medium text-gray-200">{tag.name}</h2>
                          <p className="text-xs text-gray-500">{t('contacts.tags.contactCount', { count: tag.count })}</p>
                        </div>
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      <button
                        onClick={() => {
                          customPrompt(t('contacts.tags.prompt.renameName'), tag.name, async (name) => {
                            try {
                              if (name && name.trim() && name !== tag.name) {
                                await contactService.updateTag(tag.id, { name: name.trim() });
                                setTags(tags.map((entry) => entry.id === tag.id ? { ...entry, name: name.trim() } : entry));
                                toast(t('contacts.tags.toast.renameSucceeded'), 'success');
                              }
                            } catch {
                              toast(t('contacts.tags.toast.renameFailed'), 'error');
                            } finally {
                              closePrompt();
                            }
                          });
                        }}
                        className="p-2 text-gray-400 hover:bg-white/10 rounded-lg transition-colors"
                      >
                        <Edit2 size={16} />
                      </button>
                      <button
                        onClick={async () => {
                          try {
                            await contactService.removeTag(tag.id);
                            toast(t('contacts.tags.toast.deleteSucceeded'), 'success');
                            setTags(tags.filter((entry) => entry.id !== tag.id));
                            setSelectedTag(null);
                          } catch {
                            toast(t('contacts.tags.toast.deleteFailed'), 'error');
                          }
                        }}
                        className="p-2 text-red-400 hover:bg-red-400/10 rounded-lg transition-colors"
                      >
                        <Trash2 size={16} />
                      </button>
                    </div>
                  </div>
                  <div className="flex flex-1 items-center justify-center">
                    <div className="text-center text-gray-500">
                      <Hash size={48} className="mx-auto mb-4 opacity-50" />
                      <p>{t('contacts.tags.contactCount', { count: tag.count })}</p>
                    </div>
                  </div>
                </>
              );
            })()}
          </motion.div>
        )}
      </AnimatePresence>
      <PromptModal {...promptConfig} onCancel={closePrompt} />
    </div>
  );
};
