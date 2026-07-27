import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router";
import { ChevronLeft, Share } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { CourseInstructor } from "../components/CourseInstructor";
import { CourseCurriculum } from "../components/CourseCurriculum";
import { CourseHeroHeader } from "../components/CourseHeroHeader";
import { CourseBasicInfo } from "../components/CourseBasicInfo";
import { CourseFooterBar } from "../components/CourseFooterBar";
import { CourseService, CourseData } from "../services/CourseService";

export const CourseDetail: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const [activeTab, setActiveTab] = useState<"intro" | "catalog">("intro");
  const [course, setCourse] = useState<CourseData | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchCourse = async () => {
      if (!id) return;
      setLoading(true);
      try {
        const data = await CourseService.getCourseDetail(id);
        if (data) {
          setCourse(data);
        }
      } catch (error) {
        console.error("Failed to fetch course details", error);
      } finally {
        setLoading(false);
      }
    };
    fetchCourse();
  }, [id]);

  if (loading) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black overflow-hidden relative items-center justify-center">
        <span className="text-[14px] text-text-sub">加载中...</span>
      </div>
    );
  }

  if (!course) {
    return (
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black overflow-hidden relative items-center justify-center">
        <span className="text-[14px] text-text-sub">未找到课程</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black overflow-hidden relative">
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 pt-safe bg-transparent shrink-0">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-white drop-shadow-md" />}
          className="bg-black/30 backdrop-blur-md w-10 h-10 -ml-2 border border-white/10"
          onClick={() => navigate(-1)}
        />
        <IconButton
          icon={<Share className="w-5 h-5 text-white drop-shadow-md" />}
          className="bg-black/30 backdrop-blur-md w-10 h-10 -mr-2 border border-white/10"
          onClick={() => {}}
        />
      </header>

      <div className="flex-1 overflow-y-auto w-full -mt-[56px] pb-[90px]">
        {/* Hero Header */}
        <CourseHeroHeader course={course} id={id!} navigate={navigate} />

        {/* Basic Info */}
        <CourseBasicInfo course={course} />

        {/* Sticky Tabs */}
        <div className="sticky top-[56px] z-10 bg-white dark:bg-[#1C1C1E] border-b border-black/5 dark:border-white/5 flex px-5 gap-6">
          <button
            onClick={() => setActiveTab("intro")}
            className={`py-3 text-[15px] font-medium transition-colors border-b-2 ${
              activeTab === "intro"
                ? "border-blue-500 text-blue-500"
                : "border-transparent text-text-sub"
            }`}
          >
            介绍
          </button>
          <button
            onClick={() => setActiveTab("catalog")}
            className={`py-3 text-[15px] font-medium transition-colors border-b-2 ${
              activeTab === "catalog"
                ? "border-blue-500 text-blue-500"
                : "border-transparent text-text-sub"
            }`}
          >
            大纲
          </button>
        </div>

        {/* Contents */}
        <div className="flex flex-col gap-2 mt-2">
          {activeTab === "intro" && (
            <CourseInstructor
              instructor={course.instructor}
              instructorDesc={course.instructorDesc}
              advantages={course.advantages}
            />
          )}

          {activeTab === "catalog" && (
            <CourseCurriculum
              courseId={id!}
              courseType={course.type!}
              isPurchased={course.isPurchased!}
              curriculum={course.curriculum}
            />
          )}
        </div>
      </div>

      {/* Footer Bar */}
      <CourseFooterBar course={course} id={id!} navigate={navigate} />
    </div>
  );
};

