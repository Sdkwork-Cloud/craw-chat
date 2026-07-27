import { useEffect, useMemo } from "react";
import { ArrowDownUp, LoaderCircle, WalletCards, X } from "lucide-react";
import { useTranslation } from "react-i18next";
import {
  Button,
  Modal,
  ModalBody,
  ModalClose,
  ModalContent,
  ModalHeader,
  ModalTitle,
  StatusNotice,
} from "@sdkwork/ui-pc-react";
import {
  useSdkworkMembershipController,
} from "@sdkwork/membership-pc-membership";
import {
  useSdkworkWalletControllerState,
} from "@sdkwork/account-pc-wallet";
import type { SdkworkSubscriptionCatalogModalProps } from "@sdkwork/membership-pc-subscription/catalog";
import {
  SdkworkCouponRedemptionDialog,
  SdkworkPointsRechargeDialog,
} from "@sdkwork/order-pc-recharge";
import {
  getImHostedCouponRechargeService,
  getImHostedPointsRechargeService,
} from "@sdkwork/im-pc-core";

import { getImTokenPlanWalletController } from "./tokenPlanMemberSummary";

const TOKEN_PLAN_I18N_ROOT = "commerce.tokenPlan";

function translateTokenPlan(
  translate: (key: string) => unknown,
  key: string,
): string {
  return String(translate(`${TOKEN_PLAN_I18N_ROOT}.${key}`));
}

function formatTokenPlanDateTime(
  value: string,
  formatter: Intl.DateTimeFormat,
): string {
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? value || "-" : formatter.format(date);
}

export function ImTokenPlanPointsPurchaseModal({
  currentPoints,
  isOpen,
  onClose,
}: SdkworkSubscriptionCatalogModalProps) {
  const { t } = useTranslation();
  const walletController = getImTokenPlanWalletController();
  const translate = (key: string) => translateTokenPlan(t, key);

  return (
    <SdkworkPointsRechargeDialog
      copy={{
        account: translate("recharge.account"),
        agreement: translate("recharge.agreement"),
        agreementAccepted: translate("recharge.agreementAccepted"),
        agreementRequired: translate("recharge.agreementRequired"),
        close: translate("common.close"),
        completed: translate("recharge.completed"),
        confirmPayment: translate("recharge.confirmPayment"),
        creatingPayment: translate("recharge.creatingPayment"),
        emptyPackages: translate("recharge.emptyPackages"),
        expired: translate("recharge.expired"),
        expiredDescription: translate("recharge.expiredDescription"),
        expiresIn: translate("recharge.expiresIn"),
        loadFailed: translate("recharge.loadFailed"),
        loadingPackages: translate("recharge.loadingPackages"),
        myPoints: translate("recharge.myTokenBank"),
        notice: translate("recharge.notice"),
        paymentUnavailable: translate("recharge.paymentUnavailable"),
        paymentUnavailableDescription: translate("recharge.paymentUnavailableDescription"),
        pointsUnit: translate("common.computeCredits"),
        retry: translate("common.retry"),
        retryPayment: translate("recharge.retryPayment"),
        scanPrompt: translate("recharge.scanPrompt"),
        title: translate("recharge.title"),
      }}
      currentPoints={currentPoints}
      isOpen={isOpen}
      onClose={onClose}
      onCompleted={async () => {
        await walletController.refresh().catch(() => undefined);
      }}
      service={getImHostedPointsRechargeService()}
    />
  );
}

export function ImTokenPlanRedeemModal({
  isOpen,
  onClose,
}: SdkworkSubscriptionCatalogModalProps) {
  const { t } = useTranslation();
  const membershipController = useSdkworkMembershipController();
  const walletController = getImTokenPlanWalletController();
  const translate = (key: string) => translateTokenPlan(t, key);

  return (
    <SdkworkCouponRedemptionDialog
      copy={{
        close: translate("common.close"),
        codeLabel: translate("redemption.codeLabel"),
        codePlaceholder: translate("redemption.codePlaceholder"),
        dailyQuota: translate("redemption.dailyQuota"),
        description: translate("redemption.description"),
        expiresAt: translate("redemption.expiresAt"),
        invalidCode: translate("redemption.invalidCode"),
        redeem: translate("redemption.redeem"),
        redeeming: translate("redemption.redeeming"),
        subscriptionActivated: translate("redemption.subscriptionActivated"),
        title: translate("redemption.title"),
        tokenBankCredited: translate("redemption.tokenBankCredited"),
        totalQuota: translate("redemption.totalQuota"),
      }}
      isOpen={isOpen}
      onClose={onClose}
      onCompleted={async (result) => {
        const refreshes = result.benefitKind === "subscription"
          ? [membershipController.refresh()]
          : [walletController.refresh()];
        await Promise.allSettled(refreshes);
      }}
      service={getImHostedCouponRechargeService()}
    />
  );
}

export function ImTokenPlanTokenBankDetailsModal({
  isOpen,
  onClose,
}: SdkworkSubscriptionCatalogModalProps) {
  const { i18n, t } = useTranslation();
  const walletController = getImTokenPlanWalletController();
  const state = useSdkworkWalletControllerState(walletController);
  const locale = i18n.resolvedLanguage === "en-US" ? "en-US" : "zh-CN";
  const numberFormat = useMemo(() => new Intl.NumberFormat(locale), [locale]);
  const dateTimeFormat = useMemo(() => new Intl.DateTimeFormat(locale, {
    dateStyle: "medium",
    timeStyle: "short",
  }), [locale]);
  const translate = (key: string) => translateTokenPlan(t, key);
  const tokenBankTransactions = state.overview.transactions
    .filter((transaction) => transaction.tokenBankDelta !== 0)
    .slice(0, 8);

  useEffect(() => {
    if (!isOpen) {
      return;
    }

    const request = state.isBootstrapped
      ? walletController.refresh()
      : walletController.bootstrap();
    void request.catch(() => undefined);
  }, [isOpen, state.isBootstrapped, walletController]);

  return (
    <Modal open={isOpen} onOpenChange={(open) => { if (!open) onClose(); }}>
      <ModalContent
        aria-labelledby="im-token-bank-details-title"
        className="max-h-[min(44rem,calc(100dvh-2rem))] w-[min(92vw,44rem)] overflow-hidden"
        showCloseButton={false}
      >
        <ModalHeader className="flex-row items-start justify-between gap-4 border-b border-[var(--sdk-color-border-subtle)]">
          <div className="flex min-w-0 items-start gap-3">
            <span className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-brand-primary)]">
              <WalletCards aria-hidden="true" className="h-5 w-5" />
            </span>
            <div className="min-w-0">
              <ModalTitle id="im-token-bank-details-title">
                {translate("details.title")}
              </ModalTitle>
              <p className="mt-1 text-sm text-[var(--sdk-color-text-secondary)]">
                {translate("details.description")}
              </p>
            </div>
          </div>
          <ModalClose
            aria-label={translate("common.close")}
            className="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg text-[var(--sdk-color-text-secondary)] hover:bg-[var(--sdk-color-surface-panel-muted)]"
            onClick={onClose}
          >
            <X aria-hidden="true" className="h-4 w-4" />
          </ModalClose>
        </ModalHeader>

        <ModalBody className="overflow-y-auto">
          <dl className="grid grid-cols-1 border-b border-[var(--sdk-color-border-subtle)] sm:grid-cols-2">
            <div className="px-5 py-5 sm:px-6">
              <dt className="text-xs text-[var(--sdk-color-text-muted)]">
                {translate("details.available")}
              </dt>
              <dd className="mt-2 text-3xl font-semibold tabular-nums text-[var(--sdk-color-text-primary)]">
                {numberFormat.format(state.overview.account.tokenBankAvailable)}
              </dd>
            </div>
            <div className="border-t border-[var(--sdk-color-border-subtle)] px-5 py-5 sm:border-l sm:border-t-0 sm:px-6">
              <dt className="text-xs text-[var(--sdk-color-text-muted)]">
                {translate("details.frozen")}
              </dt>
              <dd className="mt-2 text-3xl font-semibold tabular-nums text-[var(--sdk-color-text-primary)]">
                {numberFormat.format(state.overview.account.tokenBankFrozen)}
              </dd>
            </div>
          </dl>

          <section className="px-5 py-5 sm:px-6">
            <div className="mb-4 flex items-center gap-2">
              <ArrowDownUp aria-hidden="true" className="h-4 w-4 text-[var(--sdk-color-text-secondary)]" />
              <h3 className="text-sm font-semibold text-[var(--sdk-color-text-primary)]">
                {translate("details.activity")}
              </h3>
            </div>

            {state.isLoading && !state.isBootstrapped ? (
              <div className="flex min-h-28 items-center justify-center text-[var(--sdk-color-text-secondary)]" role="status">
                <LoaderCircle aria-hidden="true" className="mr-2 h-5 w-5 animate-spin" />
                {translate("details.loading")}
              </div>
            ) : null}

            {state.lastError ? (
              <StatusNotice title={translate("details.loadFailed")} tone="danger">
                <Button
                  className="mt-3"
                  onClick={() => { void walletController.bootstrap().catch(() => undefined); }}
                  size="sm"
                  type="button"
                  variant="outline"
                >
                  {translate("common.retry")}
                </Button>
              </StatusNotice>
            ) : null}

            {!state.isLoading && !state.lastError && tokenBankTransactions.length === 0 ? (
              <StatusNotice title={translate("details.emptyTitle")}>
                {translate("details.emptyDescription")}
              </StatusNotice>
            ) : null}

            {!state.lastError && tokenBankTransactions.length > 0 ? (
              <ul className="divide-y divide-[var(--sdk-color-border-subtle)]">
                {tokenBankTransactions.map((transaction) => (
                  <li className="flex items-center justify-between gap-4 py-3" key={transaction.id}>
                    <div className="min-w-0">
                      <p className="truncate text-sm font-medium text-[var(--sdk-color-text-primary)]">
                        {transaction.title}
                      </p>
                      <p className="mt-1 text-xs text-[var(--sdk-color-text-muted)]">
                        {formatTokenPlanDateTime(transaction.createdAt, dateTimeFormat)}
                      </p>
                    </div>
                    <span className={transaction.tokenBankDelta > 0
                      ? "shrink-0 text-sm font-semibold tabular-nums text-[var(--sdk-color-state-success)]"
                      : "shrink-0 text-sm font-semibold tabular-nums text-[var(--sdk-color-text-primary)]"}
                    >
                      {transaction.tokenBankDelta > 0 ? "+" : ""}
                      {numberFormat.format(transaction.tokenBankDelta)}
                    </span>
                  </li>
                ))}
              </ul>
            ) : null}
          </section>
        </ModalBody>
      </ModalContent>
    </Modal>
  );
}
