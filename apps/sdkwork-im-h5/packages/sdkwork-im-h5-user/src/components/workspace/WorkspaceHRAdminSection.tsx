import React from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { CheckSquare, FileText, Users, Briefcase } from "lucide-react";
import { WorkspaceAppIcon } from "./WorkspaceAppIcon";

export const WorkspaceHRAdminSection: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();

  return (
    <div className="pt-2 pb-6 px-4">
      <h3 className="text-[16px] font-bold text-text-main mb-4 flex items-center gap-2">
        <div className="w-1.5 h-4 bg-primary-blue rounded-full" />
        {t("workspace.hr_admin")}
      </h3>
      <div className="grid grid-cols-4 gap-y-7 gap-x-2">
        <WorkspaceAppIcon
          icon={FileText}
          label={t("workspace.leave")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={() =>
            navigate(
              `/workspace/approval/create?type=${encodeURIComponent(
                t("approval.createForm.typeLeave")
              )}`
            )
          }
        />
        <WorkspaceAppIcon
          icon={Briefcase}
          label={t("workspace.business_trip")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={() =>
            navigate(
              `/workspace/approval/create?type=${encodeURIComponent(
                t("workspace.business_trip")
              )}`
            )
          }
        />
        <WorkspaceAppIcon
          icon={CheckSquare}
          label={t("workspace.reimbursement")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={() =>
            navigate(
              `/workspace/approval/create?type=${encodeURIComponent(
                t("approval.createForm.typeExpense")
              )}`
            )
          }
        />
        <WorkspaceAppIcon
          icon={Users}
          label={t("workspace.recruitment")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={() => navigate("/workspace/recruitment")}
        />
      </div>
    </div>
  );
};
