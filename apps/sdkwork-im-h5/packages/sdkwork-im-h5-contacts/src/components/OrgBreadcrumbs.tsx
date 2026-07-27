import React from "react";
import type { OrgDepartment, Organization } from "../services/OrganizationService";

interface OrgBreadcrumbsProps {
  org: Organization | null;
  path: OrgDepartment[];
  onGoToRoot: () => void;
  onGoToDept: (id: string) => void;
}

export const OrgBreadcrumbs: React.FC<OrgBreadcrumbsProps> = ({
  org,
  path,
  onGoToRoot,
  onGoToDept,
}) => {
  return (
    <div className="px-4 py-3 bg-bg-color border-b border-border-color overflow-x-auto whitespace-nowrap no-scrollbar flex items-center gap-1.5 shrink-0">
      <span
        className="text-[14px] text-primary-blue cursor-pointer"
        onClick={onGoToRoot}
      >
        {org?.name}
      </span>
      {path.map((p, idx) => (
        <React.Fragment key={p.id}>
          <span className="text-text-sub/50 text-[12px]">/</span>
          <span
            className={`text-[14px] cursor-pointer ${
              idx === path.length - 1
                ? "text-text-main font-medium"
                : "text-primary-blue"
            }`}
            onClick={() => onGoToDept(p.id)}
          >
            {p.name}
          </span>
        </React.Fragment>
      ))}
    </div>
  );
};
