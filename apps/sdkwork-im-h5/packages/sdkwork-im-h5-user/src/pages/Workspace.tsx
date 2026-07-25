import React, { useState } from "react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import {
  Calendar,
  CheckSquare,
  FileText,
  Cloud,
  Video,
  Users,
  Briefcase,
  Scale,
  Cpu
} from "lucide-react";
import { showToast } from "@sdkwork/im-h5-commons";
import { WorkspaceHeader } from "../components/workspace/WorkspaceHeader";
import { WorkspaceAppIcon } from "../components/workspace/WorkspaceAppIcon";
import { WorkspaceAIToolsSection } from "../components/workspace/WorkspaceAIToolsSection";
import { WorkspaceCommonAppsSection } from "../components/workspace/WorkspaceCommonAppsSection";
import { WorkspaceHRAdminSection } from "../components/workspace/WorkspaceHRAdminSection";

export const Workspace: React.FC = () => {
  
  
const navigate = useNavigate();
  const { t, i18n } = useTranslation();
  const [showMenu, setShowMenu] = useState(false);

  const toggleLanguage = () => {
  const newLang = i18n.language === 'zh' ? 'en' : 'zh';
    i18n.changeLanguage(newLang);
    showToast(`Switched to ${newLang === 'zh' ? '中文' : 'English'}`);
  };

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto pb-[84px]">
      <WorkspaceHeader 
        showMenu={showMenu}
        setShowMenu={setShowMenu}
        toggleLanguage={toggleLanguage}
      />

      <div className="flex flex-col pb-6">
        <div className="px-4">
          <WorkspaceAIToolsSection />
        </div>
        <WorkspaceCommonAppsSection />
        <WorkspaceHRAdminSection />
      </div>
    </div>
  );
};

