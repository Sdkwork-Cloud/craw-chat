import React from "react";
import { useTranslation } from "react-i18next";
import { QrCode } from "lucide-react";
import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

/**
 * QR scan surface — fail-closed (PRD).
 *
 * QR scanning remains unavailable until its owning generated SDK, permission
 * composition, and end-to-end evidence exist. This page must never simulate a
 * scan or fabricate a "scan success": it renders the typed unavailable state.
 */
export const Scan: React.FC = () => {
  const { t } = useTranslation();

  return (
    <CapabilityUnavailablePage
      icon={QrCode}
      title={t("contacts.scan_unavailable_title", "QR scanning is not available yet")}
      message={t("contacts.scan_unavailable_desc", "Scanning is not connected yet; scanned contacts cannot be resolved right now.")}
      onBack={() => window.history.back()}
    />
  );
};
