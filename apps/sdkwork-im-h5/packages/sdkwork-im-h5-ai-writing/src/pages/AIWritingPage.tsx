import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  History,
} from "lucide-react";
import { IconButton, showToast } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";
import {
  AIWritingService,
  WritingTask,
  AIWritingOptions,
} from "../services/AIWritingService";
import { AIWritingSettingsPanel } from "../components/AIWritingSettingsPanel";
import { AIWritingHistoryPanel } from "../components/AIWritingHistoryPanel";

export const AIWritingPage: React.FC = () => {
  
  
const { t } = useTranslation('ai_writing');
  const navigate = useNavigate();
  const [topic, setTopic] = useState("");
  const [style, setStyle] = useState("Professional");
  const [length, setLength] = useState<AIWritingOptions["length"]>("medium");
  const [language, setLanguage] =
    useState<AIWritingOptions["language"]>("Chinese");

  const [isGenerating, setIsGenerating] = useState(false);
  const [currentTask, setCurrentTask] = useState<WritingTask | null>(null);
  const [history, setHistory] = useState<WritingTask[]>([]);
  const [copied, setCopied] = useState(false);
  const [realtimeContent, setRealtimeContent] = useState("");

  const styles = [
    "Professional",
    "Casual",
    "Creative",
    "Academic",
    "Humorous",
    "Persuasive",
  ];
  const lengths: AIWritingOptions["length"][] = ["short", "medium", "long"];
  const languages: AIWritingOptions["language"][] = ["Chinese", "English"];

  useEffect(() => {
    AIWritingService.getHistory().then(setHistory);
  }, []);

  const handleGenerate = async () => {
    if (!topic.trim()) return showToast(t('settings.generate_error'));
    setIsGenerating(true);
    setRealtimeContent("");

    const options: AIWritingOptions = { topic, style, length, language };
    setCurrentTask({
      id: "temp",
      options,
      status: "generating",
      createdAt: Date.now(),
    });
    setCopied(false);

    try {
      const task = await AIWritingService.generateArticle(options, (chunk) => {
        setRealtimeContent(chunk);
      });
      setCurrentTask(task);
      setRealtimeContent("");
      setHistory((prev) => [task, ...prev.filter((t) => t.id !== "temp")]);
      showToast(t('settings.generate_success'));
    } catch (err) {
      showToast(t('settings.generate_failed'));
      setCurrentTask(null);
    } finally {
      setIsGenerating(false);
    }
  };

  const handleCopy = (content?: string) => {
  if (content) {
      navigator.clipboard.writeText(content);
      setCopied(true);
      showToast(t('result.copy_success'));
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleDelete = (e: React.MouseEvent, id: string) => {
  e.stopPropagation();
    AIWritingService.deleteFromHistory(id);
    setHistory((prev) => prev.filter((t) => t.id !== id));
    if (currentTask?.id === id) {
      setCurrentTask(null);
      setTopic("");
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black pt-safe">
      <header className="h-[44px] flex items-center justify-between px-2 shrink-0 bg-bg-color border-b border-border-color">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <span className="font-medium text-[17px] text-text-main">
          {t('header_title')}
        </span>
        <IconButton
          icon={<History className="w-5 h-5 text-text-main" />}
          onClick={() => {
            if (history.length === 0) {
              showToast(t('history_not_found'));
            } else {
              document.getElementById("history-section")?.scrollIntoView({ behavior: "smooth" });
            }
          }}
        />
      </header>

      <div className="flex-1 overflow-y-auto flex flex-col gap-4 relative pb-safe">
        <AIWritingSettingsPanel
          t={t}
          topic={topic}
          setTopic={setTopic}
          style={style}
          setStyle={setStyle}
          length={length}
          setLength={setLength}
          language={language}
          setLanguage={setLanguage}
          styles={styles}
          lengths={lengths}
          languages={languages}
          isGenerating={isGenerating}
          handleGenerate={handleGenerate}
        />

        <AIWritingHistoryPanel
          t={t}
          currentTask={currentTask}
          history={history}
          isGenerating={isGenerating}
          realtimeContent={realtimeContent}
          copied={copied}
          handleGenerate={handleGenerate}
          handleCopy={handleCopy}
          handleDelete={handleDelete}
          setTopic={setTopic}
          setStyle={setStyle}
          setLength={setLength}
          setLanguage={setLanguage}
          setCurrentTask={setCurrentTask}
          setRealtimeContent={setRealtimeContent}
        />
      </div>
    </div>
  );
};
