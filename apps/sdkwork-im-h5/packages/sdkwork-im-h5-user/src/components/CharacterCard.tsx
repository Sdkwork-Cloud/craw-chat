import React from "react";
import { Character } from "../services/CharacterService";

export const CharacterCard: React.FC<{
  character: Character;
  onClick: () => void;
  onLongPressProps?: any;
}> = ({ character, onClick, onLongPressProps }) => {
  return (
    <div 
      className="bg-white dark:bg-[#1A1A1A] px-4 py-3.5 flex items-center gap-4 border-b border-border-color last:border-b-0 active:bg-active-bg transition-colors cursor-pointer select-none touch-callout-none"
      onClick={onClick}
      {...onLongPressProps}
    >
      <img
        src={character.avatar}
        className="w-12 h-12 rounded-full object-cover shrink-0 border border-border-color/50 pointer-events-none"
        alt="character"
      />
      <div className="flex-1 min-w-0 pointer-events-none">
        <h3 className="text-[16px] font-medium text-text-main truncate">
          {character.name}
        </h3>
        <p className="text-[13px] text-text-sub truncate mt-0.5">{character.desc}</p>
      </div>
    </div>
  );
};
