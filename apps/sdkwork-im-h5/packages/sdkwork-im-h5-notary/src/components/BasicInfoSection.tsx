import React from "react";
import { ChevronRight } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { useTranslation } from "react-i18next";

interface BasicInfoSectionProps {
  formData: any;
  setFullPageEditor: React.Dispatch<React.SetStateAction<any>>;
  setTempDate: React.Dispatch<React.SetStateAction<any>>;
  setPickerType: React.Dispatch<React.SetStateAction<any>>;
}

export const BasicInfoSection: React.FC<BasicInfoSectionProps> = ({
  formData,
  setFullPageEditor,
  setTempDate,
  setPickerType,
}) => {
  const { t } = useTranslation();
return (
    <div className="bg-bg-color px-4 flex flex-col mb-2">
      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() =>
          setFullPageEditor({
            field: "phone",
            title: t("notary.basic_info.phone"),
            placeholder: t("notary.basic_info.phone_placeholder"),
            value: formData.phone,
            inputType: "tel",
          })
        }
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          {t("notary.basic_info.phone")} <span className="text-red-500">*</span>
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.phone ? (
            <span className="text-text-main">{formData.phone}</span>
          ) : (
            <span className="text-text-sub">{t("notary.basic_info.phone_placeholder")}</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() =>
          setFullPageEditor({
            field: "name",
            title: t("notary.basic_info.name"),
            placeholder: t("notary.basic_info.name_placeholder"),
            value: formData.name,
          })
        }
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          {t("notary.basic_info.name")} <span className="text-red-500">*</span>
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.name ? (
            <span className="text-text-main">{formData.name}</span>
          ) : (
            <span className="text-text-sub">{t("notary.basic_info.identify_or_input")}</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() =>
          setFullPageEditor({
            field: "idCard",
            title: t("notary.basic_info.id_card"),
            placeholder: t("notary.basic_info.id_placeholder"),
            value: formData.idCard,
          })
        }
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          {t("notary.basic_info.id_card")} <span className="text-red-500">*</span>
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.idCard ? (
            <span className="text-text-main">{formData.idCard}</span>
          ) : (
            <span className="text-text-sub">{t("notary.basic_info.identify_or_input")}</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() => {
          if (formData.idStartDate) {
            const [y, m, d] = formData.idStartDate.split("-");
            setTempDate({
              year: parseInt(y),
              month: parseInt(m),
              day: parseInt(d),
            });
          } else {
            setTempDate({ year: 2020, month: 1, day: 1 });
          }
          setPickerType("idStartDate");
        }}
      >
        <label className="text-[15px] text-text-main w-[110px] shrink-0">
          {t("notary.basic_info.id_start")} <span className="text-red-500">*</span>
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.idStartDate ? (
            <span className="text-text-main">{formData.idStartDate}</span>
          ) : (
            <span className="text-text-sub">{t("notary.basic_info.start_placeholder")}</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() => {
          if (formData.idEndDate && formData.idEndDate !== "长期") {
            const [y, m, d] = formData.idEndDate.split("-");
            setTempDate({
              year: parseInt(y),
              month: parseInt(m),
              day: parseInt(d),
            });
          } else {
            setTempDate({ year: 2040, month: 1, day: 1 });
          }
          setPickerType("idEndDate");
        }}
      >
        <label className="text-[15px] text-text-main w-[110px] shrink-0">
          {t("notary.basic_info.id_end")} <span className="text-red-500">*</span>
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.idEndDate ? (
            <span className="text-text-main">{formData.idEndDate}</span>
          ) : (
            <span className="text-text-sub">{t("notary.basic_info.end_placeholder")}</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() => setPickerType("gender")}
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          {t("notary.basic_info.gender")}
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.gender ? (
            <span className="text-text-main">{formData.gender}</span>
          ) : (
            <span className="text-text-sub">{t("notary.basic_info.gender_placeholder")}</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() => {
          if (formData.dob) {
            const [y, m, d] = formData.dob.split("-");
            setTempDate({
              year: parseInt(y),
              month: parseInt(m),
              day: parseInt(d),
            });
          } else {
            setTempDate({ year: 1990, month: 1, day: 1 });
          }
          setPickerType("dob");
        }}
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          {t("notary.basic_info.dob")}
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.dob ? (
            <span className="text-text-main">{formData.dob}</span>
          ) : (
            <span className="text-text-sub">{t("notary.basic_info.gender_placeholder")}</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() =>
          setFullPageEditor({
            field: "address",
            title: t("notary.basic_info.address"),
            placeholder: t("notary.basic_info.address_placeholder"),
            value: formData.address,
            isTextArea: true,
          })
        }
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          {t("notary.basic_info.address")}
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.address ? (
            <span className="text-text-main truncate max-w-[150px]">
              {formData.address}
            </span>
          ) : (
            <span className="text-text-sub">{t("notary.basic_info.address_placeholder")}</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>
    </div>
  );
};
