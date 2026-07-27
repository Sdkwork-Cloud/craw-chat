import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  History,
} from "lucide-react";
import { IconButton, showToast, ModelSelectionPage, ModelVendor } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";
import {
  AIImageService,
  ImageTask,
  AIImageOptions,
} from "../services/AIImageService";
import { AIImageSettingsPanel } from "../components/AIImageSettingsPanel";
import { AIImageHistoryPanel } from "../components/AIImageHistoryPanel";

const IMAGE_VENDORS: ModelVendor[] = [
  {
    id: "midjourney",
    name: "Midjourney",
    models: [
      { id: "midjourney-v6", name: "Midjourney v6", tags: ["Photorealistic", "Arts"] },
      { id: "niji-6", name: "Niji 6 (Anime)", tags: ["Anime", "2D"] },
    ],
  },
  {
    id: "black-forest-labs",
    name: "Black Forest Labs",
    models: [
      { id: "flux-schnell", name: "FLUX.1 [schnell]", tags: ["超快速度", "优秀的细节"] },
      { id: "flux-dev", name: "FLUX.1 [dev]", tags: ["高画质", "开源"] },
      { id: "flux-pro", name: "FLUX.1 [pro]", tags: ["顶级画质"] },
    ]
  },
  {
    id: "stability",
    name: "Stability AI",
    models: [
      { id: "sdxl-1.0", name: "SDXL 1.0", tags: ["开源支持丰富插件", "高画质"] },
      { id: "sd3", name: "Stable Diffusion 3", tags: ["SOTA", "排版增强"] },
    ],
  },
  {
    id: "openai",
    name: "OpenAI",
    models: [
      { id: "dall-e-3", name: "DALL-E 3", tags: ["语义理解强", "无需复杂提示词"] },
    ],
  },
];

export const AIImagePage: React.FC = () => {
  
  
const { t } = useTranslation('ai_image');
  const navigate = useNavigate();
  const [prompt, setPrompt] = useState("");
  const [negativePrompt, setNegativePrompt] = useState("");
  const [aspectRatio, setAspectRatio] =
    useState<AIImageOptions["aspectRatio"]>("1:1");
  const [style, setStyle] = useState("Photography");
  const [showAdvanced, setShowAdvanced] = useState(false);
  
  const [showModelSelection, setShowModelSelection] = useState(false);
  const [selectedModelId, setSelectedModelId] = useState("midjourney-v6");
  const [selectedModelName, setSelectedModelName] = useState("Midjourney v6");
  const [selectedVendorId, setSelectedVendorId] = useState("midjourney");

  // Professional parameters
  const [cfgScale, setCfgScale] = useState(7);
  const [steps, setSteps] = useState(30);
  const [seed, setSeed] = useState("");

  const [isGenerating, setIsGenerating] = useState(false);
  const [currentProgress, setCurrentProgress] = useState(0);
  const [currentTask, setCurrentTask] = useState<ImageTask | null>(null);
  const [history, setHistory] = useState<ImageTask[]>([]);
  const [isOptimizingPrompt, setIsOptimizingPrompt] = useState(false);

  const handleOptimizePrompt = async () => {
    if (!prompt.trim()) return showToast(t('settings.generate_error'));
    setIsOptimizingPrompt(true);
    try {
      const optimized = await AIImageService.optimizePrompt(prompt);
      setPrompt(optimized);
      showToast(t('settings.optimize_success'));
    } catch (e) {
      showToast(t('settings.optimize_failed'));
    } finally {
      setIsOptimizingPrompt(false);
    }
  };

  const styles = [
    "Photography",
    "Anime",
    "Cyberpunk",
    "Oil Painting",
    "3D Render",
    "Pixel Art",
  ];
  const ratios: AIImageOptions["aspectRatio"][] = [
    "1:1",
    "16:9",
    "9:16",
    "4:3",
  ];

  useEffect(() => {
    AIImageService.getHistory().then(setHistory);
  }, []);

  const handleGenerate = async () => {
    if (!prompt.trim()) return showToast(t('settings.generate_error'));
    setIsGenerating(true);
    setCurrentProgress(0);

    const options: AIImageOptions = {
      prompt,
      negativePrompt,
      aspectRatio,
      style,
    };
    setCurrentTask({
      id: "temp",
      options,
      status: "generating",
      progress: 0,
      createdAt: Date.now(),
    });

    try {
      const task = await AIImageService.generateImage(options, (p) =>
        setCurrentProgress(p),
      );
      setCurrentTask(task);
      setHistory((prev) => [task, ...prev.filter((t) => t.id !== "temp")]);
      showToast(t('settings.generate_success'));
    } catch (err) {
      showToast(t('settings.generate_failed'));
      setCurrentTask(null);
    } finally {
      setIsGenerating(false);
      setCurrentProgress(0);
    }
  };

  const downloadImage = async (url?: string) => {
    if (!url) return;
    try {
      const resp = await fetch(url);
      const blob = await resp.blob();
      const objUrl = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = objUrl;
      a.download = `ai_image_${Date.now()}.png`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      window.URL.revokeObjectURL(objUrl);
      showToast(t('result.save_success'));
    } catch (e) {
      console.warn("Fetch failed, opening in new tab", e);
      window.open(url, "_blank", "noopener,noreferrer");
      showToast(t('result.save_browser'));
    }
  };

  const handleDelete = (e: React.MouseEvent, id: string) => {
  e.stopPropagation();
    AIImageService.deleteFromHistory(id);
    setHistory((prev) => prev.filter((t) => t.id !== id));
    if (currentTask?.id === id) {
      setCurrentTask(null);
      setPrompt("");
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black pt-safe relative">
      <header className="h-[44px] flex items-center justify-between px-2 shrink-0 bg-bg-color border-b border-border-color">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <span className="font-medium text-[17px] text-text-main">{t('header_title')}</span>
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
        {/* Settings Block */}
        <AIImageSettingsPanel
          t={t}
          showModelSelection={showModelSelection}
          setShowModelSelection={setShowModelSelection}
          selectedModelName={selectedModelName}
          selectedVendorId={selectedVendorId}
          prompt={prompt}
          setPrompt={setPrompt}
          negativePrompt={negativePrompt}
          setNegativePrompt={setNegativePrompt}
          aspectRatio={aspectRatio}
          setAspectRatio={setAspectRatio}
          ratios={ratios}
          style={style}
          setStyle={setStyle}
          styles={styles}
          showAdvanced={showAdvanced}
          setShowAdvanced={setShowAdvanced}
          cfgScale={cfgScale}
          setCfgScale={setCfgScale}
          steps={steps}
          setSteps={setSteps}
          seed={seed}
          setSeed={setSeed}
          isGenerating={isGenerating}
          isOptimizingPrompt={isOptimizingPrompt}
          handleOptimizePrompt={handleOptimizePrompt}
          handleGenerate={handleGenerate}
        />

        <AIImageHistoryPanel
          t={t}
          currentTask={currentTask}
          history={history}
          isGenerating={isGenerating}
          currentProgress={currentProgress}
          downloadImage={downloadImage}
          handleDelete={handleDelete}
          setPrompt={setPrompt}
          setAspectRatio={setAspectRatio}
          setStyle={setStyle}
          setCurrentTask={setCurrentTask}
        />
      </div>
      
      {showModelSelection && (
        <ModelSelectionPage
          title={t('model_page_title')}
          currentModelId={selectedModelId}
          vendors={IMAGE_VENDORS}
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
