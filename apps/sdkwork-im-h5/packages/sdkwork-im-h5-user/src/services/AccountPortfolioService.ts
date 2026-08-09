import {
  getAccountAppSdkClient,
  type SdkworkAccountAppClient,
} from "@sdkwork/im-h5-core/sdk";

/**
 * Aggregated wallet portfolio surfaced by the composed account API
 * (`GET /app/v3/api/wallet/portfolio`): the cash account, the token bank and
 * the compute points are returned by one call.
 */
export interface WalletPortfolio {
  cash: {
    currencyCode: string;
    availableAmount: string;
    frozenAmount: string;
    pendingAmount: string;
  };
  tokenBank: {
    availableAmount: string;
    frozenAmount: string;
  };
  points: {
    availablePoints: string;
    frozenPoints: string;
    pendingPoints: string;
    totalPoints: string;
  };
}

export interface AccountPortfolioSdkPort {
  wallet: Pick<SdkworkAccountAppClient["wallet"], "portfolio">;
}

function resolveAccountSdkClient(): SdkworkAccountAppClient {
  return getAccountAppSdkClient();
}

export function createAccountPortfolioService(
  resolveClient: () => AccountPortfolioSdkPort = resolveAccountSdkClient,
) {
  return {
    async getPortfolio(): Promise<WalletPortfolio> {
      const portfolio = await resolveClient().wallet.portfolio.list();
      return {
        cash: {
          currencyCode: portfolio.cash.currencyCode ?? "CNY",
          availableAmount: portfolio.cash.availableAmount,
          frozenAmount: portfolio.cash.frozenAmount,
          pendingAmount: portfolio.cash.pendingAmount,
        },
        points: {
          availablePoints: portfolio.points.availablePoints,
          frozenPoints: portfolio.points.frozenPoints,
          pendingPoints: portfolio.points.pendingPoints,
          totalPoints: portfolio.points.totalPoints,
        },
        tokenBank: {
          availableAmount: portfolio.tokenBank.availableAmount,
          frozenAmount: portfolio.tokenBank.frozenAmount,
        },
      };
    },
  };
}

export const AccountPortfolioService = createAccountPortfolioService();
