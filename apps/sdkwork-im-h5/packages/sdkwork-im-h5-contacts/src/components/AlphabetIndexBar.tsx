import React from "react";

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

interface AlphabetIndexBarProps {
  onIndexClick: (letter: string) => void;
}

export const AlphabetIndexBar: React.FC<AlphabetIndexBarProps> = ({
  onIndexClick,
}) => {
  return (
    <div className="absolute right-0 top-1/2 -translate-y-1/2 flex flex-col items-center justify-center w-6 z-30 pt-safe font-sans pb-10">
      {INDEX_ALPHABET.map((letter) => (
        <div
          key={letter}
          className="text-[10px] h-[16px] flex items-center justify-center text-text-sub/80 cursor-pointer w-full hover:bg-black/10 dark:hover:bg-white/10"
          onClick={() => onIndexClick(letter)}
        >
          {letter}
        </div>
      ))}
    </div>
  );
};
