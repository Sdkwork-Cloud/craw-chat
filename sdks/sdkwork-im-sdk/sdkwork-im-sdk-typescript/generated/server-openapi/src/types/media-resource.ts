import type { MediaKind } from './media-kind';
import type { MediaSource } from './media-source';

export interface MediaResource {
  id?: string | null;
  kind?: MediaKind | null;
  mediaKind?: MediaKind | null;
  source: MediaSource;
  uri: string;
  publicUrl?: string | null;
  url?: string | null;
  name?: string | null;
  title?: string | null;
  fileName?: string | null;
  mimeType?: string | null;
  size?: string | null;
  sizeBytes?: string | null;
  fileSize?: string | null;
  durationSeconds?: number | null;
  poster?: MediaResource | null;
  thumbnails?: MediaResource[];
}
