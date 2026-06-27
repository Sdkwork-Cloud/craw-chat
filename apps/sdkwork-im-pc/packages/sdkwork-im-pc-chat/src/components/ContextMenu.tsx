import React, { useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { cn } from '@sdkwork/im-pc-commons';

export interface ContextMenuItem {
  id: string;
  label: string;
  icon?: React.ReactNode;
  danger?: boolean;
  divider?: boolean;
  onClick: () => void;
}

interface ContextMenuProps {
  x: number;
  y: number;
  items: ContextMenuItem[];
  onClose: () => void;
}

export const ContextMenu: React.FC<ContextMenuProps> = ({ x, y, items, onClose }) => {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        onClose();
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    // Prevent default context menu on the custom context menu itself
    const handleContextMenu = (e: MouseEvent) => e.preventDefault();
    if (menuRef.current) {
      menuRef.current.addEventListener('contextmenu', handleContextMenu);
    }
    
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
      if (menuRef.current) {
        menuRef.current.removeEventListener('contextmenu', handleContextMenu);
      }
    };
  }, [onClose]);

  // Adjust position to prevent going off-screen
  const menuWidth = 180;
  const menuHeight = items.length * 36 + 16; // Approximate height
  
  const adjustedX = Math.min(x, window.innerWidth - menuWidth - 10);
  const adjustedY = Math.min(y, window.innerHeight - menuHeight - 10);

  const style: React.CSSProperties = {
    top: adjustedY,
    left: adjustedX,
  };

  return (
    <AnimatePresence>
      <motion.div
        ref={menuRef}
        initial={{ opacity: 0, scale: 0.95, transformOrigin: 'top left' }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 0.95 }}
        transition={{ duration: 0.1 }}
        style={style}
        className="fixed z-[9999] w-[180px] bg-[#2b2b2d] border border-white/10 rounded-lg shadow-2xl py-1.5"
      >
        {items.map((item, index) => (
          item.divider ? (
            <div key={`div-${index}`} className="h-px bg-white/10 my-1.5 mx-3" />
          ) : (
            <button
              key={item.id}
              onClick={(e) => {
                e.stopPropagation();
                item.onClick();
                onClose();
              }}
              className={cn(
                "w-full flex items-center px-4 py-2 text-[13px] hover:bg-white/10 transition-colors",
                item.danger ? "text-red-400 hover:text-red-300" : "text-gray-200"
              )}
            >
              {item.icon && <span className="mr-3 opacity-70">{item.icon}</span>}
              {item.label}
            </button>
          )
        ))}
      </motion.div>
    </AnimatePresence>
  );
};
