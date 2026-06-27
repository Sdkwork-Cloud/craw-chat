import React, { useState, useEffect } from 'react';
import { User, Users, Building2, Hash, ChevronRight, MessageSquare, Phone, Video, Search, Mail, Tag, Plus, MoreHorizontal, MoreVertical, Edit2, Trash2, UserPlus, Star } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { cn } from '@sdkwork/im-pc-commons';
import { toast } from '../Toast';
import { contactService } from '../../services/ContactService';
import type { ContactTag } from '../../services/ContactService';
import { PromptModal, usePrompt } from '../PromptModal';

export const TagsContainer: React.FC<{ searchQuery?: string }> = ({ searchQuery = '' }) => {
  const [selectedTag, setSelectedTag] = useState<string | null>(null);
  const [tags, setTags] = useState<ContactTag[]>([]);
  const [loading, setLoading] = useState(true);

  const { promptConfig, customPrompt, closePrompt } = usePrompt();

  useEffect(() => {
    contactService.getTags()
      .then(data => {
        setTags(data);
      })
      .catch(() => {
        setTags([]);
        toast('加载标签失败', 'error');
      })
      .finally(() => setLoading(false));
  }, []);

  const filteredTags = tags.filter(tag => {
    if (!searchQuery.trim()) return true;
    return tag.name.toLowerCase().includes(searchQuery.toLowerCase());
  });

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 h-full">
      <div className="px-8 py-6 border-b border-white/5 shrink-0 flex items-center justify-between">
        <div>
          <h2 className="text-xl font-medium text-gray-200">标签管理</h2>
          <p className="text-sm text-gray-500 mt-1">分类管理您的联系人，方便快速查找</p>
        </div>
        <button 
          onClick={() => {
             customPrompt("请输入新标签名称：", "", async (name) => {
               try {
                 if (name && name.trim()) {
                  const newTag = await contactService.addTag({ 
                    name: name.trim(), 
                    color: 'bg-indigo-500', 
                    count: 0, 
                    bg: 'bg-indigo-500/10', 
                    border: 'border-indigo-500/20' 
                  });
                  setTags([...tags, newTag]);
                  toast('新建标签成功', 'success');
                 }
               } catch {
                 toast('新建标签失败', 'error');
               } finally {
                 closePrompt();
               }
             });
          }}
          className="flex items-center gap-2 px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white text-sm font-medium rounded-lg transition-colors shadow-lg shadow-indigo-500/20"
        >
          <Plus size={16} /> 新建标签
        </button>
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar p-8">
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
          {filteredTags.map((tag) => (
            <motion.div
              layoutId={`tag-${tag.id}`}
              key={tag.id}
              onClick={() => setSelectedTag(tag.id)}
              className={cn(
                "group relative p-5 rounded-2xl border cursor-pointer transition-all",
                tag.bg, tag.border,
                "hover:scale-[1.02] hover:shadow-xl hover:shadow-black/20"
              )}
            >
              <div className="flex flex-col h-full gap-4 relative z-10">
                <div className="flex items-center justify-between">
                  <div className={cn("w-10 h-10 rounded-full flex items-center justify-center text-white shadow-inner", tag.color)}>
                    <Hash size={20} />
                  </div>
                  <button 
                    onClick={(e) => { 
                       e.stopPropagation(); 
                       customPrompt("请输入新的标签名称", tag.name, async (name) => {
                         try {
                           if (name && name.trim() && name !== tag.name) {
                            await contactService.updateTag(tag.id, { name: name.trim() });
                            setTags(tags.map(t => t.id === tag.id ? {...t, name: name.trim()} : t));
                            toast('重命名成功', 'success');
                           }
                         } catch {
                           toast('重命名失败', 'error');
                         } finally {
                           closePrompt();
                         }
                       });
                    }}
                    className="w-8 h-8 rounded-full flex items-center justify-center text-gray-400 hover:bg-white/10 hover:text-white transition-colors opacity-0 group-hover:opacity-100"
                    title="重命名"
                  >
                    <Edit2 size={16} />
                  </button>
                </div>
                <div>
                  <h3 className="text-lg font-medium text-gray-200 group-hover:text-white transition-colors">{tag.name}</h3>
                  <p className="text-sm text-gray-500 mt-1">{tag.count} 个联系人</p>
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      </div>

      {/* Selected Tag Overlay */}
      <AnimatePresence>
        {selectedTag && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="absolute inset-0 z-50 bg-[#1e1e1e] flex flex-col"
          >
            {(() => {
              const tag = tags.find(t => t.id === selectedTag);
              if (!tag) return null;
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
                        <div className={cn("w-8 h-8 rounded-full flex items-center justify-center text-white", tag.color)}>
                          <Hash size={16} />
                        </div>
                        <div>
                          <h2 className="text-xl font-medium text-gray-200">{tag.name}</h2>
                          <p className="text-xs text-gray-500">{tag.count} 个联系人</p>
                        </div>
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                       <button onClick={() => {
                         customPrompt("请输入要添加的联系人ID，逗号分隔：", "", async (userIds) => {
                           try {
                             if (userIds && userIds.trim()) {
                              const countAdded = userIds.split(',').filter(Boolean).length;
                              const updatedTag = await contactService.updateTag(tag.id, { count: tag.count + countAdded });
                              setTags(tags.map(t => t.id === tag.id ? updatedTag : t));
                              toast(`已成功将 ${countAdded} 个成员加入该标签`, 'success');
                             }
                           } catch {
                             toast('添加标签成员失败', 'error');
                           } finally {
                             closePrompt();
                           }
                         });
                       }} className="flex items-center gap-2 px-3 py-1.5 bg-white/5 hover:bg-white/10 text-gray-300 text-sm font-medium rounded-lg transition-colors border border-white/10">
                         <Plus size={14} /> 添加人员
                       </button>
                       <button onClick={() => {
                         customPrompt("请输入新的标签名称", tag.name, async (name) => {
                           try {
                             if (name && name.trim() && name !== tag.name) {
                              await contactService.updateTag(tag.id, { name: name.trim() });
                              setTags(tags.map(t => t.id === tag.id ? {...t, name: name.trim()} : t));
                              toast('重命名成功', 'success');
                             }
                           } catch {
                             toast('重命名失败', 'error');
                           } finally {
                             closePrompt();
                           }
                         });
                       }} className="p-2 text-gray-400 hover:bg-white/10 rounded-lg transition-colors">
                         <Edit2 size={16} />
                       </button>
                       <button 
                         onClick={async () => {
                           try {
                             await contactService.removeTag(tag.id);
                             toast('标签已删除', 'success');
                             setTags(tags.filter(t => t.id !== tag.id));
                             setSelectedTag(null);
                           } catch {
                             toast('删除标签失败', 'error');
                           }
                         }} 
                         className="p-2 text-red-400 hover:bg-red-400/10 rounded-lg transition-colors"
                       >
                         <Trash2 size={16} />
                       </button>
                    </div>
                  </div>
                  <div className="flex-1 flex items-center justify-center">
                    <div className="text-center text-gray-500">
                      <Users size={48} className="mx-auto mb-4 opacity-50" />
                      <p>这里将展示 "{tag.name}" 标签下的联系人列表</p>
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
