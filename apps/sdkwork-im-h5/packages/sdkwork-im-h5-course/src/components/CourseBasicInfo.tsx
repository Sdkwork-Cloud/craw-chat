import React from "react";
import { Star, Users, Clock } from "lucide-react";
import { CourseData } from "../services/CourseService";

interface CourseBasicInfoProps {
  course: CourseData;
}

export const CourseBasicInfo: React.FC<CourseBasicInfoProps> = ({ course }) => {
  return (
    <div className="px-5 py-5 bg-white dark:bg-[#1C1C1E] rounded-t-[32px] -mt-8 relative z-10 shadow-[0_-8px_30px_rgba(0,0,0,0.08)] dark:shadow-[0_-8px_30px_rgba(0,0,0,0.3)] border-b border-black/5 dark:border-white/5">
      <div className="flex items-center gap-2 mb-3.5">
        <span
          className={`text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wider ${
            course.type === "live"
              ? "bg-red-500 text-white border border-red-500"
              : "bg-blue-500 text-white border border-blue-500"
          }`}
        >
          {course.type === "live" ? "直播特训" : "精品专栏"}
        </span>
        <div className="flex items-center gap-1 text-orange-500 text-[13px] font-bold">
          <Star className="w-3.5 h-3.5 fill-orange-500" />
          {course.rating.toFixed(1)}
        </div>
      </div>
      <h1 className="text-[22px] font-bold text-text-main leading-tight mb-2 tracking-tight">
        {course.title}
      </h1>
      <p className="text-[14px] text-text-sub mb-5">{course.instructor}</p>

      <div className="flex items-center gap-6 pb-2">
        <div className="flex flex-col gap-1">
          <span className="text-[11px] text-text-sub">参与人数</span>
          <div className="flex items-center gap-1.5 text-text-main text-[13px] font-medium">
            <Users className="w-4 h-4 opacity-70" />
            <span>
              {course.students >= 10000
                ? `${(course.students / 10000).toFixed(1)}w+`
                : course.students}
            </span>
          </div>
        </div>
        <div className="w-[1px] h-8 bg-black/5 dark:bg-white/10" />
        <div className="flex flex-col gap-1">
          <span className="text-[11px] text-text-sub">课程容量</span>
          <div className="flex items-center gap-1.5 text-text-main text-[13px] font-medium">
            <Clock className="w-4 h-4 opacity-70" />
            <span>{course.totalLessons || "-"} 节详尽内容</span>
          </div>
        </div>
      </div>
    </div>
  );
};
