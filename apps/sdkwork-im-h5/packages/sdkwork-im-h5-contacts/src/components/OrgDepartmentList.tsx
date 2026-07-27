import React from "react";
import { Folder, ChevronLeft } from "lucide-react";
import type { OrgDepartment } from "../services/OrganizationService";

interface OrgDepartmentListProps {
  departments: OrgDepartment[];
  t: (key: string, options?: any) => string;
  onGoToDept: (id: string) => void;
}

export const OrgDepartmentList: React.FC<OrgDepartmentListProps> = ({
  departments,
  t,
  onGoToDept,
}) => {
  if (departments.length === 0) return null;

  return (
    <div className="bg-bg-color mt-2">
      {departments.map((dept, index) => (
        <div key={dept.id} className="relative">
          <div
            className="flex items-center justify-between px-4 py-3.5 cursor-pointer active:bg-black/5 dark:active:bg-white/5"
            onClick={() => onGoToDept(dept.id)}
          >
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded bg-[#4395F5]/10 flex items-center justify-center shrink-0">
                <Folder className="w-5 h-5 text-[#4395F5] fill-[#4395F5]" />
              </div>
              <span className="text-[16px] text-text-main">{dept.name}</span>
            </div>
            <div className="flex items-center gap-2">
              {dept.count > 0 && (
                <span className="text-[14px] text-text-sub">
                  {t("contacts.people_count", { count: dept.count })}
                </span>
              )}
              <ChevronLeft className="w-5 h-5 text-text-sub/40 rotate-180" />
            </div>
          </div>
          {index < departments.length - 1 && (
            <div className="ml-16 border-b border-border-color" />
          )}
        </div>
      ))}
    </div>
  );
};
