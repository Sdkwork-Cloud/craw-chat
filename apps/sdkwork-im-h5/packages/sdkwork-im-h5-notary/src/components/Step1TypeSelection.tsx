import React from "react";
import { CheckCircle2, FileSignature } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";
import { motion } from "motion/react";
import { useTranslation } from "react-i18next";

interface Step1TypeSelectionProps {
  notaryTypes: any[];
  selectedType: string;
  setSelectedType: (id: string) => void;
}

export const Step1TypeSelection: React.FC<Step1TypeSelectionProps> = ({
  notaryTypes,
  selectedType,
  setSelectedType,
}) => {
  const { t } = useTranslation();
return (
    <motion.div
      key="step1"
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="flex flex-col gap-3"
    >
      <h2 className="text-[18px] font-bold mb-2">{t("notary.create_steps.select_type_prompt")}</h2>
      {notaryTypes.map((type) => (
        <div
          key={type.id}
          onClick={() => setSelectedType(type.id)}
          className={cn(
            "p-4 rounded-xl border flex items-center justify-between transition-all active:scale-[0.98]",
            selectedType === type.id
              ? "bg-primary-blue/10 border-primary-blue"
              : "bg-chat-other-bg border-border-color",
          )}
        >
          <div className="flex items-center gap-3">
            <div
              className={cn(
                "w-10 h-10 rounded-full flex items-center justify-center",
                selectedType === type.id
                  ? "bg-primary-blue text-white"
                  : "bg-border-color text-text-sub",
              )}
            >
              <FileSignature className="w-5 h-5" />
            </div>
            <span
              className={cn(
                "text-[16px] font-medium",
                selectedType === type.id
                  ? "text-primary-blue"
                  : "text-text-main",
              )}
            >
              {type.name}
            </span>
          </div>
          {selectedType === type.id && (
            <CheckCircle2 className="w-6 h-6 text-primary-blue" />
          )}
        </div>
      ))}
    </motion.div>
  );
};
