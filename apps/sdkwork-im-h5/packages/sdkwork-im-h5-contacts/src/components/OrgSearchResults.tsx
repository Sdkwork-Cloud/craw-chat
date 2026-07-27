import React from "react";
import { Search } from "lucide-react";
import { Avatar } from "@sdkwork/im-h5-commons";
import type { OrgMember } from "../services/OrganizationService";

interface OrgSearchResultsProps {
  searchResults: OrgMember[];
  t: (key: string, options?: any) => string;
}

export const OrgSearchResults: React.FC<OrgSearchResultsProps> = ({
  searchResults,
  t,
}) => {
  return (
    <div className="flex flex-col bg-bg-color">
      {searchResults.length > 0 ? (
        searchResults.map((member, index) => (
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
            {index < searchResults.length - 1 && (
              <div className="ml-16 border-b border-border-color" />
            )}
          </div>
        ))
      ) : (
        <div className="flex flex-col items-center justify-center py-20 text-text-sub gap-2">
          <Search className="w-12 h-12 text-text-sub/30" />
          <span className="text-[14px]">
            {t("contacts.no_search_results", { defaultValue: "No results found" })}
          </span>
        </div>
      )}
    </div>
  );
};
