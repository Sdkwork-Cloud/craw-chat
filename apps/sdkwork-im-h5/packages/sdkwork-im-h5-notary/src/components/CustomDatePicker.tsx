import React, { useState, useEffect } from "react";
import { cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

export const CustomDatePicker = ({
  initialValue,
  onChange,
  defaultYearOffset = -30,
}: {
  initialValue: string;
  onChange: (date: string) => void;
  defaultYearOffset?: number;
}) => {
  const { t } = useTranslation();
  const [tempDate, setTempDate] = useState(() => {
    if (initialValue) {
      const [y, m, d] = initialValue.split("-");
      return { year: parseInt(y), month: parseInt(m) || 1, day: parseInt(d) || 1 };
    }
    const currentYear = new Date().getFullYear();
    return { year: currentYear + defaultYearOffset, month: 1, day: 1 };
  });

  const getDaysInMonth = (year: number, month: number) => {
  return new Date(year, month, 0).getDate();
  };

  useEffect(() => {
    const formatted = `${tempDate.year}-${String(tempDate.month).padStart(2, "0")}-${String(tempDate.day).padStart(2, "0")}`;
    onChange(formatted);
  }, [tempDate]);

  return (
    <>
      <div className="flex h-[350px] relative px-4 w-full max-w-md mx-auto">
        {/* Selection Highlight */}
        <div className="absolute top-1/2 -mt-6 h-12 left-4 right-4 bg-primary-blue/5 border-y border-primary-blue/20 pointer-events-none rounded-lg" />

        {/* Year */}
        <div
          className="flex-1 h-full overflow-y-auto no-scrollbar relative snap-y snap-mandatory py-[151px]"
          style={{ scrollBehavior: "smooth" }}
        >
          {Array.from(
            { length: 62 },
            (_, i) => new Date().getFullYear() + 31 - i, // +31 years to -30 years
          ).map((y) => (
            <div
              key={y}
              onClick={() => setTempDate((prev) => ({ ...prev, year: y }))}
              className={cn(
                "h-12 flex items-center justify-center text-[16px] snap-center cursor-pointer",
                tempDate.year === y
                  ? "font-bold text-primary-blue scale-110 shadow-sm"
                  : "font-medium text-text-sub opacity-70",
              )}
            >
              {y}{t("notary.picker.year")}
            </div>
          ))}
        </div>

        {/* Month */}
        <div
          className="flex-1 h-full overflow-y-auto no-scrollbar relative snap-y snap-mandatory py-[151px]"
          style={{ scrollBehavior: "smooth" }}
        >
          {Array.from({ length: 12 }, (_, i) => i + 1).map((m) => (
            <div
              key={m}
              onClick={() =>
                setTempDate((prev) => ({
                  ...prev,
                  month: m,
                  day: Math.min(prev.day, getDaysInMonth(prev.year, m)),
                }))
              }
              className={cn(
                "h-12 flex items-center justify-center text-[16px] snap-center cursor-pointer",
                tempDate.month === m
                  ? "font-bold text-primary-blue scale-110 shadow-sm"
                  : "font-medium text-text-sub opacity-70",
              )}
            >
              {m}{t("notary.picker.month")}
            </div>
          ))}
        </div>

        {/* Day */}
        <div
          className="flex-1 h-full overflow-y-auto no-scrollbar relative snap-y snap-mandatory py-[151px]"
          style={{ scrollBehavior: "smooth" }}
        >
          {Array.from(
            { length: getDaysInMonth(tempDate.year, tempDate.month) },
            (_, i) => i + 1,
          ).map((d) => (
            <div
              key={d}
              onClick={() => setTempDate((prev) => ({ ...prev, day: d }))}
              className={cn(
                "h-12 flex items-center justify-center text-[16px] snap-center cursor-pointer",
                tempDate.day === d
                  ? "font-bold text-primary-blue scale-110 shadow-sm"
                  : "font-medium text-text-sub opacity-70",
              )}
            >
              {d}{t("notary.picker.day")}
            </div>
          ))}
        </div>
      </div>
    </>
  );
};
