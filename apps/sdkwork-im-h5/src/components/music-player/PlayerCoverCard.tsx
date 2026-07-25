import React from "react";
import { motion } from "motion/react";

interface PlayerCoverCardProps {
  coverUrl: string;
  isPlaying: boolean;
}

export const PlayerCoverCard: React.FC<PlayerCoverCardProps> = ({
  coverUrl,
  isPlaying,
}) => {
  return (
    <motion.div
      className="w-full aspect-square max-w-[320px] rounded-3xl overflow-hidden shadow-2xl mx-auto"
      animate={{ scale: isPlaying ? 1 : 0.95 }}
      transition={{ type: "spring", bounce: 0.4 }}
    >
      <img
        src={coverUrl}
        alt="Cover"
        className="w-full h-full object-cover"
      />
    </motion.div>
  );
};
