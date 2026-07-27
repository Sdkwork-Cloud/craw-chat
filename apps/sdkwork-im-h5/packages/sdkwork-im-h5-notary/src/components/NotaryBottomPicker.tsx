import React, { useState, useEffect } from "react";
import { cn, showToast } from "@sdkwork/im-h5-commons";
import { CustomDatePicker } from "./CustomDatePicker";
import { useTranslation } from "react-i18next";

interface NotaryBottomPickerProps {
  pickerType: "gender" | "dob" | "idStartDate" | "idEndDate" | null;
  formData: any;
  setFormData: React.Dispatch<React.SetStateAction<any>>;
  setPickerType: React.Dispatch<React.SetStateAction<any>>;
}

export const NotaryBottomPicker: React.FC<NotaryBottomPickerProps> = ({
  pickerType,
  formData,
  setFormData,
  setPickerType,
}) => {
  const { t } = useTranslation();
const [tempValue, setTempValue] = useState<string>("");

  const GENDERS = [t("notary.picker.male"), t("notary.picker.female"), t("notary.picker.unknown")];

  // Initialize tempValue when picker changes
  useEffect(() => {
    if (pickerType === "gender") {
      setTempValue(formData.gender || t("notary.picker.male"));
    } else if (pickerType === "dob") {
      setTempValue(formData.dob || "");
    } else if (pickerType === "idStartDate") {
      setTempValue(formData.idStartDate || "");
    } else if (pickerType === "idEndDate") {
      setTempValue(formData.idEndDate === t("notary.picker.long_term") || formData.idEndDate === "长期" ? "" : formData.idEndDate || "");
    }
  }, [pickerType, formData, t]);

  if (!pickerType) return null;

  const handleConfirm = (overrideValue?: string) => {
  const valueToUse = overrideValue !== undefined ? overrideValue : tempValue;
    if (pickerType === "gender") {
      setFormData((prev: any) => ({ ...prev, gender: valueToUse }));
      setPickerType(null);
    } else if (pickerType === "dob") {
      setFormData((prev: any) => ({ ...prev, dob: valueToUse }));
      setPickerType(null);
    } else if (pickerType === "idStartDate") {
      let computedEndDate = formData.idEndDate;
      if (formData.idCard && (formData.idCard.length === 15 || formData.idCard.length === 18)) {
         let birthYear = 0; let birthMonth = 0; let birthDay = 0;
         if (formData.idCard.length === 18) {
           birthYear = parseInt(formData.idCard.substring(6, 10));
           birthMonth = parseInt(formData.idCard.substring(10, 12));
           birthDay = parseInt(formData.idCard.substring(12, 14));
         } else {
           birthYear = parseInt("19" + formData.idCard.substring(6, 8));
           birthMonth = parseInt(formData.idCard.substring(8, 10));
           birthDay = parseInt(formData.idCard.substring(10, 12));
         }
         const startParts = valueToUse.split("-");
         if (startParts.length === 3) {
             const startYear = parseInt(startParts[0]);
             const startMonth = parseInt(startParts[1]);
             const startDay = parseInt(startParts[2]);
            
             let ageAtIssue = startYear - birthYear;
             if (startMonth < birthMonth || (startMonth === birthMonth && startDay < birthDay)) {
               ageAtIssue--;
             }
             
             if (ageAtIssue >= 46) {
               computedEndDate = t("notary.picker.long_term");
             } else if (ageAtIssue >= 26) {
               computedEndDate = `${startYear + 20}-${valueToUse.substring(5)}`;
             } else if (ageAtIssue >= 16) {
               computedEndDate = `${startYear + 10}-${valueToUse.substring(5)}`;
             } else if (ageAtIssue >= 0) {
               computedEndDate = `${startYear + 5}-${valueToUse.substring(5)}`;
             }
         }
      }

      if (computedEndDate && computedEndDate !== t("notary.picker.long_term") && computedEndDate !== "长期" && valueToUse > computedEndDate) {
        showToast(t("notary.picker.err_start_after_end"));
        return;
      }
      setFormData((prev: any) => ({ ...prev, idStartDate: valueToUse, idEndDate: computedEndDate }));
      setPickerType(null);
    } else if (pickerType === "idEndDate") {
      if (valueToUse !== t("notary.picker.long_term") && valueToUse !== "长期" && formData.idStartDate && valueToUse < formData.idStartDate) {
        showToast(t("notary.picker.err_end_before_start"));
        return;
      }
      setFormData((prev: any) => ({ ...prev, idEndDate: valueToUse }));
      setPickerType(null);
    }
  };

  return (
    <>
      <div
        className="fixed inset-0 bg-black/40 z-[250] animate-in fade-in"
        onClick={() => setPickerType(null)}
      />
      <div className="fixed bottom-0 left-0 right-0 z-[300] bg-bg-color rounded-t-2xl flex flex-col animate-in slide-in-from-bottom pb-safe max-h-[85vh]">
        <div className="flex items-center justify-between px-4 h-14 border-b border-border-color shrink-0 relative z-20">
          <button
            onClick={() => setPickerType(null)}
            className="text-[15px] font-medium px-2 py-1 active:opacity-70 text-text-sub"
          >
            {t("notary.picker.cancel")}
          </button>
          <span className="font-bold text-[16px] pointer-events-none">
            {pickerType === "gender"
              ? t("notary.picker.select_gender")
              : pickerType === "dob"
                ? t("notary.picker.select_dob")
                : pickerType === "idStartDate"
                  ? t("notary.picker.select_start_date")
                  : pickerType === "idEndDate"
                    ? t("notary.picker.select_end_date")
                    : ""}
          </span>
          <div className="flex items-center">
            {pickerType === "idEndDate" && (
              <button
                onClick={() => {
                  setTempValue(t("notary.picker.long_term"));
                  handleConfirm(t("notary.picker.long_term"));
                }}
                className="text-[13px] text-[#FA5151] mr-3 font-medium active:opacity-70 border border-[#FA5151]/30 px-2 py-0.5 rounded"
              >
                {t("notary.picker.set_long_term")}
              </button>
            )}
            <button
              onClick={() => handleConfirm()}
              className="text-[15px] font-medium px-2 py-1 active:opacity-70 text-primary-blue"
            >
              {t("notary.picker.confirm")}
            </button>
          </div>
        </div>

        {pickerType === "gender" && (
          <div className="flex flex-col py-6 px-6 gap-3 min-h-[220px] w-full max-w-sm mx-auto justify-center">
            {GENDERS.map((g) => (
              <div
                key={g}
                onClick={() => setTempValue(g)}
                className={cn(
                  "h-14 w-full rounded-2xl flex items-center justify-center font-bold text-[16px] transition-all cursor-pointer shadow-sm active:scale-[0.98]",
                  tempValue === g
                    ? "bg-primary-blue text-white ring-2 ring-primary-blue/30 ring-offset-2 dark:ring-offset-black"
                    : "bg-input-bg text-text-main hover:bg-black/5 dark:hover:bg-white/5 border border-border-color/50",
                )}
              >
                {g}
              </div>
            ))}
          </div>
        )}

        {(pickerType === "dob" ||
          pickerType === "idStartDate" ||
          pickerType === "idEndDate") && (
          <CustomDatePicker
            initialValue={
               pickerType === "dob"
                 ? formData.dob
                 : pickerType === "idStartDate"
                   ? formData.idStartDate
                   : (formData.idEndDate === t("notary.picker.long_term") || formData.idEndDate === "长期")
                     ? ""
                     : formData.idEndDate
            }
            onChange={(formatted) => setTempValue(formatted)}
            defaultYearOffset={
              pickerType === "idEndDate"
                ? 20
                : pickerType === "idStartDate"
                  ? -1
                  : -30
            }
          />
        )}
      </div>
    </>
  );
};

