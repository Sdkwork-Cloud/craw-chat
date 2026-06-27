import type { InteractionActorView } from './interaction-actor-view';

export interface MessagePinView {
  pinnedBy: InteractionActorView;
  pinnedAt: string;
}
