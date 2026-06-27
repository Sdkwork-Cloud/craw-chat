export interface EnterpriseRecruitListing {
  id: string;
  title: string;
  company: string;
  salary: string;
  location: string;
  exp: string;
  edu: string;
  tags: string[];
  updated: string;
}

export interface EnterpriseSupplyListing {
  id: string;
  title: string;
  company: string;
  category: string;
  price: string;
  unit: string;
  minOrder: string;
  date: string;
  description: string;
  status: string;
}

export interface EnterprisePurchaseListing {
  id: string;
  title: string;
  company: string;
  budget: string;
  deadline: string;
  location: string;
  status: string;
  description: string;
}

export interface EnterpriseProductListing {
  id: string;
  title: string;
  description: string;
}

export interface EnterpriseJobListing {
  id: string;
  title: string;
  location: string;
  exp: string;
  edu: string;
  salary: string;
  tags: string[];
}

export const PC_ENTERPRISE_MARKETPLACE_CONTRACT_UNAVAILABLE =
  'pc enterprise marketplace contract is not available';

function failClosedMarketplaceMutation(): never {
  throw new Error(PC_ENTERPRISE_MARKETPLACE_CONTRACT_UNAVAILABLE);
}

export interface EnterpriseMarketplaceService {
  getRecruits(): Promise<EnterpriseRecruitListing[]>;
  getSupplies(): Promise<EnterpriseSupplyListing[]>;
  getPurchases(): Promise<EnterprisePurchaseListing[]>;
  getEnterpriseRecruits(enterpriseId: string): Promise<EnterpriseJobListing[]>;
  getEnterpriseProducts(enterpriseId: string): Promise<EnterpriseProductListing[]>;
  applyRecruit(recruitId: string): Promise<void>;
  contactSupplier(supplyId: string): Promise<void>;
  contactPurchaseBuyer(purchaseId: string): Promise<void>;
  publishListing(): Promise<void>;
}

class SdkworkEnterpriseMarketplaceService implements EnterpriseMarketplaceService {
  async getRecruits(): Promise<EnterpriseRecruitListing[]> {
    return [];
  }

  async getSupplies(): Promise<EnterpriseSupplyListing[]> {
    return [];
  }

  async getPurchases(): Promise<EnterprisePurchaseListing[]> {
    return [];
  }

  async getEnterpriseRecruits(_enterpriseId: string): Promise<EnterpriseJobListing[]> {
    return [];
  }

  async getEnterpriseProducts(_enterpriseId: string): Promise<EnterpriseProductListing[]> {
    return [];
  }

  async applyRecruit(_recruitId: string): Promise<void> {
    failClosedMarketplaceMutation();
  }

  async contactSupplier(_supplyId: string): Promise<void> {
    failClosedMarketplaceMutation();
  }

  async contactPurchaseBuyer(_purchaseId: string): Promise<void> {
    failClosedMarketplaceMutation();
  }

  async publishListing(): Promise<void> {
    failClosedMarketplaceMutation();
  }
}

export function createSdkworkEnterpriseMarketplaceService(): EnterpriseMarketplaceService {
  return new SdkworkEnterpriseMarketplaceService();
}

export const enterpriseMarketplaceService = createSdkworkEnterpriseMarketplaceService();
