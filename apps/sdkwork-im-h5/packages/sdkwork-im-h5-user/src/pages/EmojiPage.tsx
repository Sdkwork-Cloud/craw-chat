import { useTranslation } from "react-i18next";
import React from "react";
import { PageLayout } from "../components/PageLayout";

/**
 * Emoji / sticker store — fail-closed (PRD).
 *
 * No real sticker-catalog API is composed: the previously hard-coded packs
 * (placeholder images, fake authors, fake "已添加" success) were fabricated
 * and are removed. The page renders the typed unavailable state instead.
 */
export const EmojiPage = () => {
  const { t } = useTranslation();

  return (
    <PageLayout title={t('user.auto_prop_10e55d', 'Stickers')}>
      <div className="flex-1 flex flex-col items-center justify-center gap-3 px-8 text-center py-20">
        <span className="text-[40px]">😶</span>
        <p className="text-[15px] text-text-main">
          {t("commons.feature_unavailable", "This feature is not available yet while the real service is being integrated.")}
        </p>
      </div>
    </PageLayout>
  );
};
