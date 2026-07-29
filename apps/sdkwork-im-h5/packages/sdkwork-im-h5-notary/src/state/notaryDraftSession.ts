import { uuid } from "@sdkwork/utils";

import type {
  NotaryDraftAttachment,
  NotaryDraftParty,
  NotaryStaffMember,
} from "../services/notaryService";

export interface NotaryDraftPartyWithId extends NotaryDraftParty {
  id: string;
}

export interface NotaryDraftState {
  step: number;
  selectedType: string;
  selectedNotary: string;
  selectedNotaryObj: NotaryStaffMember | null;
  parties: NotaryDraftPartyWithId[];
  applicationInfo: string;
  attachments: NotaryDraftAttachment[];
  submissionIdempotencyKey: string;
}

export type NotaryPartyEditor =
  | { mode: "add" }
  | { mode: "edit" | "readonly"; partyId: string };

export interface NotaryDraftSessionDependencies {
  createId: () => string;
  revokeObjectUrl: (url: string) => void;
}

const defaultDependencies: NotaryDraftSessionDependencies = {
  createId: uuid,
  revokeObjectUrl: (url) => URL.revokeObjectURL(url),
};

export class NotaryDraftSession {
  private draft: NotaryDraftState;
  private partyEditor: NotaryPartyEditor | null = null;
  private notarySelectionOpen = false;

  constructor(
    private readonly dependencies: NotaryDraftSessionDependencies = defaultDependencies,
  ) {
    this.draft = this.createEmptyDraft();
  }

  getDraft(): NotaryDraftState {
    return {
      ...this.draft,
      selectedNotaryObj: this.draft.selectedNotaryObj
        ? { ...this.draft.selectedNotaryObj }
        : null,
      parties: this.draft.parties.map((party) => ({ ...party })),
      attachments: [...this.draft.attachments],
    };
  }

  replaceDraft(draft: NotaryDraftState): void {
    this.draft = {
      ...draft,
      selectedNotaryObj: draft.selectedNotaryObj
        ? { ...draft.selectedNotaryObj }
        : null,
      parties: draft.parties.map((party) => ({ ...party })),
      attachments: [...draft.attachments],
    };
  }

  openPartyEditor(editor: NotaryPartyEditor): void {
    this.partyEditor = editor;
  }

  getPartyEditor(): {
    mode: NotaryPartyEditor["mode"];
    party: NotaryDraftPartyWithId | null;
  } | null {
    if (!this.partyEditor) {
      return null;
    }
    if (this.partyEditor.mode === "add") {
      return { mode: "add", party: null };
    }
    const party = this.draft.parties.find(
      (candidate) => candidate.id === this.partyEditor?.partyId,
    );
    return party
      ? { mode: this.partyEditor.mode, party: { ...party } }
      : null;
  }

  closePartyEditor(): void {
    this.partyEditor = null;
  }

  addParty(party: NotaryDraftPartyWithId): void {
    if (this.draft.parties.some((candidate) => candidate.id === party.id)) {
      throw new TypeError(`Duplicate notary party id: ${party.id}`);
    }
    this.draft.parties = [...this.draft.parties, { ...party }];
  }

  updateParty(party: NotaryDraftPartyWithId): void {
    const index = this.draft.parties.findIndex(
      (candidate) => candidate.id === party.id,
    );
    if (index < 0) {
      throw new TypeError(`Unknown notary party id: ${party.id}`);
    }
    this.draft.parties = this.draft.parties.map((candidate) =>
      candidate.id === party.id ? { ...party } : candidate,
    );
  }

  openNotarySelection(): void {
    this.notarySelectionOpen = true;
  }

  isNotarySelectionOpen(): boolean {
    return this.notarySelectionOpen;
  }

  selectNotary(staff: NotaryStaffMember): void {
    if (!this.notarySelectionOpen) {
      throw new TypeError("Notary selection is not active");
    }
    this.draft.selectedNotary = staff.id;
    this.draft.selectedNotaryObj = { ...staff };
    this.notarySelectionOpen = false;
  }

  closeNotarySelection(): void {
    this.notarySelectionOpen = false;
  }

  reset(): void {
    const previewUrls = new Set(
      this.draft.attachments
        .map((attachment) => attachment.previewUrl)
        .filter((url): url is string => Boolean(url)),
    );
    for (const previewUrl of previewUrls) {
      this.dependencies.revokeObjectUrl(previewUrl);
    }
    this.partyEditor = null;
    this.notarySelectionOpen = false;
    this.draft = this.createEmptyDraft();
  }

  private createEmptyDraft(): NotaryDraftState {
    return {
      step: 1,
      selectedType: "",
      selectedNotary: "",
      selectedNotaryObj: null,
      parties: [],
      applicationInfo: "",
      attachments: [],
      submissionIdempotencyKey: this.dependencies.createId(),
    };
  }
}

export const notaryDraftSession = new NotaryDraftSession();
