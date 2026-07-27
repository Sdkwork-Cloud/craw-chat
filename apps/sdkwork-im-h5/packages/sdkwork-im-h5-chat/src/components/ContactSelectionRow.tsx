import React from "react";
import { Check } from "lucide-react";
import { Avatar, cn } from "@sdkwork/im-h5-commons";
import type { User } from "@sdkwork/im-h5-types";

interface ContactSelectionRowProps {
  contact: User;
  isSelected: boolean;
  isDisabled: boolean;
  onToggle: (id: string, disabled: boolean) => void;
}

export const ContactSelectionRow: React.FC<ContactSelectionRowProps> = ({
  contact,
  isSelected,
  isDisabled,
  onToggle,
}) => {
  return (
    <div
      key={contact.id}
      onClick={() => onToggle(contact.id, isDisabled)}
      className={cn(
        "flex items-center gap-3 px-4 transition-colors cursor-pointer",
        isDisabled ? "opacity-50 cursor-not-allowed" : "active:bg-active-bg"
      )}
    >
      <div
        className={cn(
          "w-5 h-5 rounded-full border flex items-center justify-center shrink-0 transition-colors my-3",
          isSelected
            ? "bg-primary-blue border-primary-blue"
            : "border-text-sub/50",
          isDisabled && "bg-text-sub border-text-sub"
        )}
      >
        {isSelected && (
          <Check className="w-3.5 h-3.5 text-white" strokeWidth={3} />
        )}
      </div>
      <div className="flex-1 flex items-center gap-3 min-h-[56px] border-b border-border-color/50 py-2">
        <Avatar src={contact.avatar} size="md" />
        <span className="text-[16px] text-text-main font-medium flex-1">
          {contact.name}
        </span>
      </div>
    </div>
  );
};
