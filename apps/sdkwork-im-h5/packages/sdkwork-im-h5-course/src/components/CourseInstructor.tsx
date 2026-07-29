import type { FC } from "react";
import { CheckCircle2, UserRound } from "lucide-react";
import { useTranslation } from "react-i18next";

export interface CourseInstructorProps {
  instructor: string;
  instructorDesc: string;
  advantages?: string[];
}

export const CourseInstructor: FC<CourseInstructorProps> = ({
  instructor,
  instructorDesc,
  advantages,
}) => {
  const { t } = useTranslation();

  return (
    <>
      {advantages && advantages.length > 0 && (
        <div className="bg-white p-5 dark:bg-[#1C1C1E]">
          <h3 className="mb-4 flex items-center gap-2 text-[17px] font-bold text-text-main">
            <span className="h-4 w-1 rounded-full bg-blue-500" />
            {t("course.auto_41772658", "Course highlights")}
          </h3>
          <div className="flex flex-col gap-3">
            {advantages.map((advantage) => (
              <div
                key={advantage}
                className="flex items-start gap-3 rounded-2xl border border-blue-100/50 bg-blue-50/50 p-4 dark:border-blue-800/10 dark:bg-blue-900/10"
              >
                <CheckCircle2 className="mt-0.5 h-[18px] w-[18px] shrink-0 text-blue-500" />
                <span className="text-[14px] leading-relaxed text-text-main">
                  {advantage}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      <div className="bg-white p-5 dark:bg-[#1C1C1E]">
        <h3 className="mb-4 flex items-center gap-2 text-[17px] font-bold text-text-main">
          <span className="h-4 w-1 rounded-full bg-blue-500" />
          {t("course.auto_40eb8478", "Instructor")}
        </h3>
        <div className="flex items-start gap-4">
          <div
            aria-label={t("course.auto_prop_40ec9931", "Instructor avatar")}
            className="relative z-10 flex h-[52px] w-[52px] shrink-0 items-center justify-center overflow-hidden rounded-full border-2 border-white bg-bg-color text-text-sub shadow-md dark:border-[#2A2A2D]"
          >
            <UserRound className="h-7 w-7" aria-hidden="true" />
          </div>
          <div className="-ml-10 flex-1 rounded-2xl bg-[#F8F9FA] p-4 pl-12 dark:bg-[#2A2A2D]">
            <h4 className="mb-1.5 text-[15px] font-bold text-text-main">
              {instructor}
            </h4>
            <p className="text-[13px] leading-relaxed text-text-sub">
              {instructorDesc}
            </p>
          </div>
        </div>
      </div>
    </>
  );
};
