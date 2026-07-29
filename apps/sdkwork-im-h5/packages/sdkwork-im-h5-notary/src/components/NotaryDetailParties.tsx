import React from "react";
import { PenTool, ShieldCheck, Video } from "lucide-react";
import { useTranslation } from "react-i18next";

import type { NotaryParty } from "../services/notaryService";

interface NotaryDetailPartiesProps {
  parties: NotaryParty[];
  isFinalState: boolean;
  onNavigateToSignature: (party: NotaryParty) => void;
  onNavigateToVideo: (party: NotaryParty) => void;
}

export const NotaryDetailParties: React.FC<NotaryDetailPartiesProps> = ({
  parties,
  isFinalState,
  onNavigateToSignature,
  onNavigateToVideo,
}) => {
  const { t } = useTranslation();
  return (
    <div className="flex flex-col bg-[#f4f6f9] dark:bg-black">
      {parties.map((party) => (
        <div
          key={party.id}
          className="flex gap-4 border-b border-border-color/50 bg-bg-color p-4 last:border-0"
        >
          <div className="flex h-[72px] w-[72px] shrink-0 items-center justify-center rounded-lg border border-border-color/50 bg-chat-other-bg text-[22px] font-semibold text-text-sub">
            {party.name.trim().charAt(0).toUpperCase()}
          </div>
          <div className="min-w-0 flex-1">
            <div className="flex items-start justify-between gap-2">
              <span className="truncate text-[17px] font-semibold text-text-main">
                {party.name}
              </span>
              <span className="flex shrink-0 items-center gap-1 rounded border border-green-500/30 bg-green-500/10 px-1.5 py-0.5 text-[11px] font-medium text-green-600">
                <ShieldCheck className="h-3 w-3" />
                {t(`notary.party_status.${party.status}`, party.status)}
              </span>
            </div>
            <p className="mt-1 text-[13px] text-text-sub">{party.role}</p>
            {!isFinalState && (
              <div className="mt-3 flex justify-end gap-2">
                <button
                  type="button"
                  className="flex h-8 items-center gap-1.5 rounded-lg bg-orange-500/10 px-3 text-[13px] font-semibold text-orange-600"
                  onClick={() => onNavigateToSignature(party)}
                >
                  <PenTool className="h-4 w-4" />
                  {t("notary.party.signature", "Signature")}
                </button>
                <button
                  type="button"
                  className="flex h-8 items-center gap-1.5 rounded-lg bg-primary-blue px-3 text-[13px] font-semibold text-white"
                  onClick={() => onNavigateToVideo(party)}
                >
                  <Video className="h-4 w-4" />
                  {t("notary.party.video", "Video")}
                </button>
              </div>
            )}
          </div>
        </div>
      ))}
    </div>
  );
};
