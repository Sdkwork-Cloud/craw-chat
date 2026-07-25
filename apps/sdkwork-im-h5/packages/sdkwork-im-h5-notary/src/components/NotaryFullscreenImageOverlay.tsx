import React from "react";
import { X } from "lucide-react";

interface NotaryFullscreenImageOverlayProps {
  imageUrl: string | null;
  onClose: () => void;
}

export const NotaryFullscreenImageOverlay: React.FC<
  NotaryFullscreenImageOverlayProps
> = ({ imageUrl, onClose }) => {
  if (!imageUrl) return null;

  return (
    <div
      className="fixed inset-0 z-[200] bg-black/90 flex flex-col animate-in fade-in cursor-pointer"
      onClick={onClose}
    >
      <div className="h-14 flex items-center justify-end px-4 pt-safe safe-area-top">
        <button
          className="text-white p-2"
          onClick={(e) => {
            e.stopPropagation();
            onClose();
          }}
        >
          <X className="w-8 h-8" />
        </button>
      </div>
      <div className="flex-1 flex items-center justify-center p-4">
        <img
          src={imageUrl}
          alt="Preview"
          className="max-w-full max-h-full object-contain"
        />
      </div>
    </div>
  );
};
