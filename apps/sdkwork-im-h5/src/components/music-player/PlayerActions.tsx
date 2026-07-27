import React from "react";
import { ListMusic, Laptop } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";

interface PlayerActionsProps {
  onDevicesClick?: () => void;
  onPlaylistClick?: () => void;
}

export const PlayerActions: React.FC<PlayerActionsProps> = ({
  onDevicesClick,
  onPlaylistClick,
}) => {
  return (
    <div className="flex items-center justify-between px-2 pt-4 opacity-80">
      <IconButton
        icon={<Laptop className="w-5 h-5 text-white" />}
        onClick={onDevicesClick || (() => {})}
      />
      <IconButton
        icon={<ListMusic className="w-6 h-6 text-white" />}
        onClick={onPlaylistClick || (() => {})}
      />
    </div>
  );
};
