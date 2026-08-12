/**
 * IM H5 company (企业中心) runtime port wiring.
 *
 * Binds the shared `@sdkwork/company-mobile-react-enterprise` package to the
 * IM host: `configureCompanyRuntimePort` switches the package to the
 * generated Company App SDK port constructed from the IM gateway base URL
 * and the shared H5 token manager. Without this binding the enterprise
 * center pages fail closed with `CompanyCapabilityUnavailableError`.
 */

import { configureCompanyRuntimePort } from '@sdkwork/company-mobile-react-enterprise';
import { getSdkClients } from './sdkClients';

let bootstrapped = false;

export function bootstrapImCompanyH5Port(): void {
  if (bootstrapped) {
    return;
  }
  bootstrapped = true;

  configureCompanyRuntimePort(getSdkClients().companyAppSdkPort);
}

export function isImCompanyH5PortBootstrapped(): boolean {
  return bootstrapped;
}
