import React, { useEffect, useState } from "react";
import { AlertCircle, ChevronLeft, Loader2, ShieldCheck, Video } from "lucide-react";
import { useNavigate, useParams } from "react-router";
import { useTranslation } from "react-i18next";

import {
  notaryService,
  type NotaryPartyVideoInvite,
} from "../services/notaryService";

export const NotaryVideoCall: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { caseId, partyId } = useParams();
  const [invite, setInvite] = useState<NotaryPartyVideoInvite | null>(null);
  const [loadError, setLoadError] = useState(false);

  useEffect(() => {
    let active = true;
    setInvite(null);
    setLoadError(false);
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
    <div className="fixed inset-0 z-[200] flex flex-col bg-bg-color text-text-main">
      <header className="glass-header flex h-[56px] shrink-0 items-center border-b border-border-color px-2 pt-safe">
        <button
          type="button"
          className="flex h-10 w-10 items-center justify-center"
          onClick={() => navigate(-1)}
          aria-label={t("common.back", "Back")}
        >
          <ChevronLeft className="h-6 w-6" />
        </button>
        <h1 className="flex-1 pr-10 text-center text-[17px] font-semibold">
          {t("notary.video_call.title", "Video verification")}
        </h1>
      </header>

      <main className="flex flex-1 items-center justify-center p-6">
        <div className="flex w-full max-w-md flex-col items-center text-center">
          <div className="mb-5 flex h-16 w-16 items-center justify-center rounded-full bg-primary-blue/10 text-primary-blue">
            <ShieldCheck className="h-8 w-8" />
          </div>
          {!invite && !loadError && (
            <>
              <Loader2 className="mb-3 h-6 w-6 animate-spin text-primary-blue" />
              <p className="text-[14px] text-text-sub">
                {t("notary.video_call.creating_invite", "Creating a secure video invitation...")}
              </p>
            </>
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
              <h2 className="mb-2 text-[18px] font-semibold">
                {t("notary.video_call.invite_ready", "Secure invitation ready")}
              </h2>
              <p className="mb-6 text-[13px] text-text-sub">
                {t("notary.video_call.expires_at", "Expires at {{time}}", {
                  time: new Date(invite.expiresAt).toLocaleString(),
                })}
              </p>
              <button
                type="button"
                className="flex h-12 w-full items-center justify-center gap-2 rounded-lg bg-primary-blue px-4 font-semibold text-white"
                onClick={() => window.location.assign(invite.inviteUrl)}
              >
                <Video className="h-5 w-5" />
                {t("notary.video_call.continue", "Continue to video verification")}
              </button>
            </>
          )}
        </div>
      </main>
    </div>
  );
};
