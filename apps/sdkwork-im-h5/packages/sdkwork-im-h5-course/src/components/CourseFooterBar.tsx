import React from "react";
import { CourseData } from "../services/CourseService";
import { NavigateFunction } from "react-router";

interface CourseFooterBarProps {
  course: CourseData;
  id: string;
  navigate: NavigateFunction;
}

export const CourseFooterBar: React.FC<CourseFooterBarProps> = ({
  course,
  id,
  navigate,
}) => {
  return (
    <div className="fixed bottom-0 left-0 right-0 pt-4 pb-safe bg-gradient-to-t from-white via-white/95 to-transparent dark:from-[#1C1C1E] dark:via-[#1C1C1E]/95 z-20 pointer-events-none">
      <div className="mx-4 mb-4 bg-white/80 dark:bg-[#2A2A2D]/80 backdrop-blur-xl border border-black/5 dark:border-white/10 rounded-full px-5 py-3 flex items-center justify-between shadow-xl pointer-events-auto">
        <div className="flex flex-col">
          {course.isPurchased ? (
            <span className="text-[14px] text-text-sub font-medium">
              已解锁该课程
            </span>
          ) : (
            <div className="flex flex-col justify-center">
              <div className="flex items-baseline gap-1 -mb-0.5">
                <span className="text-[12px] text-red-500 font-bold">¥</span>
                <span className="text-[22px] text-red-500 font-bold leading-none">
                  {course.price}
                </span>
              </div>
              <span className="text-[11px] text-text-sub line-through">
                原价 ¥{course.originalPrice}
              </span>
            </div>
          )}
        </div>
        {course.isPurchased && course.type !== "live" ? (
          <button
            onClick={() => navigate(`/course/${id}/play`)}
            className="bg-blue-600 hover:bg-blue-700 text-white font-medium px-6 py-2.5 rounded-full active:scale-95 transition-all text-[14px]"
          >
            继续学习
          </button>
        ) : course.isPurchased && course.type === "live" ? (
          <button
            onClick={() => navigate(`/course/${id}/live`)}
            className="bg-red-500 hover:bg-red-600 text-white font-medium px-6 py-2.5 rounded-full active:scale-95 transition-all text-[14px]"
          >
            {course.liveStatus === "live" ? "进入直播间" : "已预约 / 进入"}
          </button>
        ) : (
          <button
            onClick={() => navigate(`/course/${id}/purchase`)}
            className="bg-blue-600 hover:bg-blue-700 text-white font-medium px-6 py-2.5 rounded-full active:scale-95 transition-all text-[14px] flex items-center gap-1.5 shadow-blue-500/20 shadow-lg"
          >
            {course.type === "live" ? "立即报名 / 预约" : "解锁完整课程"}
          </button>
        )}
      </div>
    </div>
  );
};
