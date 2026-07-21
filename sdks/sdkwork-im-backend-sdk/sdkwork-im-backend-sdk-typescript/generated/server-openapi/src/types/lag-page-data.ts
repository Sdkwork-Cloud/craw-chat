import type { LagItem } from './lag-item';
import type { PageInfo } from './page-info';

export interface LagPageData {
  items: LagItem[];
  pageInfo: PageInfo;
}
