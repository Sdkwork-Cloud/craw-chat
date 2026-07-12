export interface MentionContentPart {
  kind: 'mention';
  targetKind: 'agent';
  targetId: string;
  displayText: string;
  assignmentGeneration: string;
}
