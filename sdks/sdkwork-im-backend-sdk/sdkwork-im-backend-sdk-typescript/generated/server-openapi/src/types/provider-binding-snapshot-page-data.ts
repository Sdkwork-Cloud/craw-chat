import type { PageInfo } from './page-info';
import type { ProviderBindingSnapshot } from './provider-binding-snapshot';

export interface ProviderBindingSnapshotPageData {
  items: ProviderBindingSnapshot[];
  pageInfo: PageInfo;
}
