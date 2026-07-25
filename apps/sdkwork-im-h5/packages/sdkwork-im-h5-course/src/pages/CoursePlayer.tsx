import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useParams } from "react-router";
import { PlayerCatalog } from "../components/PlayerCatalog";
import { PlayerDiscussion } from "../components/PlayerDiscussion";
import { VideoPlayer } from "../components/VideoPlayer";
import { CourseService, CourseData } from "../services/CourseService";

export const CoursePlayer: React.FC = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const [activeTab, setActiveTab] = useState<"catalog" | "discussion">("catalog");
  const [activeLesson, setActiveLesson] = useState("");
  const [isPlaying, setIsPlaying] = useState(false);
  const [course, setCourse] = useState<CourseData | null>(null);
  const [loading, setLoading] = useState(true);
  const [completedLessons, setCompletedLessons] = useState<Record<string, boolean>>({});

  useEffect(() => {
    const fetchCourse = async () => {
      if (!id) return;
      setLoading(true);
      try {
        const data = await CourseService.getCourseDetail(id);
        if (data) {
          setCourse(data);
          if (data.curriculum && data.curriculum[0]?.lessons[0]) {
             setActiveLesson(data.curriculum[0].lessons[0].id || "");
          }
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
        <div className="flex flex-col h-[100dvh] bg-white dark:bg-black overflow-hidden relative items-center justify-center">
           <span className="text-[14px] text-text-sub">{t('course.auto_7f6f37e', '加载中...')}</span>
        </div>
     );
  }

  if (!course || !course.curriculum) {
     return (
        <div className="flex flex-col h-[100dvh] bg-white dark:bg-black overflow-hidden relative items-center justify-center">
           <span className="text-[14px] text-text-sub">{t('course.auto_292f7f7d', '未找到课程内容')}</span>
        </div>
     );
  }

  // Find current lesson URL
  const allLessons = course.curriculum.flatMap(s => s.lessons);
  const currentLessonData = allLessons.find(l => l.id === activeLesson);
  const videoSrc = currentLessonData?.videoUrl;

  const handleVideoEnded = () => {
  setCompletedLessons(prev => ({ ...prev, [activeLesson]: true }));
    // Attempt auto play next
    const currentIndex = allLessons.findIndex(l => l.id === activeLesson);
    if (currentIndex >= 0 && currentIndex < allLessons.length - 1) {
       const nextLesson = allLessons[currentIndex + 1];
       if (!nextLesson.free && !course.isPurchased) {
          // Can't auto play an unpurchased non-free video
          return;
       }
       setActiveLesson(nextLesson.id!);
       setIsPlaying(true);
    }
  };

  return (
    <div className="flex flex-col h-[100dvh] bg-white dark:bg-black overflow-hidden relative">
      {/* Video Player Area (Sticky Top) */}
      <VideoPlayer 
        videoSrc={videoSrc}
        isPlaying={isPlaying}
        setIsPlaying={setIsPlaying}
        onEnded={handleVideoEnded}
      />

      {/* Tabs */}
      <div className="flex px-5 gap-6 border-b border-black/5 dark:border-white/5 shrink-0 z-10 bg-white dark:bg-[#1C1C1E]">
        <button
           onClick={() => setActiveTab("catalog")}
           className={`py-3.5 text-[15px] font-medium transition-colors border-b-2 ${activeTab === "catalog" ? "border-blue-500 text-blue-500" : "border-transparent text-text-sub"}`}
        >{t('course.auto_1207dd', '选集')}</button>
        <button
           onClick={() => setActiveTab("discussion")}
           className={`py-3.5 text-[15px] font-medium transition-colors border-b-2 ${activeTab === "discussion" ? "border-blue-500 text-blue-500" : "border-transparent text-text-sub"}`}
        >{t('course.auto_117512', '讨论')}</button>
      </div>

      {/* Content Scroll Area */}
      <div className="flex-1 overflow-y-auto bg-[#F2F2F7] dark:bg-black relative">
         <div className="p-4 bg-white dark:bg-[#1C1C1E]">
            <h1 className="text-[16px] font-bold text-text-main leading-snug mb-2">{course.title}</h1>
            <div className="text-[12px] text-text-sub flex items-center gap-2">
               <span className="bg-blue-50 text-blue-500 dark:bg-blue-500/10 px-1.5 py-0.5 rounded">{t('course.auto_18f1871', '更新中')}</span>
               <span>{t('course.total_lessons', `共 ${allLessons.length} 节课`)}</span>
            </div>
         </div>

         {activeTab === "catalog" && (
            <PlayerCatalog 
              curriculum={course.curriculum.map(s => ({
                 ...s,
                 lessons: s.lessons.map(l => ({ ...l, completed: completedLessons[l.id!] || l.completed }))
              }))}
              activeLesson={activeLesson}
              isPurchased={course.isPurchased}
              onLessonSelect={(newId) => {
                 setActiveLesson(newId);
                 setIsPlaying(true);
              }}
            />
         )}

         {activeTab === "discussion" && <PlayerDiscussion courseId={id!} lessonId={activeLesson} />}
      </div>
    </div>
  );
};
