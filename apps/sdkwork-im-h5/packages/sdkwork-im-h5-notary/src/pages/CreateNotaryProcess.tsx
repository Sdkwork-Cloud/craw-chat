import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, CheckCircle2 } from "lucide-react";
import { IconButton, cn } from "@sdkwork/im-h5-commons";
import { AnimatePresence } from "motion/react";
import { useTranslation } from "react-i18next";
import { notaryService } from "../services/notaryService";

import { NotaryPartyParams } from "./NotaryAddParty";

import { Step1TypeSelection } from "../components/Step1TypeSelection";
import { Step2NotaryParties } from "../components/Step2NotaryParties";
import { Step3ApplicationInfo } from "../components/Step3ApplicationInfo";
import { Step4Confirmation } from "../components/Step4Confirmation";

export const GLOBAL_STORE = {
  step: 1,
  selectedType: "",
  selectedNotary: "",
  selectedNotaryObj: null as any,
  parties: [] as any[],
  applicationInfo: "",
  attachments: [] as any[],
};

export const CreateNotaryProcess: React.FC = () => {
  const { t } = useTranslation();

  
const navigate = useNavigate();
  
  const [step, setStep] = useState(GLOBAL_STORE.step);
  const [notaryTypes, setNotaryTypes] = useState<any[]>([]);

  // Step 1: Business type
  const [selectedType, setSelectedType] = useState(GLOBAL_STORE.selectedType);

  // Step 2: Parties and Notary
  const [selectedNotary, setSelectedNotary] = useState(
    GLOBAL_STORE.selectedNotary,
  );
  const [selectedNotaryObj, setSelectedNotaryObj] = useState(
    GLOBAL_STORE.selectedNotaryObj,
  );
  const [parties, setParties] = useState<any[]>(GLOBAL_STORE.parties);

  // Step 3: Application Info
  const [applicationInfo, setApplicationInfo] = useState(
    GLOBAL_STORE.applicationInfo,
  );
  const [attachments, setAttachments] = useState<any[]>(
    GLOBAL_STORE.attachments
  );

  useEffect(() => {
    notaryService
      .getNotaryTypes()
      .then((types) =>
        setNotaryTypes(types.map((t) => ({ id: t.id, name: t.title }))),
      );
  }, []);

  useEffect(() => {
    GLOBAL_STORE.step = step;
    GLOBAL_STORE.selectedType = selectedType;
    GLOBAL_STORE.selectedNotary = selectedNotary;
    GLOBAL_STORE.selectedNotaryObj = selectedNotaryObj;
    GLOBAL_STORE.parties = parties;
    GLOBAL_STORE.applicationInfo = applicationInfo;
    GLOBAL_STORE.attachments = attachments;
  }, [
    step,
    selectedType,
    selectedNotary,
    selectedNotaryObj,
    parties,
    applicationInfo,
    attachments,
  ]);

  const handleNext = async () => {
    if (step === 1 && !selectedType) return;
    if (step === 2 && (!selectedNotary || parties.length === 0)) return;
    if (step === 3 && !applicationInfo) return;
    if (step < 4) {
      setStep(step + 1);
    } else {
      // Step 4 Complete
      const typeObj = notaryTypes.find((t) => t.id === selectedType);
      await notaryService.addRecord({
        title: typeObj?.name || t("notary.unknown_notary_type"),
        type: typeObj?.name?.replace("公证", "") || t("notary.evidence_preservation"),
      });

      GLOBAL_STORE.step = 1;
      GLOBAL_STORE.selectedType = "";
      GLOBAL_STORE.selectedNotary = "";
      GLOBAL_STORE.selectedNotaryObj = null;
      GLOBAL_STORE.parties = [];
      GLOBAL_STORE.applicationInfo = "";
      GLOBAL_STORE.attachments = [];
      navigate("/notary");
    }
  };

  const handleBack = () => {
  if (step > 1) {
      setStep(step - 1);
    } else {
      // Step 1 -> go back and reset store
      GLOBAL_STORE.step = 1;
      GLOBAL_STORE.selectedType = "";
      GLOBAL_STORE.selectedNotary = "";
      GLOBAL_STORE.selectedNotaryObj = null;
      GLOBAL_STORE.parties = [];
      GLOBAL_STORE.applicationInfo = "";
      GLOBAL_STORE.attachments = [];
      navigate(-1);
    }
  };

  const handleAddParty = () => {
  NotaryPartyParams.editData = null;
    NotaryPartyParams.onAdd = (party) => {
      GLOBAL_STORE.parties = [...GLOBAL_STORE.parties, party];
      setParties(GLOBAL_STORE.parties);
    };
    navigate("/notary/add-party");
  };

  const handleEditParty = (partyToEdit: any) => {
  NotaryPartyParams.editData = partyToEdit;
    NotaryPartyParams.onEdit = (updatedParty) => {
      GLOBAL_STORE.parties = GLOBAL_STORE.parties.map((p) =>
        p.id === updatedParty.id ? updatedParty : p,
      );
      setParties(GLOBAL_STORE.parties);
    };
    navigate("/notary/add-party");
  };

  const handleVideoCall = (party: any) => {
  navigate(`/call/video-notary/${party.id}`);
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
            />
          )}

          {step === 2 && (
            <Step2NotaryParties
              selectedNotary={selectedNotary}
              setSelectedNotary={setSelectedNotary}
              selectedNotaryObj={selectedNotaryObj}
              setSelectedNotaryObj={setSelectedNotaryObj}
              parties={parties}
              handleAddParty={handleAddParty}
              handleEditParty={handleEditParty}
              handleVideoCall={handleVideoCall}
              navigate={navigate}
              GLOBAL_STORE={GLOBAL_STORE}
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
          className={cn(
            "flex-1 h-12 rounded-xl font-bold text-[16px] flex items-center justify-center transition-all",
            (step === 1 && !selectedType) ||
              (step === 2 && (!selectedNotary || parties.length === 0)) ||
              (step === 3 && !applicationInfo)
              ? "bg-border-color text-text-sub opacity-50 cursor-not-allowed"
              : "bg-primary-blue text-white active:scale-[0.98] shadow-lg shadow-primary-blue/20",
          )}
        >
          {step === 4 ? t("notary.submit_finish") : t("notary.next_step")}
        </button>
      </div>
    </div>
  );
};
