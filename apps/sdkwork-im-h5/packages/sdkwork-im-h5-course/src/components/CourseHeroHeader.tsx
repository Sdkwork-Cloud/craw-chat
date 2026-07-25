import React from "react";
import { PlayCircle } from "lucide-react";
import { CourseData } from "../services/CourseService";
import { NavigateFunction } from "react-router";

interface CourseHeroHeaderProps {
  course: CourseData;
  id: string;
  navigate: NavigateFunction;
}

export const CourseHeroHeader: React.FC<CourseHeroHeaderProps> = ({
  course,
  id,
  navigate,
}) => {
  return (
    <div className="relative aspect-[4/3] w-full bg-black">
      <img
        src={course.cover}
        alt={course.title}
        className="w-full h-full object-cover opacity-80"
      />
      <div className="absolute inset-0 bg-gradient-to-t from-[#F2F2F7] dark:from-black via-transparent to-black/20" />
      <div className="absolute inset-0 flex items-center justify-center">
        <div
          className="w-16 h-16 bg-white/30 backdrop-blur-sm rounded-full flex items-center justify-center cursor-pointer hover:bg-white/40 transition-colors"
          onClick={() => {
            if (course.type === "live") {
              if (course.isPurchased) {
                navigate(`/course/${id}/live`);
              } else {
                navigate(`/course/${id}/purchase`);
              }
            } else if (course.isPurchased) {
              navigate(`/course/${id}/play`);
            } else {
              navigate(`/course/${id}/purchase`);
            }
          }}
        >
          <PlayCircle className="w-8 h-8 text-white fill-white/80" />
        </div>
      </div>
      {course.type === "live" && (
        <div className="absolute top-[80px] right-4 bg-red-500/90 backdrop-blur-sm text-white text-[12px] px-3 py-1.5 rounded-full flex items-center gap-1.5 font-medium shadow-lg">
          <span className="w-2 h-2 rounded-full bg-white animate-pulse" />
          正在直播中
        </div>
      )}
    </div>
  );
};
