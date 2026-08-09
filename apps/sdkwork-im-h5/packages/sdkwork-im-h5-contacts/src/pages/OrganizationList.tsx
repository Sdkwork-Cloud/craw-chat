import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Building2, ChevronRight } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { OrganizationService, type Organization } from "../services/OrganizationService";

import { useTranslation } from "react-i18next";

export const OrganizationList: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const [orgs, setOrgs] = useState<Organization[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    OrganizationService.getOrganizations().then(data => {
      if (data.length === 1) {
        navigate(`/contacts/org/${data[0].id}`, { replace: true });
        return;
      }
      setOrgs(data);
      setLoading(false);
    });
  }, [navigate]);

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <header className="h-[52px] flex items-center justify-between px-2 bg-bg-color/90 backdrop-blur-md sticky top-0 z-20 shrink-0 pt-safe border-b border-border-color">
        <div className="flex items-center z-10 w-[80px]">
          <IconButton
            icon={<ChevronLeft className="w-7 h-7 text-text-main" strokeWidth={2} />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 font-semibold text-[17px] text-text-main pointer-events-none">
          {t('contacts.my_orgs')}
        </div>
        <div className="flex-1 w-[80px]" />
      </header>

      <div className="flex-1 overflow-y-auto w-full bg-chat-other-bg pb-10">
        {loading ? (
          <div className="flex justify-center p-6 mt-10">
            <div className="w-6 h-6 border-2 border-primary-blue border-t-transparent rounded-full animate-spin"></div>
          </div>
        ) : (
          <div className="flex flex-col bg-bg-color mt-2">
            {orgs.map((org, index) => (
              <div key={org.id} className="relative">
                <div
                  onClick={() => navigate(`/contacts/org/${org.id}`)}
                  className="flex items-center gap-3 px-4 py-3.5 cursor-pointer active:bg-black/5 dark:active:bg-white/5"
                >
                  <div className="w-12 h-12 rounded overflow-hidden shrink-0 border border-border-color">
                    <img src={org.logo} alt={org.name} className="w-full h-full object-cover" />
                  </div>
                  <div className="flex-1 min-w-0 flex flex-col justify-center">
                    <div className="font-medium text-[16px] text-text-main truncate">
                      {org.name}
                    </div>
                    <div className="text-[13px] text-text-sub mt-0.5 flex items-center gap-1">
                      <Building2 className="w-3.5 h-3.5" />
                      <span>{t('contacts.org_structure')}</span>
                    </div>
                  </div>
                  <ChevronRight className="w-5 h-5 text-text-sub/40 shrink-0" />
                </div>
                {index < orgs.length - 1 && (
                  <div className="ml-[76px] border-b border-border-color" />
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};
