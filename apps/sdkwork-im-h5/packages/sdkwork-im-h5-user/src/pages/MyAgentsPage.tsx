import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { Plus } from "lucide-react";
import { showToast, ActionSheet } from "@sdkwork/im-h5-commons";
import { PageLayout } from "../components/PageLayout";
import { AgentService, type Agent } from "@sdkwork/im-h5-contacts";

export const MyAgentsPage = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [agents, setAgents] = useState<Agent[]>([]);
  const [actionSheetAgent, setActionSheetAgent] = useState<Agent | null>(null);
  const [isLongPressed, setIsLongPressed] = useState(false);

  const loadAgents = () => {
  AgentService.getMyAgents().then(data => {
      setAgents(data);
    });
  };

  useEffect(() => {
    loadAgents();
  }, []);

  const startLongPress = (agent: Agent) => {
  const handlePressStart = () => {
  setIsLongPressed(false);
      (window as any).longPressTimeout = setTimeout(() => {
        setIsLongPressed(true);
        setActionSheetAgent(agent);
      }, 500);
    };

    const handlePressEnd = () => {
  clearTimeout((window as any).longPressTimeout);
    };

    return {
      onPointerDown: handlePressStart,
      onPointerUp: handlePressEnd,
      onPointerLeave: () => {
        handlePressEnd();
        setIsLongPressed(false);
      },
      onContextMenu: (e: React.MouseEvent) => {
        e.preventDefault();
        handlePressStart();
        setIsLongPressed(true);
        setActionSheetAgent(agent);
        handlePressEnd();
      }
    };
  };

  const handleActionSheetSelect = async (action: string) => {
    if (!actionSheetAgent) return;
    if (action === 'edit') {
       navigate(`/agent/edit/${actionSheetAgent.id}`);
    } else if (action === 'delete') {
       await AgentService.deleteAgent(actionSheetAgent.id);
       loadAgents();
       showToast(t('user.auto_fn_16b31b6', '已删除'));
    } else if (action === 'share') {
       showToast(t('user.auto_fn_n72a4c47', '已分享智能体'));
    }
    setActionSheetAgent(null);
  };

  return (
    <PageLayout 
      title={t('user.auto_prop_n62a0a583', '我的智能体')}
      rightElement={
        <div 
          className="w-10 h-10 flex items-center justify-center cursor-pointer active:opacity-70 transition-opacity"
          onClick={() => navigate("/agent/create")}
        >
          <Plus className="w-6 h-6 text-text-main" />
        </div>
      }
    >
      {agents.length > 0 ? (
        <div className="flex-1 overflow-y-auto w-full pb-12 bg-chat-other-bg relative">
          <div className="flex flex-col gap-[2px]">
            {agents.map((agent) => (
              <div
                key={agent.id}
                className="bg-bg-color px-4 py-3.5 flex items-center gap-3 active:bg-active-bg transition-colors cursor-pointer select-none touch-callout-none"
                onClick={() => {
                  if (isLongPressed) {
                    setIsLongPressed(false);
                    return;
                  }
                  navigate(`/chat/${agent.id}`);
                }}
                {...startLongPress(agent)}
              >
                <div className="w-12 h-12 bg-gray-100 dark:bg-[#1A1A1A] rounded-xl flex items-center justify-center border border-border-color pointer-events-none overflow-hidden">
                  {agent.avatar ? (
                     <img src={agent.avatar} alt="avatar" className="w-full h-full object-cover" />
                  ) : (
                     <span className="text-[24px]">🤖</span>
                  )}
                </div>
                <div className="flex-1 min-w-0 flex flex-col justify-center pointer-events-none">
                  <div className="flex items-center justify-between mb-0.5">
                    <span className="font-medium text-text-main text-[16px] truncate">
                      {agent.name}
                    </span>
                    <span className="text-[11px] text-primary-blue bg-primary-blue/10 px-1.5 py-0.5 rounded border border-primary-blue/20">{t('user.auto_101a50', '自建')}</span>
                  </div>
                  <p className="text-[13px] text-text-sub truncate">{agent.desc || "暂无描述"}</p>
                </div>
              </div>
            ))}
          </div>

          {actionSheetAgent && (
            <ActionSheet
              isOpen={true}
              title={`${actionSheetAgent.name} - 操作`}
              options={[
                { label: '分享智能体', onClick: () => handleActionSheetSelect('share') },
                { label: '修改配置', onClick: () => handleActionSheetSelect('edit') },
                { label: '删除智能体', danger: true, onClick: () => handleActionSheetSelect('delete') }
              ]}
              onClose={() => setActionSheetAgent(null)}
            />
          )}
        </div>
      ) : (
        <div className="flex flex-col items-center justify-center flex-1 py-20 w-full h-full">
          <div className="w-20 h-20 bg-chat-other-bg rounded-[24px] flex items-center justify-center mb-6 shadow-sm border border-border-color">
            <span className="text-4xl">🤖</span>
          </div>
          <h3 className="text-lg font-bold text-text-main mb-2">{t('user.auto_dae7732', '打造专属 AI 助手')}</h3>
          <p className="text-[14px] text-text-sub mb-8 max-w-[200px] text-center leading-relaxed">{t('user.auto_73b43c07', '定制懂你的 AI 智能体，提升工作生活效率')}</p>
          <button
            onClick={() => navigate("/agent/create")}
            className="px-8 h-12 bg-primary-blue text-white rounded-full font-medium active:scale-95 transition-transform shadow-lg shadow-blue-500/30 flex items-center justify-center"
          >{t('user.auto_39152047', '立即创建')}</button>
        </div>
      )}
    </PageLayout>
  );
};
