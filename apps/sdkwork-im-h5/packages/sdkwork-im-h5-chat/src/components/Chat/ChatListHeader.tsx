import React, { RefObject } from "react";
import { useNavigate } from "react-router";
import { Search, Contact } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { AddMenu } from "./AddMenu";

interface ChatListHeaderProps {
  menuRef: RefObject<HTMLDivElement | null>;
  isMenuOpen: boolean;
  setIsMenuOpen: (open: boolean) => void;
}

export const ChatListHeader: React.FC<ChatListHeaderProps> = ({
  menuRef,
  isMenuOpen,
  setIsMenuOpen,
}) => {
  const navigate = useNavigate();

  return (
    <header className="h-[56px] px-4 flex items-center justify-between glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
      <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
        <h1 className="text-[17px] font-semibold text-text-main">Sdkwork IM H5</h1>
      </div>
      <div className="flex-1" />
      <div className="flex gap-2 relative z-10" ref={menuRef}>
        <IconButton
          icon={<Search className="w-5 h-5 text-text-main" />}
          className="bg-black/5 dark:bg-white/5 w-8 h-8 p-0"
          onClick={() => navigate("/search")}
        />
        <IconButton
          icon={<Contact className="w-5 h-5 text-text-main" />}
          className="bg-black/5 dark:bg-white/5 w-8 h-8 p-0"
          onClick={() => navigate("/workspace/contacts")}
        />
        <AddMenu isMenuOpen={isMenuOpen} setIsMenuOpen={setIsMenuOpen} />
      </div>
    </header>
  );
};
