import { useTranslation } from "react-i18next";
import React, { useState, useRef, useEffect } from "react";
import { motion, useAnimation, PanInfo } from "motion/react";

interface SwipeableItemProps {
  children: React.ReactNode;
  onEdit?: () => void;
  onDelete?: () => void;
}

export const SwipeableItem: React.FC<SwipeableItemProps> = ({ children, onEdit, onDelete }) => {
  const { t } = useTranslation();
const controls = useAnimation();
  const [isOpen, setIsOpen] = useState(false);
  const swipeRef = useRef<HTMLDivElement>(null);

  // Total width of the action buttons. Adjust if modifying buttons.
  const actionWidth = (onEdit ? 60 : 0) + (onDelete ? 60 : 0);

  const handleDragEnd = (event: MouseEvent | TouchEvent | PointerEvent, info: PanInfo) => {
  const isSwipedLeft = info.offset.x < -actionWidth / 2 || info.velocity.x < -500;
    const isSwipedRight = info.offset.x > actionWidth / 2 || info.velocity.x > 500;

    if (isOpen) {
      if (isSwipedRight) {
        controls.start({ x: 0 });
        setIsOpen(false);
      } else {
        controls.start({ x: -actionWidth });
      }
    } else {
      if (isSwipedLeft) {
        controls.start({ x: -actionWidth });
        setIsOpen(true);
      } else {
        controls.start({ x: 0 });
      }
    }
  };

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent | TouchEvent) => {
  if (isOpen && swipeRef.current && !swipeRef.current.contains(event.target as Node)) {
        controls.start({ x: 0 });
        setIsOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    document.addEventListener("touchstart", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("touchstart", handleClickOutside);
    };
  }, [isOpen, controls]);

  return (
    <div className="relative w-full overflow-hidden" ref={swipeRef}>
        {/* Actions Container - Behind the main content */}
        <div className="absolute top-0 right-0 h-full flex items-center bg-hover-bg">
            {onEdit && (
                <button 
                  onClick={() => {
                      controls.start({ x: 0 });
                      setIsOpen(false);
                      onEdit();
                  }}
                  className="h-full w-[60px] bg-blue-500 text-white flex items-center justify-center text-[15px] font-medium active:bg-blue-600 transition-colors"
                >{t('commons.auto_ff33b', 'Edit')}</button>
            )}
            {onDelete && (
                <button 
                  onClick={() => {
                      controls.start({ x: 0 });
                      setIsOpen(false);
                      onDelete();
                  }}
                  className="h-full w-[60px] bg-red-500 text-white flex items-center justify-center text-[15px] font-medium active:bg-red-600 transition-colors"
                >{t('commons.auto_a8844', 'Delete')}</button>
            )}
        </div>

        {/* Swipeable Main Content */}
        <motion.div
            drag="x"
            dragDirectionLock
            dragConstraints={{ left: -actionWidth, right: 0 }}
            dragElastic={0.1}
            onDragEnd={handleDragEnd}
            animate={controls}
            className="relative z-10 w-full bg-chat-other-bg"
        >
            {children}
        </motion.div>
    </div>
  );
};
