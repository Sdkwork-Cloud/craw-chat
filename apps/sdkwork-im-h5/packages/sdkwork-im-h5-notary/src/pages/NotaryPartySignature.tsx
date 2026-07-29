import React, { useRef, useState } from "react";
import { AlertCircle, Check, ChevronLeft, Loader2, RotateCcw } from "lucide-react";
import SignatureCanvas from "react-signature-canvas";
import { useNavigate, useParams } from "react-router";
import { useTranslation } from "react-i18next";

import { notaryService } from "../services/notaryService";

const SIGNATURE_WIDTH = 1200;
const SIGNATURE_HEIGHT = 600;

export const NotaryPartySignature: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const { caseId, partyId } = useParams();
  const signatureRef = useRef<SignatureCanvas>(null);
  const [submitting, setSubmitting] = useState(false);
  const [submitError, setSubmitError] = useState(false);

  const submitSignature = async () => {
    if (submitting) {
      return;
    }
    if (!caseId || !partyId) {
      setSubmitError(true);
      return;
    }
    const canvas = signatureRef.current;
    if (!canvas || canvas.isEmpty()) {
      setSubmitError(true);
      return;
    }

    setSubmitting(true);
    setSubmitError(false);
    try {
      const blob = await canvasToBlob(canvas.getCanvas());
      const file = new File(
        [blob],
        `party-signature-${partyId}.png`,
        { type: "image/png", lastModified: Date.now() },
      );
      await notaryService.attachPartySignature(caseId, partyId, file);
      navigate(-1);
    } catch {
      setSubmitError(true);
    } finally {
      setSubmitting(false);
    }
  };

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
        <h1 className="flex-1 text-center text-[17px] font-semibold">
          {t("notary.signature.title", "Party signature")}
        </h1>
        <button
          type="button"
          className="flex h-10 w-10 items-center justify-center text-text-sub"
          onClick={() => {
            signatureRef.current?.clear();
            setSubmitError(false);
          }}
          aria-label={t("notary.signature.rewrite", "Clear signature")}
        >
          <RotateCcw className="h-5 w-5" />
        </button>
      </header>

      <main className="flex min-h-0 flex-1 flex-col p-4">
        <div className="min-h-0 flex-1 overflow-hidden rounded-lg border border-border-color bg-white shadow-inner">
          <SignatureCanvas
            ref={signatureRef}
            penColor="#111827"
            minWidth={1.2}
            maxWidth={3}
            velocityFilterWeight={0.7}
            canvasProps={{
              width: SIGNATURE_WIDTH,
              height: SIGNATURE_HEIGHT,
              className: "h-full w-full touch-none",
            }}
          />
        </div>
        {submitError && (
          <div className="mt-3 flex items-center gap-2 text-[13px] text-red-500">
            <AlertCircle className="h-4 w-4 shrink-0" />
            <span>
              {t(
                "notary.signature.submit_failed",
                "Provide a signature and try submitting again.",
              )}
            </span>
          </div>
        )}
      </main>

      <footer className="border-t border-border-color bg-bg-color p-4 pb-safe">
        <button
          type="button"
          disabled={submitting}
          className="flex h-12 w-full items-center justify-center gap-2 rounded-lg bg-primary-blue font-semibold text-white disabled:opacity-50"
          onClick={() => void submitSignature()}
        >
          {submitting ? (
            <Loader2 className="h-5 w-5 animate-spin" />
          ) : (
            <Check className="h-5 w-5" />
          )}
          {t("notary.signature.confirm", "Submit signature")}
        </button>
      </footer>
    </div>
  );
};

function canvasToBlob(canvas: HTMLCanvasElement): Promise<Blob> {
  return new Promise((resolve, reject) => {
    canvas.toBlob((blob) => {
      if (!blob || blob.size === 0) {
        reject(new Error("Signature canvas produced an empty image"));
        return;
      }
      resolve(blob);
    }, "image/png");
  });
}
