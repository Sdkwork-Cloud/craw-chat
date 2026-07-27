import { useTranslation } from "react-i18next";
import React from "react";
import type { Message } from "@sdkwork/im-h5-types";

export const TextMessage = ({ msg }: { msg: Message }) => {
  const { t } = useTranslation();
  return (
  <span className="whitespace-pre-wrap">{msg.content}</span>
);
};

