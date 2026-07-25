import { useTranslation } from "react-i18next";
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Search, BookOpen, X } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { CourseCard } from "../components/CourseCard";
import { CourseService, CourseData } from "../services/CourseService";

export const CourseHome: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState("all");
  const [courses, setCourses] = useState<CourseData[]>([]);
  const [loading, setLoading] = useState(true);
  const [showSearch, setShowSearch] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");

  const categories = [
    { id: "all", name: "全部" },
    { id: "live", name: "直播课" },
    { id: "tech", name: "技术开发" },
    { id: "design", name: "设计创意" },
    { id: "business", name: "商业思维" },
  ];

  useEffect(() => {
    const fetchCourses = async () => {
      setLoading(true);
      try {
        const data = await CourseService.getCourses(activeTab);
        setCourses(data);
      } catch (error) {
        console.error("Failed to fetch courses", error);
      } finally {
        setLoading(false);
      }
    };
    fetchCourses();
  }, [activeTab]);


  const displayCourses = courses.filter(c => 
    searchQuery ? c.title.toLowerCase().includes(searchQuery.toLowerCase()) || (c.instructor && c.instructor.toLowerCase().includes(searchQuery.toLowerCase())) : true
  );

  return (
    <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black overflow-hidden relative">
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 pt-safe bg-bg-color shrink-0 shadow-sm border-b border-black/5 dark:border-white/5">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          className="bg-transparent w-10 h-10 -ml-2"
          onClick={() => navigate(-1)}
        />
        {!showSearch ? (
          <h1 className="text-[17px] font-semibold text-text-main">{t('course.auto_298bb0a4', '在线课程')}</h1>
        ) : (
          <div className="flex-1 px-2 fade-in">
             <input 
               autoFocus
               type="text"
               value={searchQuery}
               onChange={(e) => setSearchQuery(e.target.value)}
               placeholder={t('course.auto_prop_1231ec55', '搜索课程或讲师...')}
               className="w-full bg-black/5 dark:bg-white/10 rounded-full px-4 py-1.5 text-[14px] text-text-main outline-none"
             />
          </div>
        )}
        <div className="flex items-center -mr-2">
           {!showSearch && (
             <IconButton
               icon={<BookOpen className="w-5 h-5 text-text-main" />}
               className="bg-transparent w-9 h-9 fade-in"
               onClick={() => navigate('/course/my')}
             />
           )}
           <IconButton
             icon={showSearch ? <X className="w-5 h-5 text-text-main" /> : <Search className="w-5 h-5 text-text-main" />}
             className="bg-transparent w-9 h-9"
             onClick={() => {
                if (showSearch) setSearchQuery("");
                setShowSearch(!showSearch);
             }}
           />
        </div>
      </header>

      <div className="flex-1 overflow-y-auto w-full pb-safe">
        {/* Banner */}
        <div className="p-4 bg-white dark:bg-[#1C1C1E]">
           <div className="bg-gradient-to-r from-blue-600 to-indigo-600 rounded-2xl p-6 text-white flex flex-col justify-end shadow-sm relative overflow-hidden h-[160px] cursor-pointer">
              <div className="absolute inset-0 bg-[url('https://picsum.photos/seed/cb/800/400')] opacity-40 mix-blend-overlay object-cover" />
              <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent" />
              <div className="absolute top-3 right-3 bg-white/20 backdrop-blur-md px-2 py-0.5 rounded text-[10px] font-medium border border-white/10">{t('course.auto_250d7220', '主打推荐')}</div>
              <div className="relative z-10">
                 <h2 className="text-[20px] font-bold mb-1 leading-tight">{t('course.auto_n22f09a3c', '2026 年度跨端技术合集')}</h2>
                 <p className="text-[13px] text-white/80 line-clamp-1">{t('course.auto_43bffece', '掌握前沿开发趋势，提升核心竞争力')}</p>
              </div>
           </div>
           {/* Dots */}
           <div className="flex justify-center gap-1.5 mt-3">
              <div className="w-1.5 h-1.5 rounded-full bg-blue-500" />
              <div className="w-1.5 h-1.5 rounded-full bg-black/10 dark:bg-white/20" />
              <div className="w-1.5 h-1.5 rounded-full bg-black/10 dark:bg-white/20" />
           </div>
        </div>

        {/* Categories */}
        <div className="bg-white dark:bg-[#1C1C1E] px-4 py-3 border-b border-black/5 dark:border-white/5 sticky top-0 z-10 flex overflow-x-auto hide-scrollbar gap-4">
           {categories.map((cat) => (
             <div 
               key={cat.id}
               onClick={() => setActiveTab(cat.id)}
               className={`whitespace-nowrap pb-2 text-[15px] font-medium transition-colors cursor-pointer border-b-2 ${
                 activeTab === cat.id 
                   ? "text-blue-500 border-blue-500" 
                   : "text-text-sub border-transparent"
               }`}
             >
               {cat.name}
             </div>
           ))}
        </div>

        {/* Course List */}
        <div className="flex flex-col bg-white dark:bg-[#1C1C1E] min-h-[300px]">
          {loading ? (
            <div className="flex flex-col">
               {[1, 2, 3, 4].map(i => (
                  <div key={i} className="flex items-start gap-4 p-4 border-b border-black/5 dark:border-white/5 animate-pulse">
                     <div className="w-[130px] shrink-0 aspect-[4/3] rounded-xl bg-black/5 dark:bg-white/5" />
                     <div className="flex flex-col flex-1 h-[97px] py-1">
                        <div className="w-full h-4 bg-black/5 dark:bg-white/5 rounded mt-1" />
                        <div className="w-2/3 h-4 bg-black/5 dark:bg-white/5 rounded mt-2" />
                        <div className="w-1/3 h-3 bg-black/5 dark:bg-white/5 rounded mt-3" />
                        <div className="w-1/4 h-5 bg-black/5 dark:bg-white/5 rounded mt-auto" />
                     </div>
                  </div>
               ))}
            </div>
          ) : displayCourses.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-20 text-text-sub">
                <BookOpen className="w-12 h-12 mb-3 text-black/10 dark:text-white/10" />
                <span className="text-[14px]">{t('course.auto_3028cdeb', '暂无课程')}</span>
            </div>
          ) : (
            displayCourses.map((course) => (
               <CourseCard 
                 key={course.id} 
                 course={course} 
                 onClick={() => navigate(`/course/${course.id}`)} 
               />
            ))
          )}
        </div>
      </div>
    </div>
  );
};
