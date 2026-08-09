import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { Avatar, cn, showToast } from "@sdkwork/im-h5-commons";

import { ContactService, type Contact } from "../services/ContactService";

export const ContactRow: React.FC<{
  contact: Contact;
  isLast: boolean;
}> = ({ contact, isLast }) => {
  const navigate = useNavigate();
  const { t } = useTranslation("contacts");
  const [opening, setOpening] = useState(false);

  return (
    <button
      type="button"
      disabled={opening}
      className="flex w-full cursor-pointer items-center bg-bg-color pl-4 pr-3 text-left transition-colors active:bg-active-bg disabled:cursor-wait disabled:opacity-70"
      onClick={async () => {
        if (opening) {
          return;
        }
        setOpening(true);
        try {
          const conversationId = contact.conversationId
            ?? await ContactService.startDirectConversation(contact.id);
          navigate(`/chat/${conversationId}`);
        } catch (error) {
          console.error(error);
          showToast(t("open_conversation_failed"));
          setOpening(false);
        }
      }}
    >
      <div className="my-2 mr-3.5">
        <Avatar
          src={contact.avatar}
          alt={contact.name}
          fallback={contact.name}
          size="md"
          className="rounded-md"
        />
      </div>
      <div
        className={cn(
          "flex min-h-[56px] flex-1 items-center",
          !isLast && "border-b border-border-color/50",
        )}
      >
        <span className="text-[16px] text-text-main">{contact.name}</span>
      </div>
    </button>
  );
};
