import React, { useState } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Loader2,
} from "lucide-react";
import { IconButton, cn, showToast, ModelSelectionPage, ModelVendor } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";
import { AIMusicCreatePanel } from "../components/AIMusicCreatePanel";
import { AIMusicLibraryPanel } from "../components/AIMusicLibraryPanel";

export interface MusicTask {
  id: string;
  prompt: string;
  style: string;
  status: "processing" | "completed" | "failed";
  progress: number;
  audioUrl?: string;
  coverUrl?: string;
  title?: string;
}

const MUSIC_VENDORS: ModelVendor[] = [
  {
    id: "suno",
    name: "Suno AI",
    models: [
      { id: "suno-v3.5", name: "Suno v3.5", tags: ["最强人声", "最高达4分钟", "多语言"] },
      { id: "suno-v3", name: "Suno v3", tags: ["快速生成"] },
    ]
  },
  {
    id: "udio",
    name: "Udio",
    models: [
      { id: "udio-32", name: "Udio-32", tags: ["无损音质", "极佳结构", "32秒"] },
      { id: "udio-130", name: "Udio-130", tags: ["130秒超长片段"] },
    ]
  },
  {
    id: "stability",
    name: "Stability AI",
    models: [
      { id: "stable-audio-2", name: "Stable Audio 2.0", tags: ["纯音乐大师", "声效最强"] },
    ]
  }
];

export const AIMusicPage: React.FC = () => {
  
  
const { t } = useTranslation('ai_music');
  const navigate = useNavigate();
  const [prompt, setPrompt] = useState("");
  const [lyrics, setLyrics] = useState("");
  const [style, setStyle] = useState("Pop");
  const [isInstrumental, setIsInstrumental] = useState(false);
  const [mode, setMode] = useState<"create" | "library">("create");

  const [showModelSelection, setShowModelSelection] = useState(false);
  const [selectedModelId, setSelectedModelId] = useState("suno-v3.5");
  const [selectedModelName, setSelectedModelName] = useState("Suno v3.5");
  const [selectedVendorId, setSelectedVendorId] = useState("suno");
  const [showAdvanced, setShowAdvanced] = useState(false);

  // Advanced Music Control
  const [vocalType, setVocalType] = useState("auto");
  const [tempo, setTempo] = useState("auto");

  const [isGenerating, setIsGenerating] = useState(false);
  const [currentProgress, setCurrentProgress] = useState(0);
  const [history, setHistory] = useState<MusicTask[]>([]);
  const [playingId, setPlayingId] = useState<string | null>(null);

  const styles = [
    "Pop",
    "Rock",
    "Electronic",
    "Hip Hop",
    "Classical",
    "Jazz",
    "Lo-Fi"
  ];

  const handleGenerate = () => {
  if (!prompt.trim() && !lyrics.trim() && !isInstrumental) return showToast(t('create.generate_error'));
    
    setIsGenerating(true);
    setCurrentProgress(0);
    setMode("library");

    const newTask: MusicTask = {
      id: Math.random().toString(),
      prompt: prompt || "Custom track",
      style,
      status: "processing",
      progress: 0,
      title: "Generating Track..."
    };

    setHistory([newTask, ...history]);

    let p = 0;
    const interval = setInterval(() => {
      p += 5;
      setCurrentProgress(p);
      setHistory(prev => {
        const next = [...prev];
        if (next[0].id === newTask.id) {
          next[0].progress = p;
        }
        return next;
      });

      if (p >= 100) {
        clearInterval(interval);
        setIsGenerating(false);
        showToast(t('create.generate_success'));
        setHistory(prev => {
          const next = [...prev];
          if (next[0].id === newTask.id) {
            next[0].status = "completed";
            next[0].title = prompt.length > 10 ? prompt.substring(0, 10) + "..." : (prompt || style);
            next[0].coverUrl = "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/unsplash-1614613535308-eb5fbd6d2c17.png";
          }
          return next;
        });
      }
    }, 200);
  };

  const handlePlay = (id: string) => {
  if (playingId === id) {
      setPlayingId(null);
    } else {
      setPlayingId(id);
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#111111] text-white overflow-hidden relative">
      <header className="flex items-center justify-between px-4 pt-safe-top h-14 bg-[#111111]/80 backdrop-blur-md z-10 shrink-0 border-b border-white/5">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-white" />}
          onClick={() => navigate(-1)}
          className="w-10 h-10 -ml-2"
        />
        <div className="flex bg-[#222] rounded-full p-1">
           <button
             onClick={() => setMode("create")}
             className={cn(
               "px-4 py-1.5 rounded-full text-[14px] font-medium transition-all",
               mode === "create" ? "bg-white text-black" : "text-white/60"
             )}
           >
             {t('tabs.create')}
           </button>
           <button
             onClick={() => setMode("library")}
             className={cn(
               "px-4 py-1.5 rounded-full text-[14px] font-medium transition-all flex items-center gap-1.5",
               mode === "library" ? "bg-white text-black" : "text-white/60"
             )}
           >
             {t('tabs.library')}
             {isGenerating && (
                <Loader2 className="w-3 h-3 animate-spin"/>
             )}
           </button>
        </div>
        <div className="w-10"></div>
      </header>

      <div className="flex-1 overflow-y-auto no-scrollbar pb-10">
        {mode === "create" ? (
          <AIMusicCreatePanel
            t={t}
            showModelSelection={showModelSelection}
            setShowModelSelection={setShowModelSelection}
            selectedModelName={selectedModelName}
            isInstrumental={isInstrumental}
            setIsInstrumental={setIsInstrumental}
            prompt={prompt}
            setPrompt={setPrompt}
            lyrics={lyrics}
            setLyrics={setLyrics}
            style={style}
            setStyle={setStyle}
            styles={styles}
            showAdvanced={showAdvanced}
            setShowAdvanced={setShowAdvanced}
            vocalType={vocalType}
            setVocalType={setVocalType}
            tempo={tempo}
            setTempo={setTempo}
            isGenerating={isGenerating}
            handleGenerate={handleGenerate}
          />
        ) : (
          <AIMusicLibraryPanel
            t={t}
            history={history}
            setMode={setMode}
            playingId={playingId}
            handlePlay={handlePlay}
            vocalType={vocalType}
            tempo={tempo}
          />
        )}
      </div>

      {showModelSelection && (
        <ModelSelectionPage
          title={t('model_page_title')}
          currentModelId={selectedModelId}
          vendors={MUSIC_VENDORS}
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
