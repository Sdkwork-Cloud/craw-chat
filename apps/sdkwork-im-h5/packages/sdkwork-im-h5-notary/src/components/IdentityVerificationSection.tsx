import React from "react";
import { Plus, X, ScanFace, CheckCircle2 } from "lucide-react";
import { cn, showToast } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

interface IdentityVerificationSectionProps {
  isReadonly: boolean;
  idFrontRef: React.RefObject<HTMLInputElement>;
  idBackRef: React.RefObject<HTMLInputElement>;
  faceRef: React.RefObject<HTMLInputElement>;
  idFrontPreview: string | null;
  idBackPreview: string | null;
  facePreview: string | null;
  faceScore: number | null;
  isScanning: boolean;
  setIdFrontPreview: React.Dispatch<React.SetStateAction<string | null>>;
  setIdBackPreview: React.Dispatch<React.SetStateAction<string | null>>;
  setFacePreview: React.Dispatch<React.SetStateAction<string | null>>;
  setFaceScore: React.Dispatch<React.SetStateAction<number | null>>;
  setFullscreenImage: (url: string | null) => void;
  handleFileChange: (
    e: React.ChangeEvent<HTMLInputElement>,
    setter: React.Dispatch<React.SetStateAction<string | null>>,
    existingUrl: string | null,
    side?: "front" | "back",
  ) => void;
  handleFaceChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleStartComparison: () => void;
}

export const IdentityVerificationSection: React.FC<
  IdentityVerificationSectionProps
> = ({
  isReadonly,
  idFrontRef,
  idBackRef,
  faceRef,
  idFrontPreview,
  idBackPreview,
  facePreview,
  faceScore,
  isScanning,
  setIdFrontPreview,
  setIdBackPreview,
  setFacePreview,
  setFaceScore,
  setFullscreenImage,
  handleFileChange,
  handleFaceChange,
  handleStartComparison,
}) => {
  const { t } = useTranslation();
return (
    <div className="bg-bg-color px-4 py-4 mb-2 flex flex-col gap-4">
      <input
        type="file"
        accept="image/*"
        className="hidden"
        ref={idFrontRef}
        onChange={(e) =>
          handleFileChange(e, setIdFrontPreview, idFrontPreview, "front")
        }
      />
      <input
        type="file"
        accept="image/*"
        className="hidden"
        ref={idBackRef}
        onChange={(e) =>
          handleFileChange(e, setIdBackPreview, idBackPreview, "back")
        }
      />
      <input
        type="file"
        accept="image/*"
        capture="user"
        className="hidden"
        ref={faceRef}
        onChange={handleFaceChange}
      />

      {/* ID Cards Row: 2 columns */}
      <div className="grid grid-cols-2 gap-3">
        <div
          onClick={() =>
            idFrontPreview
              ? setFullscreenImage(idFrontPreview)
              : idFrontRef.current?.click()
          }
          className="aspect-[4/3] bg-input-bg rounded-xl border border-dashed border-border-color flex flex-col items-center justify-center cursor-pointer active:bg-active-bg transition-colors relative overflow-hidden group"
        >
          {idFrontPreview ? (
            <>
              <img
                src={idFrontPreview}
                alt={t("notary.verify.id_front")}
                className="w-full h-full object-contain bg-black/5 dark:bg-white/5"
              />
              {!isReadonly && (
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    setIdFrontPreview(null);
                  }}
                  className="absolute top-2 right-2 w-6 h-6 bg-black/50 rounded-full flex items-center justify-center text-white"
                >
                  <X className="w-4 h-4" />
                </button>
              )}
            </>
          ) : (
            <>
              <Plus className="w-6 h-6 text-text-sub mb-1 opacity-70 group-active:scale-110 transition-transform" />
              <span className="text-[12px] text-text-sub">{t("notary.verify.id_front")}</span>
            </>
          )}
        </div>
        <div
          onClick={() =>
            idBackPreview
              ? setFullscreenImage(idBackPreview)
              : idBackRef.current?.click()
          }
          className="aspect-[4/3] bg-input-bg rounded-xl border border-dashed border-border-color flex flex-col items-center justify-center cursor-pointer active:bg-active-bg transition-colors relative overflow-hidden group"
        >
          {idBackPreview ? (
            <>
              <img
                src={idBackPreview}
                alt={t("notary.verify.id_back")}
                className="w-full h-full object-contain bg-black/5 dark:bg-white/5"
              />
              {!isReadonly && (
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    setIdBackPreview(null);
                  }}
                  className="absolute top-2 right-2 w-6 h-6 bg-black/50 rounded-full flex items-center justify-center text-white"
                >
                  <X className="w-4 h-4" />
                </button>
              )}
            </>
          ) : (
            <>
              <Plus className="w-6 h-6 text-text-sub mb-1 opacity-70 group-active:scale-110 transition-transform" />
              <span className="text-[12px] text-text-sub">{t("notary.verify.id_back")}</span>
            </>
          )}
        </div>
      </div>

      {/* Face Collection Cell */}
      <div
        className="flex items-center pt-3 border-t border-border-color/50 mt-1 cursor-pointer active:bg-active-bg transition-colors -mx-4 px-4 pb-3"
        onClick={() => !facePreview && !faceScore && faceRef.current?.click()}
      >
        <div className="flex items-center">
          <label className="text-[15px] text-text-main w-[70px] shrink-0 pointer-events-none">
            {t("notary.verify.face_snapshot")} <span className="text-red-500">*</span>
          </label>
          {/* If there is a photo, show the photo thumb with absolute click zone */}
          {facePreview ? (
            <div
              className={cn(
                "w-[64px] h-[64px] rounded-lg border relative overflow-hidden shrink-0 ml-2",
                faceScore ? "border-green-500/50" : "border-primary-blue/50",
              )}
              onClick={(e) => {
                e.stopPropagation();
                setFullscreenImage(facePreview);
              }}
            >
              <img
                src={facePreview}
                alt={t("notary.verify.face_preview")}
                className="absolute inset-0 w-full h-full object-contain bg-black/5 dark:bg-white/5"
              />
              <div className="absolute inset-0 bg-black/30 flex items-center justify-center pointer-events-none">
                {faceScore ? (
                  <CheckCircle2 className="w-6 h-6 text-green-400" />
                ) : (
                  <ScanFace className="w-6 h-6 text-white/90" />
                )}
              </div>
              {!isReadonly && (
                <button
                  onClick={(e) => {
                    e.stopPropagation();
                    setFacePreview(null);
                    setFaceScore(null);
                  }}
                  className="absolute top-1 right-1 w-5 h-5 bg-black/50 rounded-full flex items-center justify-center text-white"
                >
                  <X className="w-3 h-3" />
                </button>
              )}
            </div>
          ) : (
            <div className="w-[64px] h-[64px] rounded-lg bg-input-bg border flex flex-col items-center justify-center shrink-0 ml-2 border-dashed border-primary-blue/30 text-primary-blue">
              <Plus className="w-6 h-6 opacity-70 mb-0.5" />
              <span className="text-[10px] opacity-80">{t("notary.verify.click_capture")}</span>
            </div>
          )}
        </div>

        <div className="flex-1 flex items-center justify-end pl-3">
          {/* Right side interactions */}
          {facePreview && !faceScore ? (
            !isReadonly ? (
              <button
                disabled={isScanning}
                onClick={(e) => {
                  e.stopPropagation();
                  handleStartComparison();
                }}
                className="bg-primary-blue text-white px-4 py-1.5 rounded-full text-[13px] font-medium active:scale-95 transition-transform disabled:opacity-50"
              >
                {isScanning ? t("notary.verify.comparing") : t("notary.verify.compare")}
              </button>
            ) : (
              <span className="text-[14px] text-orange-500">{t("notary.verify.pending")}</span>
            )
          ) : faceScore ? (
            <div className="flex flex-col items-end">
              <span className="text-[14px] font-bold font-mono text-green-600 dark:text-green-400">
                {faceScore.toFixed(1)}%
              </span>
              <span className="text-[11px] text-green-700/70 dark:text-green-500/70">
                {t("notary.verify.passed")}
              </span>
            </div>
          ) : (
            <div className="flex items-center text-[15px]">
              <span className="text-text-sub">{t("notary.verify.capture")}</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
