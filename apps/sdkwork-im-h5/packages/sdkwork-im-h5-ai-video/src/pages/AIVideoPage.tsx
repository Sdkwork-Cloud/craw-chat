import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  History,
  Download,
} from "lucide-react";
import { IconButton, showToast, ModelSelectionPage, ModelVendor } from "@sdkwork/im-h5-commons";
import {
  AIVideoService,
  VideoTask,
  AIVideoOptions,
} from "../services/AIVideoService";
import { motion, AnimatePresence } from "motion/react";
import { cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";
import { AIVideoSettings } from "../components/AIVideoSettings";
import { AIVideoHistory } from "../components/AIVideoHistory";
import { AIVideoPreviewCard } from "../components/AIVideoPreviewCard";

const VIDEO_VENDORS: ModelVendor[] = [
  {
    id: "runway",
    name: "Runway ML",
    models: [
      { id: "gen3-alpha", name: "Gen-3 Alpha", tags: ["SOTA极高真实度", "超快生成"] },
      { id: "gen2", name: "Gen-2", tags: ["稳定", "支持笔刷控制"] },
    ],
  },
  {
    id: "kling",
    name: "可灵 (Kling AI)",
    models: [
      { id: "kling-1.5", name: "Kling 1.5 Pro", tags: ["支持最长3分钟", "1080P", "物理规律强"] },
      { id: "kling-standard", name: "Kling 1.0", tags: ["快速生成"] },
    ]
  },
  {
    id: "luma",
    name: "Luma AI",
    models: [
      { id: "dream-machine", name: "Dream Machine", tags: ["免费", "渲染极快", "写实"] },
    ],
  },
  {
    id: "sora",
    name: "OpenAI",
    models: [
      { id: "sora", name: "Sora", tags: ["暂未全面开放", "物理引擎级"] },
    ],
  },
];

export const AIVideoPage: React.FC = () => {
  
  
const { t } = useTranslation('ai_video');
  const navigate = useNavigate();
  const [prompt, setPrompt] = useState("");
  const [style, setStyle] = useState("Cinematic");
  const [aspectRatio, setAspectRatio] =
    useState<AIVideoOptions["aspectRatio"]>("16:9");

  const [showModelSelection, setShowModelSelection] = useState(false);
  const [selectedModelId, setSelectedModelId] = useState("gen3-alpha");
  const [selectedModelName, setSelectedModelName] = useState("Gen-3 Alpha");
  const [selectedVendorId, setSelectedVendorId] = useState("runway");
  const [showAdvanced, setShowAdvanced] = useState(false);

  // Advanced Params
  const [cameraMotion, setCameraMotion] = useState("none");
  const [videoLength, setVideoLength] = useState(5);
  const [fps, setFps] = useState(30);

  const [isGenerating, setIsGenerating] = useState(false);
  const [currentProgress, setCurrentProgress] = useState(0);
  const [currentTask, setCurrentTask] = useState<VideoTask | null>(null);
  const [history, setHistory] = useState<VideoTask[]>([]);

  const styles = ["Cinematic", "Anime", "3D Animation", "Drone", "Time Lapse"];
  const ratios: AIVideoOptions["aspectRatio"][] = ["16:9", "9:16", "1:1"];

  useEffect(() => {
    AIVideoService.getHistory().then(setHistory);
  }, []);

  const handleGenerate = async () => {
    if (!prompt.trim()) return showToast(t('settings.generate_error'));
    setIsGenerating(true);
    setCurrentProgress(0);

    const options: AIVideoOptions = { prompt, style, aspectRatio };
    setCurrentTask({
      id: "temp",
      options,
      status: "generating",
      progress: 0,
      createdAt: Date.now(),
      estimatedTimeSec: 15,
    });

    try {
      const task = await AIVideoService.generateVideo(options, (p) => {
        setCurrentProgress(p);
      });
      setCurrentTask(task);
      setHistory((prev) => [task, ...prev.filter((t) => t.id !== "temp")]);
      showToast(t('result.generate_success'));
    } catch (err) {
      showToast(t('result.generate_failed'));
      setCurrentTask(null);
    } finally {
      setIsGenerating(false);
      setCurrentProgress(0);
    }
  };

  const downloadVideo = async (url?: string) => {
    if (!url) return;
    try {
      const resp = await fetch(url);
      const blob = await resp.blob();
      const objUrl = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = objUrl;
      a.download = `ai_video_${Date.now()}.mp4`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      window.URL.revokeObjectURL(objUrl);
      showToast(t('result.saving'));
    } catch (e) {
      console.warn("Fetch failed, opening in new tab", e);
      window.open(url, "_blank");
      showToast(t('result.save_browser'));
    }
  };

  const handleDelete = (e: React.MouseEvent, id: string) => {
  e.stopPropagation();
    AIVideoService.deleteFromHistory(id);
    setHistory((prev) => prev.filter((t) => t.id !== id));
    if (currentTask?.id === id) {
      setCurrentTask(null);
      setPrompt("");
    }
    showToast(t('result.delete_success'));
  };

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black pt-safe">
      <header className="h-[44px] flex items-center justify-between px-2 shrink-0 bg-bg-color border-b border-border-color">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <span className="font-medium text-[17px] text-text-main">
          {t('title')}
        </span>
        <IconButton
          icon={<History className="w-5 h-5 text-text-main" />}
          onClick={() =>
            document
              .getElementById("history-section")
              ?.scrollIntoView({ behavior: "smooth" })
          }
        />
      </header>

      <div className="flex-1 overflow-y-auto flex flex-col gap-4 pb-safe">
        <AIVideoSettings
          t={t}
          prompt={prompt}
          setPrompt={setPrompt}
          style={style}
          setStyle={setStyle}
          aspectRatio={aspectRatio}
          setAspectRatio={setAspectRatio}
          styles={styles}
          ratios={ratios}
          showAdvanced={showAdvanced}
          setShowAdvanced={setShowAdvanced}
          cameraMotion={cameraMotion}
          setCameraMotion={setCameraMotion}
          videoLength={videoLength}
          setVideoLength={setVideoLength}
          fps={fps}
          setFps={setFps}
          isGenerating={isGenerating}
          handleGenerate={handleGenerate}
          selectedModelName={selectedModelName}
          selectedVendorId={selectedVendorId}
          onModelSelectClick={() => setShowModelSelection(true)}
        />

        <div className="px-4 pb-6">
          <AnimatePresence>
            {currentTask && (
              <AIVideoPreviewCard
                t={t}
                currentTask={currentTask}
                isGenerating={isGenerating}
                currentProgress={currentProgress}
                onDownload={downloadVideo}
              />
            )}
          </AnimatePresence>

          {!isGenerating && (
            <AIVideoHistory
              t={t}
              history={history}
              currentTask={currentTask}
              onSelect={(item) => {
                setPrompt(item.options.prompt);
                setStyle(item.options.style);
                setAspectRatio(item.options.aspectRatio);
                setCurrentTask(item);
                document
                  .querySelector(".pb-safe")
                  ?.scrollTo({ top: 0, behavior: "smooth" });
              }}
              onDelete={handleDelete}
              onSuggestionClick={setPrompt}
            />
          )}
        </div>
      </div>

      {showModelSelection && (
        <ModelSelectionPage
          title={t('settings.model_selection')}
          currentModelId={selectedModelId}
          vendors={VIDEO_VENDORS}
          onSelect={(model, vendor) => {
            setSelectedModelId(model.id);
            setSelectedModelName(model.name);
            setSelectedVendorId(vendor.id);
            setShowModelSelection(false);
          }}
          onClose={() => setShowModelSelection(false)}
        />
      )}
    </div>
  );
};

