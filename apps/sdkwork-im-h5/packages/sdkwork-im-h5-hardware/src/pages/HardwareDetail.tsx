import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router";
import { HardwareService } from "../services/HardwareService";
import { Hardware, Agent } from "../types";
import { cn, IconButton, showToast } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Cpu, Bot, Battery, Wifi, Trash2, Unlink, Check, Link } from "lucide-react";
import { useTranslation } from "react-i18next";

export const HardwareDetail: React.FC = () => {
  const { t } = useTranslation();
  
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  
  const [hardware, setHardware] = useState<Hardware | null>(null);
  const [agents, setAgents] = useState<Agent[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [showAgentModal, setShowAgentModal] = useState(false);

  useEffect(() => {
    loadData();
  }, [id]);

  const loadData = async () => {
    if (!id) return;
    setIsLoading(true);
    try {
      const [hw, tempAgents] = await Promise.all([
        HardwareService.getHardwareById(id),
        HardwareService.getAllAgents()
      ]);
      if (hw) setHardware(hw);
      setAgents(tempAgents);
    } catch {
      showToast(t('hardware.failedToLoad')); // Actually this is fetch detail fail, we can re-use or add new one. I'll use common message.
    } finally {
      setIsLoading(false);
    }
  };

  const handleUnbind = async () => {
    if (!id) return;
    try {
      await HardwareService.deleteHardware(id);
      showToast(t('hardware.unbindSuccess'));
      navigate(-1);
    } catch {
      showToast(t('common.error', { defaultValue: '失败' }));
    }
  };

  const handleAssociateAgent = async (agentId?: string) => {
    if (!id) return;
    try {
      const updated = await HardwareService.associateAgent(id, agentId);
      setHardware(updated);
      showToast(t('common.success', { defaultValue: '更新成功' }));
      setShowAgentModal(false);
    } catch {
      showToast(t('common.error', { defaultValue: '更新失败' }));
    }
  };

  if (isLoading) {
    return (
      <div className="flex flex-col h-full bg-bg-color">
         <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-bg-color">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
         </header>
         <div className="flex-1 flex items-center justify-center text-text-sub">{t('hardware.loading')}</div>
      </div>
    );
  }

  if (!hardware) {
    return (
      <div className="flex flex-col h-full bg-bg-color">
        <header className="h-[56px] px-4 flex items-center sticky top-0 z-10 pt-safe bg-bg-color">
            <IconButton icon={<ChevronLeft className="w-6 h-6 text-text-main" />} className="bg-transparent w-10 h-10 -ml-2" onClick={() => navigate(-1)} />
            <h1 className="text-[17px] font-semibold text-text-main ml-2">{t('hardware.detail.title')}</h1>
        </header>
        <div className="flex-1 flex items-center justify-center text-text-sub">{t('common.empty', { defaultValue: '为空' })}</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black">
      {/* Header */}
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-transparent">
        <div className="absolute left-1/2 -translate-x-1/2 font-semibold text-[17px] text-text-main">
          {t('hardware.detail.title')}
        </div>
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          className="bg-black/5 dark:bg-white/5 w-9 h-9 p-0 rounded-full"
          onClick={() => navigate(-1)}
        />
      </header>

      <div className="flex-1 overflow-y-auto pb-8">
        <div className="py-8 flex flex-col items-center">
            <div className="w-24 h-24 bg-white dark:bg-[#1C1C1E] rounded-[28px] shadow-sm flex items-center justify-center mb-4">
               <Cpu className="w-12 h-12 text-blue-500" />
            </div>
            <h2 className="text-[22px] font-bold text-text-main mb-2">{hardware.name}</h2>
            <div className="flex items-center gap-1.5 px-3 py-1 bg-white/50 dark:bg-white/5 rounded-full border border-black/5 dark:border-white/5">
                <span className={cn("w-2 h-2 rounded-full", hardware.status === 'online' ? "bg-emerald-500" : "bg-gray-400")} />
                <span className="text-[13px] text-text-sub">{hardware.status === 'online' ? t('hardware.detail.deviceNormal') : t('hardware.detail.offline')}</span>
            </div>
        </div>

        <div className="px-4 flex flex-col gap-4">
          {/* Status Panel */}
          <div className="bg-white dark:bg-[#1C1C1E] rounded-2xl flex divide-x divide-black/5 dark:divide-white/5 shadow-[0_2px_10px_rgba(0,0,0,0.02)]">
             <div className="flex-1 p-4 flex flex-col items-center gap-1.5">
                <Wifi className="w-5 h-5 text-text-sub" />
                <span className="text-[13px] text-text-sub">{t('hardware.detail.networkStatus')}</span>
                <span className="text-[14px] font-medium text-text-main">{t('hardware.detail.good')}</span>
             </div>
             <div className="flex-1 p-4 flex flex-col items-center gap-1.5">
                <Battery className="w-5 h-5 text-emerald-500" />
                <span className="text-[13px] text-text-sub">{t('hardware.detail.batteryLevel')}</span>
                <span className="text-[14px] font-medium text-text-main">92%</span>
             </div>
          </div>

          {/* Agent Association */}
          <div className="bg-white dark:bg-[#1C1C1E] rounded-2xl p-1 shadow-[0_2px_10px_rgba(0,0,0,0.02)]">
             <div className="px-3 py-3 border-b border-black/5 dark:border-white/5 last:border-0 flex items-center justify-between">
                <div className="flex items-center gap-3">
                   <div className="w-8 h-8 rounded-full bg-purple-500/10 flex items-center justify-center">
                     <Bot className="w-4 h-4 text-purple-500" />
                   </div>
                   <div className="flex flex-col">
                     <span className="text-[15px] font-medium text-text-main">{t('hardware.detail.agentTitle')}</span>
                     <span className="text-[12px] text-text-sub mt-0.5">{hardware.agentName ? t('hardware.detail.agentBinded', { agentName: hardware.agentName }) : t('hardware.detail.agentUnbinded')}</span>
                   </div>
                </div>
                <button 
                  className={cn("px-4 py-1.5 rounded-full text-[13px] font-medium transition-colors", hardware.agentName ? "bg-black/5 dark:bg-white/10 text-text-main" : "bg-purple-500 text-white")}
                  onClick={() => setShowAgentModal(true)}
                >
                  {hardware.agentName ? t('hardware.detail.replace') : t('hardware.detail.configAgent')}
                </button>
             </div>
          </div>
          
          <div className="bg-white dark:bg-[#1C1C1E] rounded-2xl p-1 shadow-[0_2px_10px_rgba(0,0,0,0.02)] mt-4">
             <div 
               className="px-3 py-3.5 flex items-center justify-between cursor-pointer active:bg-black/5 dark:active:bg-white/5 rounded-xl transition-colors"
               onClick={handleUnbind}
             >
                <div className="flex items-center gap-3">
                   <Trash2 className="w-5 h-5 text-[#FA5151]" />
                   <span className="text-[15px] font-medium text-[#FA5151]">{t('hardware.detail.unbindDelete')}</span>
                </div>
             </div>
          </div>
        </div>
      </div>

      {/* Agent Modal */}
      {showAgentModal && (
        <div className="fixed inset-0 z-50 flex items-end justify-center">
           <div className="absolute inset-0 bg-black/40 backdrop-blur-sm" onClick={() => setShowAgentModal(false)} />
           <div className="bg-bg-color w-full rounded-t-[24px] overflow-hidden relative flex flex-col pb-safe max-h-[80vh]">
              <div className="w-10 h-1 bg-black/20 dark:bg-white/20 rounded-full mx-auto mt-3 mb-1" />
              <div className="px-4 py-3 flex items-center justify-between border-b border-border-color">
                 <h3 className="text-[17px] font-semibold text-text-main">{t('hardware.detail.selectAgent')}</h3>
                 <span className="text-[15px] text-text-sub cursor-pointer" onClick={() => setShowAgentModal(false)}>{t('hardware.detail.cancel')}</span>
              </div>
              <div className="flex-1 overflow-y-auto px-4 py-2 flex flex-col gap-2">
                 <div 
                   className="p-4 rounded-2xl border border-border-color flex items-center justify-between cursor-pointer active:bg-chat-active-bg"
                   onClick={() => handleAssociateAgent(undefined)}
                 >
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 rounded-full bg-gray-100 dark:bg-white/10 flex items-center justify-center">
                        <Unlink className="w-5 h-5 text-text-sub" />
                      </div>
                      <span className="text-[16px] text-text-main font-medium">{t('hardware.detail.noneAgent')}</span>
                    </div>
                    {!hardware.agentId && <Check className="w-5 h-5 text-purple-500" />}
                 </div>

                 {agents.map(agent => (
                   <div 
                     key={agent.id}
                     className={cn(
                       "p-4 rounded-2xl border flex items-center justify-between cursor-pointer transition-colors",
                       hardware.agentId === agent.id 
                         ? "border-purple-500 bg-purple-500/5" 
                         : "border-border-color active:bg-chat-active-bg"
                     )}
                     onClick={() => handleAssociateAgent(agent.id)}
                   >
                     <div className="flex items-center gap-3">
                        <div className="w-10 h-10 rounded-full bg-gradient-to-br from-purple-500 to-indigo-500 flex items-center justify-center shadow-md shadow-purple-500/20">
                          <Bot className="w-5 h-5 text-white" />
                        </div>
                        <div className="flex flex-col gap-1">
                          <span className="text-[16px] text-text-main font-medium">{agent.name}</span>
                          <span className="text-[12px] text-text-sub">{t('hardware.detail.capability', { capabilities: agent.capabilities.join(", ") })}</span>
                        </div>
                     </div>
                     {hardware.agentId === agent.id && <Check className="w-5 h-5 text-purple-500" />}
                   </div>
                 ))}
              </div>
           </div>
        </div>
      )}
    </div>
  );
};
