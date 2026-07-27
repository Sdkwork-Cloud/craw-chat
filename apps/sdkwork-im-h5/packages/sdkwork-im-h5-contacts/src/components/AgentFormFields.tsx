import React from "react";
import { Sparkles, MessageSquare, FileText, UploadCloud, Mic, ChevronRight } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { KnowledgeBase } from "@sdkwork/im-h5-knowledge";
import { AdvancedSettingsPanel } from "./AdvancedSettingsPanel";

export interface AgentFormFieldsProps {
  name: string;
  setName: (val: string) => void;
  prompt: string;
  setPrompt: (val: string) => void;
  greeting: string;
  setGreeting: (val: string) => void;
  selectedKb: KnowledgeBase | null;
  setShowKbSelection: (val: boolean) => void;
  voice: { id: string; label: string };
  setShowVoiceSelection: (val: boolean) => void;
  showAdvanced: boolean;
  setShowAdvanced: (val: boolean) => void;
  tools: { webSearch: boolean; imageGen: boolean };
  setTools: React.Dispatch<React.SetStateAction<{ webSearch: boolean; imageGen: boolean }>>;
  temperature: number;
  setTemperature: (val: number) => void;
  maxTokens: number;
  setMaxTokens: (val: number) => void;
}

export const AgentFormFields: React.FC<AgentFormFieldsProps> = ({
  name,
  setName,
  prompt,
  setPrompt,
  greeting,
  setGreeting,
  selectedKb,
  setShowKbSelection,
  voice,
  setShowVoiceSelection,
  showAdvanced,
  setShowAdvanced,
  tools,
  setTools,
  temperature,
  setTemperature,
  maxTokens,
  setMaxTokens,
}) => {
  const { t } = useTranslation();

  return (
    <div className="flex flex-col gap-4">
      {/* Name */}
      <div className="flex flex-col gap-2">
        <label className="text-[14px] font-medium text-text-main ml-1">
          {t('contacts.agent_name')}
        </label>
        <div className="bg-chat-other-bg rounded-xl px-4 py-3 border border-border-color focus-within:border-primary-blue transition-colors">
          <input
            type="text"
            placeholder={t('contacts.agent_name_placeholder')}
            className="w-full bg-transparent text-[16px] text-text-main focus:outline-none placeholder:text-text-sub"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
      </div>

      {/* Prompt/Persona */}
      <div className="flex flex-col gap-2">
        <div className="flex items-center justify-between ml-1">
          <label className="text-[14px] font-medium text-text-main flex items-center gap-1.5">
            <Sparkles className="w-4 h-4 text-primary-blue" />
            {t('contacts.agent_prompt_label')}
          </label>
          <span className="text-[12px] text-text-sub">
            {prompt.length}/2000
          </span>
        </div>
        <div className="bg-chat-other-bg rounded-xl px-4 py-3 border border-border-color focus-within:border-primary-blue transition-colors">
          <textarea
            placeholder={t('contacts.agent_prompt_placeholder')}
            className="w-full bg-transparent text-[16px] text-text-main focus:outline-none placeholder:text-text-sub resize-none min-h-[120px]"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
          />
        </div>
      </div>

      {/* Greeting */}
      <div className="flex flex-col gap-2">
        <label className="text-[14px] font-medium text-text-main flex items-center gap-1.5 ml-1">
          <MessageSquare className="w-4 h-4 text-text-sub" />
          {t('contacts.agent_greeting')}
        </label>
        <div className="bg-chat-other-bg rounded-xl px-4 py-3 border border-border-color focus-within:border-primary-blue transition-colors">
          <input
            type="text"
            placeholder={t('contacts.agent_greeting_placeholder')}
            className="w-full bg-transparent text-[16px] text-text-main focus:outline-none placeholder:text-text-sub"
            value={greeting}
            onChange={(e) => setGreeting(e.target.value)}
          />
        </div>
      </div>

      {/* Knowledge Base */}
      <div className="flex flex-col gap-2">
        <label className="text-[14px] font-medium text-text-main flex items-center gap-1.5 ml-1">
          <FileText className="w-4 h-4 text-primary-blue" />
          {t('contacts.agent_knowledge')}
        </label>
        <div 
          onClick={() => setShowKbSelection(true)}
          className="bg-chat-other-bg rounded-xl px-4 py-5 border border-border-color border-dashed flex flex-col items-center justify-center gap-3 cursor-pointer active:bg-active-bg transition-colors"
        >
          {selectedKb ? (
            <>
              <div 
                className="w-12 h-12 rounded-full flex items-center justify-center text-2xl shadow-inner"
                style={{ 
                  backgroundColor: selectedKb.color ? `${selectedKb.color}1A` : 'rgba(0, 102, 255, 0.1)', 
                  color: selectedKb.color || '#0066FF'
                }}
              >
                {selectedKb.icon || "📚"}
              </div>
              <div className="flex flex-col items-center gap-1">
                <span className="text-[15px] font-medium text-text-main">
                  {selectedKb.name}
                </span>
                <span className="text-[12px] text-text-sub text-center">
                  Tap to change knowledge base
                </span>
              </div>
            </>
          ) : (
            <>
              <div className="w-12 h-12 rounded-full bg-primary-blue/10 flex items-center justify-center">
                <UploadCloud className="w-6 h-6 text-primary-blue" />
              </div>
              <div className="flex flex-col items-center gap-1">
                <span className="text-[15px] font-medium text-text-main">
                  Select Knowledge Base
                </span>
                <span className="text-[12px] text-text-sub text-center leading-relaxed whitespace-pre-line">
                  Let the agent answer based on your exclusive data
                </span>
              </div>
            </>
          )}
        </div>
      </div>

      {/* Voice Cell inside normal flow */}
      <div className="flex flex-col gap-2 mt-2">
        <div
          onClick={() => setShowVoiceSelection(true)}
          className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg border border-border-color rounded-xl active:bg-active-bg transition-colors cursor-pointer"
        >
          <div className="flex flex-col gap-1">
            <div className="flex items-center gap-2">
              <Mic className="w-5 h-5 text-primary-blue" />
              <span className="text-[16px] text-text-main font-medium">
                {t('contacts.config_voice')}
              </span>
            </div>
            <span className="text-[12px] text-text-sub">
              {t('contacts.config_voice_desc')}
            </span>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-[15px] text-text-main">
              {voice.label}
            </span>
            <ChevronRight className="w-5 h-5 opacity-50 text-text-sub" />
          </div>
        </div>
      </div>

      {/* Advanced Settings */}
      <AdvancedSettingsPanel
        showAdvanced={showAdvanced}
        setShowAdvanced={setShowAdvanced}
        tools={tools}
        setTools={setTools}
        temperature={temperature}
        setTemperature={setTemperature}
        maxTokens={maxTokens}
        setMaxTokens={setMaxTokens}
      />
    </div>
  );
};
