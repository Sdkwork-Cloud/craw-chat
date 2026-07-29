import React, { useEffect, useState } from "react";
import { AlertCircle, Loader2, ShieldCheck } from "lucide-react";
import { useTranslation } from "react-i18next";
import {
  notaryService,
  type NotaryAccessSummary,
} from "../services/notaryService";

export const NotaryMe: React.FC = () => {
  const { t } = useTranslation();
  const [access, setAccess] = useState<NotaryAccessSummary | null>(null);
  const [error, setError] = useState(false);

  useEffect(() => {
    let active = true;
    void notaryService.getAccess().then(
      (value) => {
        if (active) {
          setAccess(value);
        }
      },
      () => {
        if (active) {
          setError(true);
        }
      },
    );
    return () => {
      active = false;
    };
  }, []);

  if (error) {
    return (
      <div className="flex h-full flex-col items-center justify-center gap-3 bg-bg-color px-8 text-center">
        <AlertCircle className="h-10 w-10 text-text-sub" />
        <p className="text-[14px] text-text-sub">
          {t("notary.me.access_unavailable", "Unable to load notary access")}
        </p>
      </div>
    );
  }

  if (!access) {
    return (
      <div className="flex h-full items-center justify-center bg-bg-color">
        <Loader2 className="h-6 w-6 animate-spin text-text-sub" />
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col bg-[#f2f2f2] dark:bg-[#121212]">
      <header className="glass-header h-[56px] shrink-0 pt-safe" />
      <main className="flex-1 overflow-y-auto pb-[90px]">
        <section className="border-b border-border-color/50 bg-bg-color p-6">
          <div className="flex items-center gap-4">
            <div className="flex h-14 w-14 shrink-0 items-center justify-center rounded-lg bg-primary-blue/10 text-primary-blue">
              <ShieldCheck className="h-7 w-7" />
            </div>
            <div className="min-w-0 flex-1">
              <h1 className="truncate text-[18px] font-bold text-text-main">
                {t("notary.me.member", "Notary member")}
              </h1>
              <p className="truncate font-mono text-[12px] text-text-sub">
                {access.memberId}
              </p>
            </div>
          </div>
          <div className="mt-4 flex flex-wrap gap-2">
            {access.roles.map((role) => (
              <span
                key={role}
                className="rounded-md border border-border-color bg-input-bg px-2 py-1 text-[12px] text-text-main"
              >
                {role}
              </span>
            ))}
          </div>
        </section>

        <section className="mt-2 divide-y divide-border-color/50 border-y border-border-color/50 bg-bg-color">
          <AccessRow
            label={t("notary.me.organization_verified", "Organization verified")}
            enabled={access.organizationVerified}
          />
          <AccessRow
            label={t("notary.me.business_enabled", "Notary business enabled")}
            enabled={access.businessEnabled}
          />
          <AccessRow
            label={t("notary.me.visible", "Notary workspace visible")}
            enabled={access.visible}
          />
        </section>
        {access.reason && (
          <p className="px-5 py-4 text-[13px] text-text-sub">{access.reason}</p>
        )}
      </main>
    </div>
  );
};

const AccessRow: React.FC<{ label: string; enabled: boolean }> = ({ label, enabled }) => (
  <div className="flex min-h-12 items-center justify-between px-4 py-3">
    <span className="text-[14px] text-text-main">{label}</span>
    <span className={enabled ? "text-green-600" : "text-text-sub"}>
      {enabled ? "Enabled" : "Unavailable"}
    </span>
  </div>
);
