import { configureCompanyPcHost } from '@sdkwork/company-pc-company';
import '@sdkwork/company-pc-company/i18n';
import {
  createImCompanyPcHostAdapter,
  type CreateImCompanyPcHostAdapterOptions,
} from './createImCompanyPcHostAdapter';

let companyPcHostBootstrapped = false;

export function bootstrapImCompanyPcHost(
  options: CreateImCompanyPcHostAdapterOptions,
): void {
  configureCompanyPcHost(createImCompanyPcHostAdapter(options));
  companyPcHostBootstrapped = true;
}

export function isImCompanyPcHostBootstrapped(): boolean {
  return companyPcHostBootstrapped;
}

export function resetImCompanyPcHostBootstrap(): void {
  companyPcHostBootstrapped = false;
}
