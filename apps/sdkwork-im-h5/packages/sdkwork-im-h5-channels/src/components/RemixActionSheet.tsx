import { useTranslation } from "react-i18next";
import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { Layers, Scissors, Music, LayoutTemplate, Wand2 } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { CreativeWork } from "../types";

interface RemixActionSheetProps {
  remixWork: CreativeWork | null;
  setRemixWork: (w: CreativeWork | null) => void;
}

export const RemixActionSheet: React.FC<RemixActionSheetProps> = ({ remixWork, setRemixWork }) => {
  const { t } = useTranslation();
return (
    <AnimatePresence>
       {remixWork && (
          <motion.div
             initial={{ opacity: 0 }}
             animate={{ opacity: 1 }}
             exit={{ opacity: 0 }}
             className="fixed inset-0 z-[100] bg-black/60 flex flex-col justify-end touch-none"
             onClick={() => setRemixWork(null)}
          >
             <motion.div
                initial={{ y: "100%" }}
                animate={{ y: 0 }}
                exit={{ y: "100%" }}
                transition={{ type: "spring", damping: 25, stiffness: 200 }}
                className="bg-[#1C1C1E] rounded-t-2xl w-full pb-safe pt-2 px-4 shadow-2xl relative"
                onClick={(e) => e.stopPropagation()}
             >
                <div className="w-10 h-1 bg-white/20 rounded-full mx-auto mb-4" />
                
                <div className="flex items-center gap-3 mb-6">
                   <div className="w-12 h-16 rounded overflow-hidden shrink-0 border border-white/10 shrink-0 bg-black">
                       {remixWork.type === "video" ? (
                          <video src={remixWork.mediaUrl} className="w-full h-full object-cover" muted />
                       ) : (
                          <img src={remixWork.mediaUrl} className="w-full h-full object-cover" alt="" />
                       )}
                   </div>
                   <div className="flex-1 min-w-0">
                      <h3 className="text-white font-semibold text-[16px] mb-1 line-clamp-1 truncate">Remix: {remixWork.title}</h3>
                      <p className="text-white/50 text-[13px]">{t('channels.auto_752d22d0', 'Co-create with @{remixWork.author}')}</p>
                   </div>
                </div>

                <div className="grid grid-cols-4 gap-4 mb-6">
                   <RemixOption icon={Layers} label={t('channels.auto_prop_a8fc5', 'Duet')} color="bg-blue-500" />
                   {remixWork.type === "video" && <RemixOption icon={Scissors} label={t('channels.auto_prop_13ff89c', 'Remix')} color="bg-orange-500" />}
                   {remixWork.type === "image" && <RemixOption icon={Wand2} label={t('channels.auto_prop_3c237dd', 'AI edit')} color="bg-purple-500" />}
                   <RemixOption icon={Music} label={t('channels.auto_prop_1c24459', 'Original audio')} color="bg-rose-500" />
                   {remixWork.type === "video" && <RemixOption icon={LayoutTemplate} label={t('channels.auto_prop_fbf36', 'Green screen')} color="bg-emerald-500" />}
                </div>

                <button 
                   className="w-full py-3.5 bg-white/10 active:bg-white/20 rounded-xl text-white font-medium text-[16px] transition-colors mb-2"
                   onClick={() => setRemixWork(null)}
                >{t('channels.auto_a9472', 'Cancel')}</button>
             </motion.div>
          </motion.div>
       )}
    </AnimatePresence>
  );
};

const RemixOption = ({ icon: Icon, label, color }: any) => {
  const { t } = useTranslation();
  
  return (
  <div className="flex flex-col items-center gap-2 cursor-pointer active:scale-95 transition-transform">
     <div className={cn("w-14 h-14 rounded-full flex items-center justify-center text-white shadow-lg", color)}>
        <Icon className="w-6 h-6" />
     </div>
     <span className="text-white/80 text-[12px] font-medium">{label}</span>
  </div>
);
};

