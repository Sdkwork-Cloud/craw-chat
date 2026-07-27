import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, PlayCircle, Clock, BookOpen, ChevronRight } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { CourseService, MyCourseData } from "../services/CourseService";

export const MyCourses: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [myCourses, setMyCourses] = useState<MyCourseData[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeFilter, setActiveFilter] = useState<"all" | "live" | "recorded">("all");

  useEffect(() => {
    const fetchMyCourses = async () => {
      setLoading(true);
      try {
        const data = await CourseService.getMyCourses();
        setMyCourses(data);
      } catch (error) {
        console.error("Failed to fetch my courses", error);
      } finally {
        setLoading(false);
      }
    };
    fetchMyCourses();
  }, []);

  const filteredCourses = myCourses.filter(course => {
     if (activeFilter === 'all') return true;
     if (activeFilter === 'live') return course.isLive;
     if (activeFilter === 'recorded') return !course.isLive;
     return true;
  });

  return (
    <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-black w-full overflow-hidden">
      {/* Header */}
      <header className="h-14 px-4 flex items-center justify-between sticky top-0 z-20 bg-white dark:bg-[#121212] shrink-0 border-b border-black/5 dark:border-white/5 w-full">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          className="bg-transparent w-10 h-10 -ml-2 shrink-0"
          onClick={() => navigate(-1)}
        />
        <h1 className="text-[17px] font-semibold text-text-main flex-1 text-center truncate px-2">我的课程</h1>
        <div className="w-10 -mr-2 shrink-0" />
      </header>
      
      {/* Tabs */}
      <div className="bg-white dark:bg-[#121212] px-4 h-12 flex items-center gap-6 shrink-0 w-full relative z-10 shadow-sm">
         {(['all', 'recorded', 'live'] as const).map(f => (
            <button
              key={f}
              onClick={() => setActiveFilter(f)}
              className={`h-full relative text-[15px] font-medium transition-colors ${activeFilter === f ? "text-blue-600 dark:text-blue-500" : "text-text-sub"}`}
            >
              {f === 'all' ? '全部' : f === 'recorded' ? '录播课' : '直播课'}
              {activeFilter === f && (
                <span className="absolute bottom-0 left-1/2 -translate-x-1/2 w-4 h-0.5 bg-blue-600 dark:bg-blue-500 rounded-full" />
              )}
            </button>
         ))}
      </div>

      {/* List */}
      <div className="flex-1 overflow-x-hidden overflow-y-auto w-full">
         <div className="p-4 pb-safe space-y-4">
            {loading ? (
                <div className="space-y-4 w-full">
                   {[1, 2, 3].map(i => (
                      <div key={i} className="bg-white dark:bg-[#1C1C1E] rounded-2xl p-4 border border-black/5 dark:border-white/5 animate-pulse w-full">
                         <div className="flex gap-3">
                            <div className="w-24 aspect-[4/3] rounded-xl bg-black/5 dark:bg-white/5 shrink-0" />
                            <div className="flex flex-col flex-1 py-1 w-full min-w-0">
                               <div className="w-full h-4 bg-black/5 dark:bg-white/5 rounded" />
                               <div className="w-2/3 h-4 bg-black/5 dark:bg-white/5 rounded mt-2" />
                            </div>
                         </div>
                      </div>
                   ))}
                </div>
             ) : filteredCourses.length === 0 ? (
                 <div className="flex flex-col items-center justify-center pt-24 text-text-sub">
                     <BookOpen className="w-12 h-12 mb-3 text-black/10 dark:text-white/10" />
                     <span className="text-[14px]">暂无该类课程</span>
                 </div>
             ) : filteredCourses.map(course => (
               <div 
                 key={course.id}
                 className="bg-white dark:bg-[#1C1C1E] rounded-2xl overflow-hidden shadow-sm active:scale-[0.98] transition-transform cursor-pointer w-full"
                 onClick={() => navigate(course.isLive ? `/course/${course.id}/live` : `/course/${course.id}/play`)}
               >
                  <div className="p-4">
                    <div className="flex gap-3">
                       <div className="w-[100px] aspect-[4/3] rounded-xl overflow-hidden shrink-0 relative bg-gray-100 dark:bg-gray-800">
                          <img src={course.cover} alt="cover" className="w-full h-full object-cover" />
                          {course.isLive && (
                             <div className="absolute top-1.5 left-1.5 bg-red-500/90 text-white text-[10px] px-1.5 py-0.5 rounded flex items-center gap-1 font-medium backdrop-blur-sm pointer-events-none">
                               <span className="w-1.5 h-1.5 rounded-full bg-white animate-pulse" />直播中</div>
                          )}
                       </div>
                       <div className="flex-1 min-w-0 flex flex-col justify-between">
                          <h3 className="text-[15px] font-bold text-text-main line-clamp-2 leading-snug w-full break-words">
                            {course.title}
                          </h3>
                          <div className="flex flex-col gap-1 mt-1.5">
                             <span className="text-[12px] text-text-sub truncate w-full">{course.isLive ? course.lastWatched : `上次学习: ${course.lastWatched}`}</span>
                             
                             {!course.isLive && (
                               <div className="flex items-center gap-2 mt-1">
                                 <div className="flex-1 h-1.5 bg-gray-100 dark:bg-[#2A2A2D] rounded-full overflow-hidden shrink-0">
                                    <div className="h-full bg-blue-500 rounded-full" style={{ width: `${course.progress}%` }} />
                                 </div>
                                 <span className="text-[11px] text-text-sub font-medium shrink-0">{course.progress}%</span>
                               </div>
                             )}
                          </div>
                       </div>
                    </div>
                  </div>
                  
                  {/* Footer Actions */}
                  <div className="px-4 py-3 bg-gray-50 dark:bg-[#242426] border-t border-black/5 dark:border-white/5 flex items-center justify-between gap-3 w-full">
                     <span className="text-[12px] text-text-sub flex items-center gap-1 flex-1 min-w-0">
                       {course.isLive ? <Clock className="w-3.5 h-3.5 shrink-0" /> : null}
                       <span className="truncate">{course.isLive ? '正在直播' : `进度: ${course.completedLessons}/${course.totalLessons} 课时`}</span>
                     </span>
                     <button className={`shrink-0 text-[13px] font-medium px-4 py-1.5 rounded-full flex items-center gap-1.5 transition-colors ${course.isLive ? 'bg-red-50 text-red-600 dark:bg-red-500/10 dark:text-red-400' : 'bg-blue-50 text-blue-600 dark:bg-blue-500/10 dark:text-blue-400'}`}>
                       {course.isLive ? null : <PlayCircle className="w-[15px] h-[15px] shrink-0" />}
                       {course.isLive ? "进入直播" : (course.progress >0 ? "继续学习" : "开始学习")}</button>
                  </div>
               </div>
             ))}
         </div>
      </div>
    </div>
  );
};

