import { useTranslation } from "react-i18next";
import React from "react";
import { motion, AnimatePresence } from "motion/react";

const INDEX_ALPHABET = [
  "↑",
  "☆",
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
  "#",
];

interface AlphabetIndexProps {
  searchQuery: string;
  activeLetter: string | null;
  handleIndexClick: (letter: string) => void;
}

export const AlphabetIndex: React.FC<AlphabetIndexProps> = ({
  searchQuery,
  activeLetter,
  handleIndexClick,
}) => {
  const { t } = useTranslation();
return (
    <>
      {/* Right Alphabet Index */}
      {!searchQuery && (
        <div className="absolute right-0 top-[150px] bottom-0 flex flex-col items-center justify-center w-6 z-30 font-sans pb-10">
          {INDEX_ALPHABET.map((letter) => (
            <div
              key={letter}
              className="text-[10px] h-[16px] flex items-center justify-center text-text-sub/80 cursor-pointer w-full hover:bg-black/10 dark:hover:bg-white/10"
              onClick={() => handleIndexClick(letter)}
            >
              {letter}
            </div>
          ))}
        </div>
      )}

      {/* Center Letter Indicator (Pop-up) */}
      <AnimatePresence>
        {activeLetter && (
          <motion.div
            initial={{ opacity: 0, scale: 0.8 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.8 }}
            className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-16 h-16 bg-black/60 backdrop-blur-md rounded-xl flex items-center justify-center z-50 shadow-2xl pointer-events-none"
          >
            <span className="text-white text-3xl font-bold">
              {activeLetter}
            </span>
          </motion.div>
        )}
      </AnimatePresence>
    </>
  );
};
