import React from "react";
import { Avatar } from "@sdkwork/im-h5-commons";

interface MomentsHeaderCoverProps {
  name: string;
  avatarUrl: string;
  coverUrl: string;
}

export const MomentsHeaderCover: React.FC<MomentsHeaderCoverProps> = ({
  name,
  avatarUrl,
  coverUrl,
}) => {
  return (
    <>
      <div className="relative h-[300px] w-full bg-gray-200 dark:bg-gray-800 shrink-0">
        <img
          src={coverUrl}
          alt="Cover"
          className="w-full h-full object-cover"
        />
        <div className="absolute top-0 left-0 right-0 h-24 bg-gradient-to-b from-black/40 to-transparent pointer-events-none" />

        <div className="absolute -bottom-6 right-4 flex items-end gap-4 z-10">
          <span className="text-white font-bold text-[20px] mb-3 drop-shadow-[0_2px_4px_rgba(0,0,0,0.5)]">
            {name}
          </span>
          <Avatar
            src={avatarUrl}
            size="lg"
            className="w-[72px] h-[72px] rounded-xl border-2 border-white dark:border-[#1C1C1E] bg-chat-other-bg shadow-sm"
          />
        </div>
      </div>
      <div className="h-10 w-full bg-chat-other-bg" />
    </>
  );
};
