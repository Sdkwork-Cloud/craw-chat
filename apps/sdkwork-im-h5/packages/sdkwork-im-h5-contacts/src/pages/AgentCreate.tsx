import React, { useState, useRef, useEffect } from "react";
import { useNavigate, useParams } from "react-router";
import {
  ChevronLeft,
  Camera,
  Bot,
  Sparkles,
  MessageSquare,
  Settings2,
  ChevronRight,
  FileText,
  UploadCloud,
  Mic,
  Globe,
  Image as ImageIcon,
  Search,
  X,
  PlusCircle,
  Play,
  Square,
  Trash2,
} from "lucide-react";
import { IconButton, cn, showToast, showPrompt, showConfirm, ActionSheet } from "@sdkwork/im-h5-commons";
import { motion, AnimatePresence } from "motion/react";
import { AgentService } from "../services/AgentService";
import {
  VoiceService,
  VoiceSelectionPage,
  type VoiceCategory,
} from "@sdkwork/im-h5-commons";
import { CreateVoice } from "@sdkwork/im-h5-user";
import { useTranslation } from "react-i18next";
import { KnowledgeBaseService, type KnowledgeBase } from "@sdkwork/im-h5-knowledge";
import { KnowledgeBaseSelectionModal } from "../components/KnowledgeBaseSelectionModal";
import { AdvancedSettingsPanel } from "../components/AdvancedSettingsPanel";
import { AgentAvatarUpload } from "../components/AgentAvatarUpload";

export const AgentCreate: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const { id } = useParams<{ id?: string }>();
  const isEdit = !!id;

  const [name, setName] = useState("");
  const [prompt, setPrompt] = useState("");
  const [greeting, setGreeting] = useState("");

  // Advanced Settings State
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [temperature, setTemperature] = useState(0.7);
  const [maxTokens, setMaxTokens] = useState(2048);
  const [voice, setVoice] = useState({ id: "female1", label: t('contacts.voice_female1') });
  const [tools, setTools] = useState({ webSearch: true, imageGen: false });
  const [isCreating, setIsCreating] = useState(false);
  const [showVoiceSelection, setShowVoiceSelection] = useState(false);
  const [avatarPreview, setAvatarPreview] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const [knowledgeBases, setKnowledgeBases] = useState<KnowledgeBase[]>([]);
  const [selectedKb, setSelectedKb] = useState<KnowledgeBase | null>(null);
  const [showKbSelection, setShowKbSelection] = useState(false);

  useEffect(() => {
    KnowledgeBaseService.getKnowledgeBases().then(data => {
      setKnowledgeBases(data);
    });
  }, []);

  useEffect(() => {
    if (isEdit && id) {
      AgentService.getAgentById(id).then(agent => {
        if (agent) {
          setName(agent.name);
          setPrompt(agent.prompt || "");
          setGreeting(""); // greeting is not stored in Agent, but can be added if needed
          if (agent.kbId) {
             KnowledgeBaseService.getKnowledgeBase(agent.kbId).then(kb => {
                if (kb) setSelectedKb(kb);
             });
          }
          if (agent.avatar) setAvatarPreview(agent.avatar);
        }
      });
    }
  }, [isEdit, id]);

  const handleAvatarSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
  const file = e.target.files?.[0];
    if (file) {
      const url = URL.createObjectURL(file);
      setAvatarPreview(url);
    }
  };

  const handleCreate = async () => {
    if (!name.trim() || !prompt.trim() || isCreating) return;
    setIsCreating(true);
    try {
      if (isEdit && id) {
        await AgentService.updateAgent(id, {
          name: name.trim(),
          prompt: prompt.trim(),
          kbId: selectedKb?.id,
          avatar: avatarPreview,
        });
        showToast(t('contacts.save_success'));
        navigate(-1);
      } else {
        const newAgent = await AgentService.createAgent({
          name: name.trim(),
          prompt: prompt.trim(),
          kbId: selectedKb?.id,
          avatar: avatarPreview,
        });
        showToast(t('contacts.create_success', 'Agent created'));
        navigate(-1);
      }
    } catch (error) {
      console.error(error);
      showToast(isEdit ? t('contacts.save_failed') : t('contacts.create_failed'));
      setIsCreating(false);
    }
  };

  const handleDelete = async () => {
    const isConfirmed = await showConfirm(t('contacts.delete_confirm'));
    if (isConfirmed) {
      showToast(t('contacts.deleted'));
      navigate(-1);
    }
  };

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto w-full">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{isEdit ? t('contacts.edit_agent') : t('contacts.create_agent')}</h2>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-3 gap-2">
          {isEdit && (
             <IconButton
                icon={<Trash2 className="w-5 h-5 text-red-500" />}
                onClick={handleDelete}
             />
          )}
          <button
            onClick={handleCreate}
            disabled={!name.trim() || !prompt.trim() || isCreating}
            className={cn(
              "px-3 py-1.5 rounded-md text-[14px] font-medium transition-colors",
              name.trim() && prompt.trim() && !isCreating
                ? "bg-primary-blue text-white active:bg-blue-600"
                : "bg-black/5 dark:bg-white/5 text-text-sub cursor-not-allowed",
            )}
          >
            {isCreating ? (isEdit ? t('contacts.saving') : t('contacts.creating')) : (isEdit ? t('contacts.save') : t('contacts.complete'))}
          </button>
        </div>
      </header>

      <div className="flex flex-col px-4 py-6 gap-6 pb-[84px]">
        {/* Avatar Upload */}
        <AgentAvatarUpload
          avatarPreview={avatarPreview}
          onAvatarSelect={handleAvatarSelect}
        />

        {/* Form Fields */}
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

          {/* Voice Cell inside normal flow (before advanced) */}
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
      </div>

      {/* Voice Selection Fullscreen Overlay */}
      {showVoiceSelection && (
        <VoiceSelectionPage
          currentVoiceId={voice.id}
          onSelect={(v) => {
            setVoice(v);
            setShowVoiceSelection(false);
          }}
          onClose={() => setShowVoiceSelection(false)}
          renderCreateVoice={(onCloseAndRefresh) => (
            <CreateVoice onClose={onCloseAndRefresh} />
          )}
        />
      )}

      {/* KB Selection Modal */}
      <KnowledgeBaseSelectionModal
        show={showKbSelection}
        onClose={() => setShowKbSelection(false)}
        knowledgeBases={knowledgeBases}
        selectedKb={selectedKb}
        onSelect={setSelectedKb}
      />
    </div>
  );
};
