import React, { useEffect, useState } from "react";
import { AlertCircle, ChevronLeft, Loader2, Video } from "lucide-react";
import QRCode from "react-qr-code";
import { useNavigate, useParams } from "react-router";
import { useTranslation } from "react-i18next";

import {
  notaryService,
  type NotaryPartyVideoInvite,
} from "../services/notaryService";

export const NotaryPartyVideoQR: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { caseId, partyId } = useParams();
  const [invite, setInvite] = useState<NotaryPartyVideoInvite | null>(null);
  const [loadError, setLoadError] = useState(false);

  useEffect(() => {
    let active = true;
    if (!caseId || !partyId) {
      setLoadError(true);
      return () => {
        active = false;
      };
    }
    void notaryService.createPartyVideoInvite(caseId, partyId).then(
      (value) => {
        if (active) {
          setInvite(value);
        }
      },
      () => {
        if (active) {
          setLoadError(true);
        }
      },
    );
    return () => {
      active = false;
    };
  }, [caseId, partyId]);

  return (
    <div className="flex h-full flex-col bg-bg-color text-text-main">
      <header className="flex h-[56px] shrink-0 items-center border-b border-border-color px-2 pt-safe">
        <button
          type="button"
          className="flex h-10 w-10 items-center justify-center"
          onClick={() => navigate(-1)}
          aria-label={t("common.back", "Back")}
        >
          <ChevronLeft className="h-6 w-6" />
        </button>
        <h1 className="flex-1 pr-10 text-center text-[17px] font-semibold">
          {t("notary.video_call.qr_title", "Video invitation")}
        </h1>
      </header>

      <main className="flex flex-1 items-center justify-center p-6">
        <div className="flex w-full max-w-sm flex-col items-center text-center">
          <Video className="mb-4 h-10 w-10 text-primary-blue" />
          {!invite && !loadError && (
            <Loader2 className="h-6 w-6 animate-spin text-primary-blue" />
          )}
          {loadError && (
            <>
              <AlertCircle className="mb-3 h-7 w-7 text-red-500" />
              <p className="text-[14px] text-text-sub">
                {t("notary.video_call.invite_failed", "Unable to create the video invitation")}
              </p>
            </>
          )}
          {invite && (
            <>
              <div className="mb-5 bg-white p-4">
                <QRCode value={invite.inviteUrl} size={200} level="H" />
              </div>
              <p className="text-[13px] text-text-sub">
                {t("notary.video_call.expires_at", "Expires at {{time}}", {
                  time: new Date(invite.expiresAt).toLocaleString(),
                })}
              </p>
            </>
          )}
        </div>
      </main>
    </div>
  );
};
