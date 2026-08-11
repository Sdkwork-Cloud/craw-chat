import React from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import {
  Calendar,
  CheckSquare,
  FileText,
  Cloud,
  Video,
  Users,
  Briefcase,
  Scale,
  Cpu,
  BookOpen,
} from "lucide-react";
import { WorkspaceAppIcon } from "./WorkspaceAppIcon";
import { showToast } from "@sdkwork/im-h5-commons";

export const WorkspaceCommonAppsSection: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();

  // calendar / approval / attendance / report are not composed by default
  // (audited as mock-only, no owner SDK): fail closed on click instead of
  // navigating to routes that no longer exist.
  const unavailable = () =>
    showToast(t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated."));

  return (
    <div className="pt-2 pb-6 px-4">
      <h3 className="text-[16px] font-bold text-text-main mb-4 flex items-center gap-2">
        <div className="w-1.5 h-4 bg-primary-blue rounded-full" />
        {t("workspace.common_apps")}
      </h3>
      <div className="grid grid-cols-4 gap-y-7 gap-x-2">
        <WorkspaceAppIcon
          icon={Cpu}
          label={t("workspace.hardware")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={() => navigate("/hardware")}
        />
        <WorkspaceAppIcon
          icon={Scale}
          label={t("workspace.notary")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={() => navigate("/notary")}
        />
        <WorkspaceAppIcon
          icon={Calendar}
          label={t("workspace.calendar")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={unavailable}
        />
        <WorkspaceAppIcon
          icon={CheckSquare}
          label={t("workspace.approval")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={unavailable}
        />
        <WorkspaceAppIcon
          icon={Briefcase}
          label={t("workspace.attendance")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={unavailable}
        />
        <WorkspaceAppIcon
          icon={FileText}
          label={t("workspace.report")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={unavailable}
        />
        <WorkspaceAppIcon
          icon={Cloud}
          label={t("workspace.drive")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={() => navigate("/workspace/drive")}
        />
        <WorkspaceAppIcon
          icon={Video}
          label={t("workspace.meeting")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={() => navigate("/workspace/meeting")}
        />
        <WorkspaceAppIcon
          icon={Users}
          label={t("workspace.contacts")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={() => navigate("/workspace/contacts")}
        />
        <WorkspaceAppIcon
          icon={BookOpen}
          label={t("knowledge.title", "Knowledge")}
          colorClass="text-primary-blue"
          bgClass="bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20"
          onClick={() => navigate("/workspace/knowledge")}
        />
      </div>
    </div>
  );
};
