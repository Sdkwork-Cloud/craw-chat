import { useTranslation } from "react-i18next";
import React from "react";
import { PlayCircle, Lock } from "lucide-react";

export interface PlayerLesson {
  id: string;
  title: string;
  duration: string;
  completed?: boolean;
  free?: boolean;
}

export interface PlayerSection {
  section: string;
  lessons: PlayerLesson[];
}

export interface PlayerCatalogProps {
  curriculum: PlayerSection[];
  activeLesson: string;
  isPurchased?: boolean;
  onLessonSelect: (lessonId: string) => void;
}

export const PlayerCatalog: React.FC<PlayerCatalogProps> = ({ curriculum = [], activeLesson, isPurchased, onLessonSelect }) => {
  const { t } = useTranslation();
return (
    <div className="p-4 pt-4 mt-2 pb-safe bg-white dark:bg-[#1C1C1E] min-h-full">
        <div className="flex flex-col gap-6">
          {curriculum.map((section, idx) => (
            <div key={idx} className="flex flex-col gap-3">
                <h4 className="text-[15px] font-bold text-text-main">{section.section}</h4>
                <div className="flex flex-col gap-1.5 pl-2 border-l-2 border-black/5 dark:border-white/5">
                  {section.lessons.map((lesson) => {
                    const isLocked = !isPurchased && !lesson.free;
                    return (
                    <div 
                      key={lesson.id} 
                      onClick={() => {
                        if (!isLocked) onLessonSelect(lesson.id);
                      }}
                      className={`flex items-center justify-between p-3 rounded-xl transition-colors ${isLocked ? "opacity-60 cursor-not-allowed" : "cursor-pointer"} ${activeLesson === lesson.id ? "bg-blue-50 dark:bg-blue-500/10" : !isLocked ? "active:bg-black/5 dark:active:bg-white/5" : ""}`}
                    >
                        <div className="flex items-center gap-3 min-w-0 flex-1">
                          {activeLesson === lesson.id ? (
                              <div className="w-5 h-5 flex items-center justify-center shrink-0">
                                <div className="flex items-end gap-0.5 h-3">
                                    <div className="w-1 bg-blue-500 h-full animate-[bounce_1s_infinite]" />
                                    <div className="w-1 bg-blue-500 h-2/3 animate-[bounce_1s_infinite_0.2s]" />
                                    <div className="w-1 bg-blue-500 h-1/2 animate-[bounce_1s_infinite_0.4s]" />
                                </div>
                              </div>
                          ) : isLocked ? (
                            <Lock className="w-4 h-4 text-text-sub shrink-0 mx-0.5" />
                          ) : lesson.completed ? (
                            <div className="w-5 h-5 rounded-full bg-blue-500 flex items-center justify-center shrink-0">
                                <span className="text-white text-[10px] font-bold">✓</span>
                            </div>
                          ) : (
                            <PlayCircle className="w-5 h-5 text-text-sub shrink-0 opacity-40" />
                          )}
                          <span className={`text-[14px] truncate ${activeLesson === lesson.id ? "text-blue-500 font-bold" : lesson.completed ? "text-text-sub" : "text-text-main"}`}>
                            {lesson.title}
                          </span>
                        </div>
                        <span className={`text-[12px] font-mono shrink-0 ml-3 ${activeLesson === lesson.id ? "text-blue-500" : "text-text-sub opacity-70"}`}>
                          {lesson.duration}
                        </span>
                    </div>
                  )})}
                </div>
            </div>
          ))}
        </div>
    </div>
  );
};
