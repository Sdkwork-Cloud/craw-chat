import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "motion/react";
import { X, Bot, Search, Check, Filter } from "lucide-react";
import { Device, deviceService } from "../services/DeviceService";
import { toast, agentService, AgentConfig } from "@sdkwork/im-pc-chat";
import { cn } from "@sdkwork/im-pc-commons";

interface AgentSelectModalProps {
  device: Device;
  onClose: () => void;
  onSelect: (agentIds: string[]) => void;
  multiple?: boolean;
}

function mergeUniqueAgents(myAgents: AgentConfig[], marketAgents: AgentConfig[]): AgentConfig[] {
  const merged = new Map<string, AgentConfig>();
  for (const agent of [...myAgents, ...marketAgents]) {
    if (agent.id && !merged.has(agent.id)) {
      merged.set(agent.id, agent);
    }
  }
  return Array.from(merged.values());
}

export const BindAgentModal: React.FC<AgentSelectModalProps> = ({ device, onClose, onSelect, multiple = false }) => {
  const [agents, setAgents] = useState<AgentConfig[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedCategoryId, setSelectedCategoryId] = useState<string>("all");
  const [selectedIds, setSelectedIds] = useState<string[]>(device.agentId ? [device.agentId] : []);
  const [submitting, setSubmitting] = useState(false);

  const categories = [
    { id: 'all', name: '全部分类' },
    { id: 'device', name: '硬件专管' },
    { id: 'tech', name: '研发助手' },
    { id: 'office', name: '办公效率' }
  ];

  useEffect(() => {
    let mounted = true;
    const fetchAgents = async () => {
      try {
        const [myAgents, marketAgents] = await Promise.all([
          agentService.getAgents(),
          agentService.getMarketAgents(),
        ]);
        if (mounted) {
          setAgents(mergeUniqueAgents(myAgents, marketAgents));
          setLoading(false);
        }
      } catch (err) {
        if (mounted) {
          toast("获取智能体列表失败", "error");
          setLoading(false);
        }
      }
    };
    fetchAgents();
    return () => { mounted = false; };
  }, []);

  const filteredAgents = agents.filter(agent => {
    const matchSearch = agent.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
                       (agent.description || "").toLowerCase().includes(searchQuery.toLowerCase());
    const matchCategory = selectedCategoryId === 'all' || agent.categoryId === selectedCategoryId;
    return matchSearch && matchCategory;
  });

  const toggleSelect = (id: string) => {
    if (multiple) {
      setSelectedIds(prev => prev.includes(id) ? prev.filter(x => x !== id) : [...prev, id]);
    } else {
      setSelectedIds([id]);
    }
  };

  const handleConfirm = async () => {
    if (selectedIds.length === 0 || submitting) {
      return;
    }
    setSubmitting(true);
    try {
      for (const agentId of selectedIds) {
        await deviceService.bindAgent(device.id, agentId);
      }
      onSelect(selectedIds);
      toast(multiple ? "Bindings submitted" : "Binding submitted", "success");
      onClose();
    } catch (error) {
      const message = error instanceof Error && error.message ? error.message : "Binding failed";
      toast(message, "error");
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-md p-4"
    >
      <motion.div
        initial={{ opacity: 0, scale: 0.95, y: 20 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        exit={{ opacity: 0, scale: 0.95, y: 20 }}
        className="w-full max-w-3xl bg-white dark:bg-[#1e1e1e] border border-gray-200 dark:border-white/10 rounded-2xl shadow-2xl flex flex-col max-h-[85vh] overflow-hidden"
      >
        {/* Header */}
        <div className="flex justify-between items-center px-6 py-5 border-b border-gray-200 dark:border-white/5 bg-gray-50 dark:bg-[#252528] shrink-0">
          <div>
            <h2 className="text-xl font-bold text-gray-900 dark:text-white tracking-wide">分配智能体</h2>
            <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
              为设备 <span className="text-indigo-600 dark:text-indigo-400 font-mono bg-indigo-50 dark:bg-indigo-500/10 px-2 py-0.5 rounded">{device.name}</span> 选择托管核心
            </p>
          </div>
          <button
            onClick={onClose}
            className="w-8 h-8 flex items-center justify-center rounded-full bg-gray-200 dark:bg-white/5 text-gray-500 dark:text-gray-400 hover:bg-gray-300 dark:hover:bg-white/10 hover:text-gray-900 dark:hover:text-white transition-colors"
          >
            <X size={18} />
          </button>
        </div>

        {/* Top Filter Bar */}
        <div className="px-6 py-4 border-b border-gray-200 dark:border-white/5 bg-white dark:bg-[#202022] shrink-0 flex flex-col gap-4">
          <div className="relative">
            <Search className="absolute left-3.5 top-1/2 -translate-y-1/2 text-gray-400 dark:text-gray-500" size={18} />
            <input 
              type="text" 
              placeholder="搜索智能体名称或描述..." 
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-gray-50 dark:bg-[#18181a] border border-gray-200 dark:border-white/10 rounded-xl pl-11 pr-4 py-2.5 text-sm text-gray-900 dark:text-gray-200 outline-none focus:border-indigo-500 focus:bg-white dark:focus:bg-[#151515] transition-all shadow-inner"
            />
          </div>
          <div className="flex items-center gap-2 overflow-x-auto pb-1 custom-scrollbar">
            <Filter size={14} className="text-gray-400 dark:text-gray-500 mr-1" />
            {categories.map(cat => (
              <button
                key={cat.id}
                onClick={() => setSelectedCategoryId(cat.id)}
                className={cn(
                  "px-4 py-1.5 rounded-full text-xs font-semibold whitespace-nowrap transition-all border",
                  selectedCategoryId === cat.id
                    ? "bg-indigo-50 dark:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 border-indigo-200 dark:border-indigo-500/40 shadow-sm dark:shadow-[0_0_10px_rgba(99,102,241,0.2)]"
                    : "bg-gray-50 dark:bg-[#28282b] text-gray-500 dark:text-gray-400 border-gray-200 dark:border-white/5 hover:bg-gray-100 dark:hover:bg-[#303033] hover:text-gray-900 dark:hover:text-gray-200"
                )}
              >
                {cat.name}
              </button>
            ))}
          </div>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-6 bg-gray-50 dark:bg-[#1a1a1c]">
          {loading ? (
            <div className="flex flex-col flex-1 items-center justify-center h-full text-gray-500 space-y-4 py-10">
              <div className="w-8 h-8 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>
              <p className="text-sm">正在同步智能体网络...</p>
            </div>
          ) : filteredAgents.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-full text-gray-500 py-20">
              <Bot size={40} className="mb-4 text-gray-600" />
              <p className="text-sm">未找到匹配的智能体组件</p>
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {filteredAgents.map(agent => {
                const isSelected = selectedIds.includes(agent.id || '');
                return (
                  <div
                    key={agent.id}
                    onClick={() => toggleSelect(agent.id || '')}
                    className={cn(
                      "flex items-start gap-4 p-4 rounded-xl border cursor-pointer transition-all relative overflow-hidden group",
                      isSelected 
                        ? "bg-indigo-50 dark:bg-indigo-500/10 border-indigo-200 dark:border-indigo-500/50 shadow-sm dark:shadow-[0_0_15px_rgba(99,102,241,0.1)]" 
                        : "bg-white dark:bg-[#252528] border-gray-200 dark:border-white/5 hover:border-gray-300 dark:hover:border-white/20 hover:bg-gray-50 dark:hover:bg-[#2a2a2d]"
                    )}
                  >
                    {/* Active Background Glow */}
                    {isSelected && (
                       <div className="absolute top-0 right-0 w-24 h-24 bg-indigo-500/5 dark:bg-indigo-500/10 rounded-full blur-[30px] pointer-events-none"></div>
                    )}
                    
                    <div className={cn(
                      "w-12 h-12 rounded-xl flex items-center justify-center text-white shrink-0 shadow-sm dark:shadow-lg relative z-10", 
                      agent.color || 'bg-gray-400 dark:bg-gray-600'
                    )}>
                      <Bot size={24} />
                    </div>
                    
                    <div className="flex-1 min-w-0 relative z-10">
                      <div className="flex items-center justify-between mb-1">
                        <h4 className="text-sm font-bold text-gray-900 dark:text-gray-200 truncate pr-2">{agent.name}</h4>
                        <div className={cn(
                          "w-5 h-5 rounded-full flex items-center justify-center shrink-0 border-2 transition-all",
                          isSelected 
                            ? "bg-indigo-600 dark:bg-indigo-500 border-indigo-600 dark:border-indigo-500 shadow-sm dark:shadow-[0_0_10px_rgba(99,102,241,0.5)] text-white" 
                            : "border-gray-300 dark:border-gray-600 bg-transparent"
                        )}>
                          {isSelected && <Check size={12} strokeWidth={3} />}
                        </div>
                      </div>
                      <p className="text-xs text-gray-500 line-clamp-2 leading-relaxed">{agent.description}</p>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-gray-200 dark:border-white/5 bg-gray-50 dark:bg-[#252528] shrink-0 flex items-center justify-between">
          <div className="text-sm text-gray-500 dark:text-gray-400">
            已选择的智能体: <span className="text-indigo-600 dark:text-indigo-400 font-bold ml-1">{selectedIds.length}</span>
            {multiple ? ' (允许多选)' : ' (单节点独占)'}
          </div>
          <div className="flex gap-3">
            <button
              onClick={onClose}
              className="px-5 py-2.5 rounded-lg text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white bg-gray-200 dark:bg-white/5 hover:bg-gray-300 dark:hover:bg-white/10 transition-colors"
            >
              取消
            </button>
            <button
              onClick={handleConfirm}
              disabled={selectedIds.length === 0 || submitting}
              className="px-6 py-2.5 rounded-lg text-sm font-semibold text-white bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-lg shadow-indigo-500/20"
            >
              {submitting ? "Submitting..." : "确认编排"}
            </button>
          </div>
        </div>
      </motion.div>
    </motion.div>
  );
};
