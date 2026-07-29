import React from "react";
import { ChevronRight, Plus, User, UserPlus } from "lucide-react";
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";

import type { NotaryStaffMember } from "../services/notaryService";
import {
  notaryDraftSession,
  type NotaryDraftPartyWithId,
} from "../state/notaryDraftSession";

interface Step2NotaryPartiesProps {
  selectedNotary: string;
  selectedNotaryObj: NotaryStaffMember | null;
  parties: NotaryDraftPartyWithId[];
  handleAddParty: () => void;
  handleEditParty: (party: NotaryDraftPartyWithId) => void;
  navigate: (path: string) => void;
}

export const Step2NotaryParties: React.FC<Step2NotaryPartiesProps> = ({
  selectedNotary,
  selectedNotaryObj,
  parties,
  handleAddParty,
  handleEditParty,
  navigate,
}) => {
  const { t } = useTranslation();
  const openNotarySearch = () => {
    notaryDraftSession.openNotarySelection();
    navigate("/notary/search");
  };

  return (
    <motion.div
      key="step2"
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="flex flex-col gap-4"
    >
      <h2 className="text-[18px] font-bold">
        {t("notary.create_steps.notary_and_parties")}
      </h2>

      <div className="flex flex-col gap-2">
        <span className="text-[14px] font-medium text-text-sub">
          {t("notary.create_steps.select_notary_prompt")}
        </span>
        <button
          type="button"
          onClick={openNotarySearch}
          className="flex min-h-[48px] w-full items-center justify-between rounded-lg border border-border-color bg-input-bg px-4 py-3 text-left"
        >
          {selectedNotaryObj ? (
            <span className="min-w-0">
              <span className="block truncate text-[15px] font-medium text-text-main">
                {selectedNotaryObj.name}
              </span>
              <span className="block truncate text-[12px] text-text-sub">
                {selectedNotaryObj.organization}
              </span>
            </span>
          ) : (
            <span className="text-[15px] text-text-sub">
              {t("notary.create_steps.please_select_notary")}
            </span>
          )}
          <ChevronRight className="h-5 w-5 shrink-0 text-text-sub" />
        </button>
      </div>

      <div className="mt-2 flex items-center justify-between">
        <span className="text-[14px] font-medium text-text-sub">
          {t("notary.create_steps.party_list")}
        </span>
        <button
          type="button"
          onClick={handleAddParty}
          className="flex items-center gap-1 text-[13px] font-medium text-primary-blue"
        >
          <UserPlus className="h-4 w-4" />
          {t("notary.create_steps.add_party")}
        </button>
      </div>

      {parties.length === 0 ? (
        <div className="flex flex-col items-center justify-center rounded-lg border border-dashed border-border-color py-10 text-text-sub">
          <User className="mb-3 h-9 w-9 opacity-40" />
          <span className="mb-4 text-[14px]">
            {t("notary.create_steps.no_parties_prompt")}
          </span>
          <button
            type="button"
            onClick={handleAddParty}
            className="flex h-10 w-10 items-center justify-center rounded-full bg-primary-blue text-white"
            aria-label={t("notary.create_steps.add_party")}
          >
            <Plus className="h-5 w-5" />
          </button>
        </div>
      ) : (
        <div className="flex flex-col divide-y divide-border-color border-y border-border-color">
          {parties.map((party) => (
            <button
              key={party.id}
              type="button"
              className="flex items-center gap-3 px-2 py-4 text-left"
              onClick={() => handleEditParty(party)}
            >
              <div className="flex h-11 w-11 shrink-0 items-center justify-center rounded-full bg-primary-blue/10 font-semibold text-primary-blue">
                {party.name.trim().charAt(0).toUpperCase()}
              </div>
              <span className="min-w-0 flex-1">
                <span className="block truncate text-[15px] font-medium text-text-main">
                  {party.name}
                </span>
                <span className="block text-[13px] text-text-sub">
                  {maskIdentityNumber(party.idCard)}
                </span>
              </span>
              <ChevronRight className="h-5 w-5 shrink-0 text-text-sub" />
            </button>
          ))}
        </div>
      )}
    </motion.div>
  );
};

function maskIdentityNumber(value: string): string {
  const normalized = value.trim();
  if (normalized.length <= 8) {
    return "****";
  }
  return normalized.substring(0, 4)
    + "*".repeat(normalized.length - 8)
    + normalized.substring(normalized.length - 4);
}
