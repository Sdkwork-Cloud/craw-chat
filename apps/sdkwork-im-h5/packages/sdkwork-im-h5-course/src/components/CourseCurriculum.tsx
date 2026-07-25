import { useTranslation } from "react-i18next";
import React from "react";
import { PlayCircle, Lock } from "lucide-react";
import { useNavigate } from "react-router";

export interface CurriculumLesson {
  title: string;
  free?: boolean;
}

export interface CurriculumSection {
  section: string;
  lessons: CurriculumLesson[];
}

export interface CourseCurriculumProps {
  courseId: string;
  courseType: string;
  isPurchased: boolean;
  curriculum: CurriculumSection[];
}

export const CourseCurriculum: React.FC<CourseCurriculumProps> = ({ courseId, courseType, isPurchased, curriculum = [] }) => {
  const { t } = useTranslation();
const navigate = useNavigate();

  return (
    <div className="p-5 bg-white dark:bg-[#1C1C1E]">
      <div className="flex flex-col gap-6">
        {curriculum.map((section, idx) => (
          <div key={idx} className="flex flex-col gap-4">
            <h4 className="text-[15px] font-bold text-text-main">{section.section}</h4>
            <div className="flex flex-col gap-1">
              {section.lessons.map((lesson, lIdx) => (
                <div 
                  key={lIdx} 
                  className={`flex items-center justify-between p-3.5 rounded-2xl cursor-pointer transition-colors active:bg-black/5 dark:active:bg-white/5 ${lesson.free ? "bg-blue-50/50 dark:bg-blue-900/10" : ""}`}
                  onClick={() => {
                    if (isPurchased || lesson.free) {
                      if (courseType !== 'live') navigate(`/course/${courseId}/play`);
                    } 
                  }}
                >
                  <div className="flex items-center gap-3.5 min-w-0 flex-1">
                    <span className="text-[13px] text-text-sub opacity-50 font-mono w-5 shrink-0 text-center">{String(lIdx + 1).padStart(2, '0')}</span>
                    {(lesson.free || isPurchased) && courseType !== 'live' ? (
                      <PlayCircle className="w-5 h-5 text-blue-500 shrink-0" />
                    ) : courseType === 'live' ? (
                      <PlayCircle className="w-5 h-5 text-red-500 shrink-0" />
                    ) : (
                      <Lock className="w-4 h-4 text-text-sub shrink-0 opacity-40 ml-0.5" />
                    )}
                    <span className={`text-[14px] truncate ${(lesson.free || isPurchased) ? "text-text-main font-medium" : "text-text-sub"}`}>{lesson.title}</span>
                  </div>
                  {lesson.free && !isPurchased && courseType !== 'live' && (
                    <span className="text-[11px] text-blue-500 font-medium px-2 py-0.5 bg-blue-500/10 rounded ml-3 shrink-0">{t('course.auto_14c7a05', '可试看')}</span>
                  )}
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
