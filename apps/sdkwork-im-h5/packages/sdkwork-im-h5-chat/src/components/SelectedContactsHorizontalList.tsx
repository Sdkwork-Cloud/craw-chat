import { useTranslation } from "react-i18next";
import React from "react";
import { Avatar } from "@sdkwork/im-h5-commons";
import { User } from "@sdkwork/im-h5-types";

interface SelectedContactsHorizontalListProps {
  selectedIds: Set<string>;
  contacts: User[];
  toggleSelection: (id: string, disabled: boolean) => void;
}

export const SelectedContactsHorizontalList: React.FC<SelectedContactsHorizontalListProps> = ({
  selectedIds,
  contacts,
  toggleSelection,
}) => {
  const { t } = useTranslation();
if (selectedIds.size === 0) return null;

  return (
    <div className="flex gap-3 px-4 py-3 overflow-x-auto no-scrollbar border-b border-border-color bg-chat-other-bg shrink-0">
      {Array.from(selectedIds).map((id: string) => {
        const contact = contacts.find((c) => c.id === id);
        if (!contact) return null;
        return (
          <div
            key={id}
            className="relative shrink-0 animate-in fade-in zoom-in duration-200"
            onClick={() => toggleSelection(id, false)}
          >
            <Avatar src={contact.avatar} size="md" className="w-12 h-12" />
            <div className="absolute -top-1 -right-1 w-4 h-4 bg-bg-color rounded-full flex items-center justify-center border border-border-color shadow-sm">
              <div className="w-2.5 h-[1.5px] bg-text-sub rotate-45 absolute" />
              <div className="w-2.5 h-[1.5px] bg-text-sub -rotate-45 absolute" />
            </div>
          </div>
        );
      })}
    </div>
  );
};
