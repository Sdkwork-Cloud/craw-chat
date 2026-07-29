import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { useRef } from "react";
import { ChevronLeft, CheckCircle2 } from "lucide-react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import { AnimatePresence } from "motion/react";
import { useTranslation } from "react-i18next";
import {
  appendBoundedUnique,
  NOTARY_CLIENT_WINDOW_LIMIT,
  notaryService,
  type NotaryDraftAttachment,
} from "../services/notaryService";

import {
  notaryDraftSession,
  type NotaryDraftPartyWithId,
} from "../state/notaryDraftSession";

import { Step1TypeSelection } from "../components/Step1TypeSelection";
import { Step2NotaryParties } from "../components/Step2NotaryParties";
import { Step3ApplicationInfo } from "../components/Step3ApplicationInfo";
import { Step4Confirmation } from "../components/Step4Confirmation";

export const CreateNotaryProcess: React.FC = () => {
  const { t } = useTranslation();

  const navigate = useNavigate();
  const [initialDraft] = useState(() => notaryDraftSession.getDraft());
  const [step, setStep] = useState(initialDraft.step);
  const [notaryTypes, setNotaryTypes] = useState<Array<{ id: string; name: string }>>([]);
  const [nextMatterCursor, setNextMatterCursor] = useState<string | undefined>();
  const [isLoadingMatters, setIsLoadingMatters] = useState(false);
  const matterLoadingRef = useRef(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [submitError, setSubmitError] = useState(false);

  // Step 1: Business type
  const [selectedType, setSelectedType] = useState(initialDraft.selectedType);

  // Step 2: Parties and Notary
  const [selectedNotary, setSelectedNotary] = useState(
    initialDraft.selectedNotary,
  );
  const [selectedNotaryObj, setSelectedNotaryObj] = useState(
    initialDraft.selectedNotaryObj,
  );
  const [parties] = useState(initialDraft.parties);

  // Step 3: Application Info
  const [applicationInfo, setApplicationInfo] = useState(
    initialDraft.applicationInfo,
  );
  const [attachments, setAttachments] = useState<NotaryDraftAttachment[]>(
    initialDraft.attachments,
  );

  const loadMatters = async (cursor?: string) => {
    if (matterLoadingRef.current) return;
    matterLoadingRef.current = true;
    setIsLoadingMatters(true);
    try {
      const page = await notaryService.getNotaryTypes(cursor);
      const matters = page.matters.map((matter) => ({
        id: matter.id,
        name: matter.title,
      }));
      const merged = appendBoundedUnique(
        notaryTypes,
        matters,
        (matter) => matter.id,
      );
      setNotaryTypes(merged);
      setNextMatterCursor(
        page.pageInfo.hasMore && merged.length < NOTARY_CLIENT_WINDOW_LIMIT
          ? page.pageInfo.nextCursor
          : undefined,
      );
    } finally {
      matterLoadingRef.current = false;
      setIsLoadingMatters(false);
    }
  };

  useEffect(() => {
    void loadMatters();
  }, []);

  useEffect(() => {
    notaryDraftSession.replaceDraft({
      step,
      selectedType,
      selectedNotary,
      selectedNotaryObj,
      parties,
      applicationInfo,
      attachments,
      submissionIdempotencyKey: initialDraft.submissionIdempotencyKey,
    });
  }, [
    step,
    selectedType,
    selectedNotary,
    selectedNotaryObj,
    parties,
    applicationInfo,
    attachments,
    initialDraft.submissionIdempotencyKey,
  ]);

  const handleNext = async () => {
    if (isSubmitting) return;
    if (step === 1 && !selectedType) return;
    if (step === 2 && (!selectedNotary || parties.length === 0)) return;
    if (step === 3 && !applicationInfo) return;
    if (step < 4) {
      setStep(step + 1);
    } else {
      const typeObj = notaryTypes.find((t) => t.id === selectedType);
      const firstParty = parties[0];
      if (!typeObj || !firstParty) return;
      setIsSubmitting(true);
      setSubmitError(false);
      try {
        await notaryService.createCase({
          skuId: selectedType,
          title: typeObj.name,
          applicantName: firstParty.name,
          description: applicationInfo,
          primaryNotaryMembershipId: selectedNotary,
          parties,
          attachments,
          idempotencyKey: initialDraft.submissionIdempotencyKey,
        });
        notaryDraftSession.reset();
        navigate("/notary");
      } catch {
        setSubmitError(true);
      } finally {
        setIsSubmitting(false);
      }
    }
  };

  const handleBack = () => {
    if (step > 1) {
      setStep(step - 1);
    } else {
      notaryDraftSession.reset();
      navigate(-1);
    }
  };

  const handleAddParty = () => {
    notaryDraftSession.openPartyEditor({ mode: "add" });
    navigate("/notary/add-party");
  };

  const handleEditParty = (partyToEdit: NotaryDraftPartyWithId) => {
    notaryDraftSession.openPartyEditor({
      mode: "edit",
      partyId: partyToEdit.id,
    });
    navigate("/notary/add-party");
  };

  const currentStepTitle = [
    t("notary.create_steps.step_1"),
    t("notary.create_steps.step_2"),
    t("notary.create_steps.step_3"),
    t("notary.create_steps.step_4"),
  ][step - 1];

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Header */}
      <header className="h-[44px] flex items-center justify-between glass-header sticky top-0 z-10 shrink-0 pt-safe px-1">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={handleBack}
          />
        </div>
        <div className="flex items-center justify-center font-bold text-text-main text-[17px] pointer-events-none">
          {currentStepTitle}
        </div>
        <div className="flex justify-end z-10 flex-1 px-3">
          <span className="text-[13px] text-text-sub font-mono">
            {step} / 4
          </span>
        </div>
      </header>

      {/* Step Indicator */}
      <div className="flex px-6 py-4 items-center justify-between bg-bg-color shrink-0 shadow-sm z-10">
        {[1, 2, 3, 4].map((s) => (
          <React.Fragment key={s}>
            <div
              className={cn(
                "w-8 h-8 rounded-full flex items-center justify-center text-[14px] font-bold transition-colors z-10",
                step >= s
                  ? "bg-primary-blue text-white"
                  : "bg-border-color text-text-sub",
              )}
            >
              {s === 4 && step === 4 ? <CheckCircle2 className="w-5 h-5" /> : s}
            </div>
            {s < 4 && (
              <div className="flex-1 h-1 mx-2 bg-border-color overflow-hidden rounded-full">
                <div
                  className={cn(
                    "h-full bg-primary-blue transition-all duration-300",
                    step > s ? "w-full" : "w-0",
                  )}
                />
              </div>
            )}
          </React.Fragment>
        ))}
      </div>

      {/* Content Area */}
      <div className="flex-1 overflow-y-auto p-4 flex flex-col relative">
        <AnimatePresence mode="wait">
          {step === 1 && (
            <Step1TypeSelection
              notaryTypes={notaryTypes}
              selectedType={selectedType}
              setSelectedType={setSelectedType}
              hasMore={Boolean(nextMatterCursor)}
              isLoadingMore={isLoadingMatters}
              onLoadMore={() => void loadMatters(nextMatterCursor)}
            />
          )}

          {step === 2 && (
            <Step2NotaryParties
              selectedNotary={selectedNotary}
              selectedNotaryObj={selectedNotaryObj}
              parties={parties}
              handleAddParty={handleAddParty}
              handleEditParty={handleEditParty}
              navigate={navigate}
            />
          )}

          {step === 3 && (
            <Step3ApplicationInfo
              applicationInfo={applicationInfo}
              setApplicationInfo={setApplicationInfo}
              attachments={attachments}
              setAttachments={setAttachments}
            />
          )}

          {step === 4 && (
            <Step4Confirmation
              notaryTypes={notaryTypes}
              selectedType={selectedType}
              selectedNotaryObj={selectedNotaryObj}
              parties={parties}
              applicationInfo={applicationInfo}
              attachments={attachments}
              navigate={navigate}
            />
          )}
        </AnimatePresence>
      </div>

      {/* Footer */}
      <div className="px-4 py-3 pb-safe border-t border-border-color bg-bg-color shrink-0 flex gap-3">
        {submitError && (
          <span className="self-center text-[12px] text-red-500">
            {t("notary.create_steps.submit_failed", "Submission failed")}
          </span>
        )}
        {step > 1 && (
          <button
            onClick={handleBack}
            className="flex-1 h-12 rounded-xl font-bold text-[16px] flex items-center justify-center transition-all bg-input-bg text-text-main active:bg-black/5 dark:active:bg-white/5 border border-border-color"
          >
            {t("notary.prev_step")}
          </button>
        )}
        <button
          onClick={handleNext}
          disabled={isSubmitting}
          className={cn(
            "flex-1 h-12 rounded-xl font-bold text-[16px] flex items-center justify-center transition-all",
            (step === 1 && !selectedType) ||
              (step === 2 && (!selectedNotary || parties.length === 0)) ||
              (step === 3 && !applicationInfo)
              || isSubmitting
              ? "bg-border-color text-text-sub opacity-50 cursor-not-allowed"
              : "bg-primary-blue text-white active:scale-[0.98] shadow-lg shadow-primary-blue/20",
          )}
        >
          {isSubmitting
            ? t("notary.create_steps.submitting", "Submitting...")
            : step === 4
              ? t("notary.submit_finish")
              : t("notary.next_step")}
        </button>
      </div>
    </div>
  );
};
