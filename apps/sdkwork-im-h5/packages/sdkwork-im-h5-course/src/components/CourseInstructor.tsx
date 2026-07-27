import { useTranslation } from "react-i18next";
import React from "react";
import { CheckCircle2 } from "lucide-react";

export interface CourseInstructorProps {
  instructor: string;
  instructorDesc: string;
  advantages?: string[];
}

export const CourseInstructor: React.FC<CourseInstructorProps> = ({ instructor, instructorDesc, advantages }) => {
  const { t } = useTranslation();
return (
    <>
    {/* Advantages */}
    {advantages && advantages.length > 0 && (
      <div className="p-5 bg-white dark:bg-[#1C1C1E]">
          <h3 className="text-[17px] font-bold text-text-main mb-4 flex items-center gap-2">
            <span className="w-1 h-4 bg-blue-500 rounded-full" />{t('course.auto_41772658', '课程亮点')}</h3>
          <div className="flex flex-col gap-3">
            {advantages.map((adv, idx) => (
              <div key={idx} className="flex items-start gap-3 bg-blue-50/50 dark:bg-blue-900/10 p-4 rounded-2xl border border-blue-100/50 dark:border-blue-800/10">
                  <CheckCircle2 className="w-[18px] h-[18px] text-blue-500 shrink-0 mt-0.5" />
                  <span className="text-[14px] text-text-main leading-relaxed">{adv}</span>
              </div>
            ))}
          </div>
      </div>
    )}

      {/* Instructor */}
      <div className="p-5 bg-white dark:bg-[#1C1C1E]">
          <h3 className="text-[17px] font-bold text-text-main mb-4 flex items-center gap-2">
            <span className="w-1 h-4 bg-blue-500 rounded-full" />{t('course.auto_40eb8478', '讲师介绍')}</h3>
          <div className="flex items-start gap-4">
            <div className="w-[52px] h-[52px] rounded-full overflow-hidden shrink-0 border-2 border-white dark:border-[#2A2A2D] shadow-md relative z-10">
                <img src="https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/avatar2/100x100.png" alt={t('course.auto_prop_40ec9931', '讲师头像')} className="w-full h-full object-cover" />
            </div>
            <div className="flex-1 bg-[#F8F9FA] dark:bg-[#2A2A2D] p-4 rounded-2xl -ml-10 pl-12">
                <h4 className="text-[15px] font-bold text-text-main mb-1.5">{instructor}</h4>
                <p className="text-[13px] text-text-sub leading-relaxed">{instructorDesc}</p>
            </div>
          </div>
      </div>
    </>
  );
};
