import React from "react";
import { Avatar } from "@sdkwork/im-h5-commons";
import type { OrgMember } from "../services/OrganizationService";

interface OrgMemberListProps {
  members: OrgMember[];
  t: (key: string, options?: any) => string;
}

export const OrgMemberList: React.FC<OrgMemberListProps> = ({ members, t }) => {
  if (members.length === 0) return null;

  return (
    <div className="bg-bg-color mt-2">
      <div className="px-4 py-2 border-b border-border-color">
        <span className="text-[13px] text-text-sub">
          {t("contacts.org_members")} ({members.length})
        </span>
      </div>
      {members.map((member, index) => (
        <div key={member.id} className="relative">
          <div className="flex items-center gap-3 px-4 py-3 cursor-pointer active:bg-black/5 dark:active:bg-white/5">
            <Avatar
              src={member.avatar || ""}
              alt={member.name}
              fallback={member.name}
              size="md"
              className="rounded"
            />
            <div className="flex flex-col flex-1 min-w-0 justify-center">
              <span className="text-[16px] text-text-main truncate font-medium">
                {member.name}
              </span>
              {member.jobTitle && (
                <span className="text-[13px] text-text-sub truncate mt-0.5">
                  {member.jobTitle}
                </span>
              )}
            </div>
          </div>
          {index < members.length - 1 && (
            <div className="ml-16 border-b border-border-color" />
          )}
        </div>
      ))}
    </div>
  );
};
