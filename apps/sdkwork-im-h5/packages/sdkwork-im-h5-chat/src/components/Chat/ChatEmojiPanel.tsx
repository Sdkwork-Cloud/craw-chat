import { useTranslation } from "react-i18next";
import React from "react";
import { motion } from "motion/react";
import { Editor } from "@tiptap/react";

interface ChatEmojiPanelProps {
  emojis: string[];
  editor: Editor | null;
}

export const ChatEmojiPanel: React.FC<ChatEmojiPanelProps> = ({
  emojis,
  editor,
}) => {
  const { t } = useTranslation();
return (
    <motion.div
      initial={{ height: 0, opacity: 0 }}
      animate={{ height: 256, opacity: 1 }}
      exit={{ height: 0, opacity: 0 }}
      transition={{ duration: 0.2, ease: "easeOut" }}
      className="bg-input-bg border-t border-border-color overflow-y-auto"
    >
      <div className="grid grid-cols-8 gap-4 p-4 h-64">
        {emojis.map((e) => (
          <span
            key={e}
            className="text-2xl cursor-pointer hover:bg-chat-active-bg rounded p-1 flex items-center justify-center transition-colors"
            onClick={() => editor?.commands.insertContent(e)}
          >
            {e}
          </span>
        ))}
      </div>
    </motion.div>
  );
};
