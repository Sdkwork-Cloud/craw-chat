import React, { useState } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, ChevronRight } from "lucide-react";
import { IconButton, cn, showToast } from "@sdkwork/im-h5-commons";
import { NotaryFullPageEditor } from "../components/NotaryFullPageEditor";
import { NotaryBottomPicker } from "../components/NotaryBottomPicker";
import { BasicInfoSection } from "../components/BasicInfoSection";
import { NotaryPartyBottomBar } from "../components/NotaryPartyBottomBar";
import { useTranslation } from "react-i18next";
import { uuid } from "@sdkwork/utils";
import type { NotaryDraftParty } from "../services/notaryService";

export interface NotaryDraftPartyWithId extends NotaryDraftParty {
  id: string;
}

export const NotaryPartyParams: {
  editData: NotaryDraftPartyWithId | null;
  isReadonly: boolean;
  onAdd: (party: NotaryDraftPartyWithId) => void;
  onEdit: (party: NotaryDraftPartyWithId) => void;
} = {
  editData: null,
  isReadonly: false,
  onAdd: () => undefined,
  onEdit: () => undefined,
};

export const NotaryAddParty: React.FC = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  

  const [formData, setFormData] = useState(() => {
    if (NotaryPartyParams.editData) {
      return {
        name: NotaryPartyParams.editData.name || "",
        idCard: NotaryPartyParams.editData.idCard || "",
        gender: NotaryPartyParams.editData.gender || "男",
        dob: NotaryPartyParams.editData.dob || "",
        idStartDate: NotaryPartyParams.editData.idStartDate || "",
        idEndDate: NotaryPartyParams.editData.idEndDate || "",
        phone: NotaryPartyParams.editData.phone || "",
        address: NotaryPartyParams.editData.address || "",
        remarks: NotaryPartyParams.editData.remarks || "",
      };
    }
    return {
      name: "",
      idCard: "",
      gender: "男",
      dob: "",
      idStartDate: "",
      idEndDate: "",
      phone: "",
      address: "",
      remarks: "",
    };
  });

  const [pickerType, setPickerType] = useState<
    "gender" | "dob" | "idStartDate" | "idEndDate" | null
  >(null);
  const [tempDate, setTempDate] = useState({ year: 1990, month: 1, day: 1 });

  const [fullPageEditor, setFullPageEditor] = useState<{
    field: keyof typeof formData;
    title: string;
    placeholder: string;
    value: string;
    isTextArea?: boolean;
    inputType?: string;
  } | null>(null);

  const handleSave = () => {
    if (!formData.name || formData.name.trim().length < 2)
      return showToast(t("notary.add_party.err_name"));
    if (
      !formData.idCard ||
      !/(^\d{15}$)|(^\d{18}$)|(^\d{17}(\d|X|x)$)/.test(formData.idCard)
    )
      return showToast(t("notary.add_party.err_id"));
    if (!formData.phone || !/^1\d{10}$/.test(formData.phone))
      return showToast(t("notary.add_party.err_phone"));

    if (!formData.idStartDate || !formData.idEndDate)
      return showToast(t("notary.add_party.err_id_date"));

    const partyData = {
      id: NotaryPartyParams.editData
        ? NotaryPartyParams.editData.id
        : uuid(),
      name: formData.name,
      idCard: formData.idCard,
      gender: formData.gender,
      dob: formData.dob,
      idStartDate: formData.idStartDate,
      idEndDate: formData.idEndDate,
      phone: formData.phone,
      address: formData.address,
      remarks: formData.remarks,
    };

    if (NotaryPartyParams.editData && NotaryPartyParams.onEdit) {
      NotaryPartyParams.onEdit(partyData);
      navigate(-1);
    } else if (NotaryPartyParams.onAdd) {
      NotaryPartyParams.onAdd(partyData);
      navigate(-1);
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black text-text-main fixed inset-0 z-[100] animate-in slide-in-from-right">
      <header className="h-[44px] flex items-center justify-between sticky top-0 shrink-0 pt-safe px-1 z-20 bg-bg-color border-b border-border-color">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="flex items-center justify-center font-bold text-[17px] pointer-events-none">
          {NotaryPartyParams.isReadonly ? t("notary.add_party.detail") : (NotaryPartyParams.editData ? t("notary.add_party.edit") : t("notary.add_party.add"))}
        </div>
        <div className="flex justify-end items-center gap-3 z-10 flex-1 pr-4">
        </div>
      </header>

      <div className="flex-1 overflow-y-auto pb-24 relative z-0">
        <div className="flex flex-col gap-2">
          <div className={cn(NotaryPartyParams.isReadonly && "pointer-events-none cursor-default")}>
            <BasicInfoSection
              formData={formData}
              setFullPageEditor={setFullPageEditor}
              setTempDate={setTempDate}
              setPickerType={setPickerType}
            />
          </div>

          <button
            type="button"
            className="flex min-h-[54px] items-center border-b border-border-color bg-bg-color px-4 text-left"
            onClick={() => setFullPageEditor({
              field: "remarks",
              title: t("notary.extra_info.remarks"),
              placeholder: t("notary.extra_info.remarks_placeholder"),
              value: formData.remarks,
              isTextArea: true,
            })}
          >
            <span className="w-[100px] shrink-0 text-[15px] text-text-main">
              {t("notary.extra_info.remarks")}
            </span>
            <span className="min-w-0 flex-1 truncate text-right text-[15px] text-text-sub">
              {formData.remarks || t("notary.extra_info.no_remarks")}
            </span>
            <ChevronRight className="ml-1 h-5 w-5 shrink-0 text-text-sub" />
          </button>
        </div>
      </div>

      {/* Fixed Bottom Operations */}
      <NotaryPartyBottomBar
        isReadonly={NotaryPartyParams.isReadonly}
        onBack={() => navigate(-1)}
        onSave={handleSave}
      />

      {/* Full Page Editor Overlay */}
      {fullPageEditor && (
        <NotaryFullPageEditor
          field={fullPageEditor.field}
          title={fullPageEditor.title}
          placeholder={fullPageEditor.placeholder}
          value={fullPageEditor.value}
          isTextArea={fullPageEditor.isTextArea}
          inputType={fullPageEditor.inputType}
          onChange={(val) =>
            setFullPageEditor({ ...fullPageEditor, value: val })
          }
          onSave={() => {
            setFormData((prev) => ({
              ...prev,
              [fullPageEditor.field]: fullPageEditor.value,
            }));
            setFullPageEditor(null);
          }}
          onClose={() => setFullPageEditor(null)}
        />
      )}

      {/* Pickers Overlay */}
      <NotaryBottomPicker
        pickerType={pickerType}
        formData={formData}
        setFormData={setFormData}
        setPickerType={setPickerType}
      />

    </div>
  );
};
