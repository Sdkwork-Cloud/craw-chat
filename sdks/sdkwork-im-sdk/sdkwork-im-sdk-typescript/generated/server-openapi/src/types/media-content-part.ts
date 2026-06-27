import type { DriveReference } from './drive-reference';
import type { MediaResource } from './media-resource';

export interface MediaContentPart {
  kind: 'media';
  drive: DriveReference;
  resource: MediaResource;
  mediaRole?: string | null;
}
