import type { DataContentPart } from './data-content-part';
import type { MediaContentPart } from './media-content-part';
import type { SignalContentPart } from './signal-content-part';
import type { StreamRefContentPart } from './stream-ref-content-part';
import type { TextContentPart } from './text-content-part';

export type ContentPart = TextContentPart | DataContentPart | MediaContentPart | SignalContentPart | StreamRefContentPart;
