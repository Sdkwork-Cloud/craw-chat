export {
  CompanyView,
  CompanyViewWithI18n,
  enterpriseMarketplaceService,
  enterpriseService,
  configureCompanyPcHost,
  configureCompanyMarketplacePort,
  CompanyPcHostProvider,
} from '@sdkwork/company-pc-company';

export type {
  CompanyPcHostAdapter,
  EnterpriseMarketplaceService,
  EnterpriseViewProps,
} from '@sdkwork/company-pc-company';

export {
  bootstrapImCompanyPcHost,
  isImCompanyPcHostBootstrapped,
  resetImCompanyPcHostBootstrap,
} from './bootstrapImCompanyPcHost';
export {
  createImCompanyPcHostAdapter,
  type CreateImCompanyPcHostAdapterOptions,
} from './createImCompanyPcHostAdapter';
