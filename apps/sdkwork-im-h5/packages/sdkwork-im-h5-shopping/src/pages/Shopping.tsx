import { ShoppingBag } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { CapabilityUnavailablePage } from "@sdkwork/im-h5-commons";

export function ShoppingPage() {
  const { t } = useTranslation("shopping");
  const navigate = useNavigate();

  return (
    <CapabilityUnavailablePage
      icon={ShoppingBag}
      message={t("unavailable")}
      onBack={() => navigate(-1)}
      title={t("title")}
    />
  );
}
