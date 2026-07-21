import type { PageInfo } from './page-info';
import type { ProviderBindingDriftItem } from './provider-binding-drift-item';

export interface ProviderBindingDriftPageData {
  items: ProviderBindingDriftItem[];
  pageInfo: PageInfo;
}
