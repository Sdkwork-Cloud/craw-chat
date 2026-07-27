import React from "react";
import type { Message } from "@sdkwork/im-h5-types";

export const ImageMessage = ({
  msg,
  onClick,
}: {
  msg: Message;
  onClick: () => void;
}) => (
  <div
    className="overflow-hidden rounded-lg cursor-pointer bg-black/5 dark:bg-white/5"
    onClick={onClick}
  >
    <img
      src={msg.content}
      alt="Image"
      className="max-w-[200px] max-h-[300px] min-w-[100px] min-h-[100px] w-auto h-auto object-cover"
    />
  </div>
);
