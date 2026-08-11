import { useTranslation } from "react-i18next";
import React from "react";
import { Search, Sparkles, Wand2, Copy, Heart, Share2, Play } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

export const PromptsTab = () => {
  const { t } = useTranslation();
const prompts = [
    {
      id: 1,
      title: "赛博朋克城市设定图",
      type: "图像",
      content: "未来世界，霓虹闪烁的赛博朋克城市夜景，雨后潮湿的地面反光，高耸入云的摩天大楼，飞行汽车穿梭其间，8k分辨率，辛烷值渲染，极致细节。",
      uses: 12500,
      likes: 3400
    },
    {
      id: 2,
      title: "产品发布会高燃开场视频",
      type: "视频",
      content: "镜头从宇宙深处快速拉近地球，穿过云层，聚焦到一个充满科技感的未来都市，随后屏幕出现粒子特效组成的LOGO，配以震撼的鼓点节奏，适合科技峰会开场。",
      uses: 8900,
      likes: 2100
    },
    {
      id: 3,
      title: "深沉Lo-Fi学习音乐",
      type: "音乐",
      content: "一首深沉的Lo-Fi音乐，带有夜晚都市的霓虹氛围和轻微的黑胶底噪，适合安静地学习和深夜冥想，节奏舒缓连贯。",
      uses: 45000,
      likes: 12000
    },
    {
      id: 4,
      title: "专业的技术架构分析",
      type: "文本",
      content: "作为一位资深的系统架构师，请对目前的微服务架构和Serverless架构进行深度对比，分析它们在处理高并发场景下的优劣势，并提供一个渐进式演进的实施方案。",
      uses: 3200,
      likes: 800
    }
  ];

  return (
    <div className="w-full h-full bg-[#121212] overflow-y-auto pb-[60px]">
       <div className="pt-safe px-4 pb-3 mt-4 sticky top-0 bg-[#121212]/95 backdrop-blur-md z-10">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-[20px] font-bold text-white tracking-wide">{t('channels.auto_n4842822b', 'Prompt Universe')}<Sparkles className="inline-block w-5 h-5 text-purple-400 mb-1" /></h2>
          </div>
          
          <div className="bg-[#2B2B2B] h-10 rounded-full flex items-center px-4 gap-2">
             <Search className="w-4 h-4 text-white/50" />
             <input 
               type="text" 
               placeholder={t('channels.auto_prop_n4dac60c7', 'Search thousands of great prompts...')}
               className="bg-transparent flex-1 outline-none text-[14px] text-white placeholder:text-white/40"
             />
          </div>
          
          <div className="flex gap-4 mt-4 overflow-x-auto no-scrollbar pb-1">
             <FilterTag active label={t('channels.auto_prop_a6c80', 'All')} />
             <FilterTag label={t('channels.auto_prop_28c94a22', 'Image generation')} />
             <FilterTag label={t('channels.auto_prop_40e7d16c', 'Video creation')} />
             <FilterTag label={t('channels.auto_prop_n7f15ef82', 'Music & sound effects')} />
             <FilterTag label={t('channels.auto_prop_2fb45628', 'Text writing')} />
             <FilterTag label={t('channels.auto_prop_a057e', 'Code')} />
          </div>
       </div>

       <div className="px-4 py-2 flex flex-col gap-4">
          {prompts.map(prompt => (
            <div key={prompt.id} className="bg-[#1C1C1E] rounded-xl p-4 border border-white/5 relative overflow-hidden">
               <div className="flex justify-between items-start mb-2">
                  <h3 className="text-white font-bold text-[16px] line-clamp-1 mr-4">{prompt.title}</h3>
                  <span className="px-2 py-0.5 rounded text-[11px] font-medium bg-white/10 text-white/70 whitespace-nowrap">
                    {prompt.type}
                  </span>
               </div>
               
               <div className="bg-[#121212] rounded-lg p-3 text-white/80 text-[14px] leading-relaxed mb-4 relative group">
                 {prompt.content}
                 <div className="absolute top-2 right-2 w-7 h-7 bg-white/10 rounded-md flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer active:bg-white/20">
                    <Copy className="w-3.5 h-3.5 text-white" />
                 </div>
               </div>
               
               <div className="flex items-center justify-between text-white/50 text-[12px]">
                  <div className="flex items-center gap-4">
                     <span className="flex items-center gap-1"><Wand2 className="w-3.5 h-3.5" />{t('channels.auto_n3a4f3f94', 'Used {prompt.uses} times')}</span>
                     <span className="flex items-center gap-1"><Heart className="w-3.5 h-3.5" /> {prompt.likes}</span>
                  </div>
                  
                  <div className="flex items-center gap-3">
                     <Share2 className="w-4 h-4" />
                     <button className="flex items-center gap-1 bg-white text-black px-3 py-1 rounded-full font-bold active:scale-95 transition-transform">
                        <Play className="w-3 h-3 fill-black" />{t('channels.auto_1163f3', 'Try it')}</button>
                  </div>
               </div>
            </div>
          ))}
       </div>
    </div>
  );
};

const FilterTag = ({ label, active }: { label: string, active?: boolean }) => {
  const { t } = useTranslation();
  
  return (
  <span className={cn(
    "text-[13px] font-medium px-4 py-1.5 rounded-full whitespace-nowrap transition-colors border",
    active ? "bg-white text-black border-transparent" : "bg-[#1C1C1E] text-white/70 border-white/10"
  )}>
    {label}
  </span>
);
};

