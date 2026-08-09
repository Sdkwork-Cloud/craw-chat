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
import { KnowledgeBaseService, type KnowledgeBase } from "@sdkwork/knowledgebase-mobile-react-knowledge";
import { KnowledgeBaseSelectionModal } from "../components/KnowledgeBaseSelectionModal";
import { AgentAvatarUpload } from "../components/AgentAvatarUpload";
import { AgentFormFields } from "../components/AgentFormFields";

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
        <AgentFormFields
          name={name}
          setName={setName}
          prompt={prompt}
          setPrompt={setPrompt}
          greeting={greeting}
          setGreeting={setGreeting}
          selectedKb={selectedKb}
          setShowKbSelection={setShowKbSelection}
          voice={voice}
          setShowVoiceSelection={setShowVoiceSelection}
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
