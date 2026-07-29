import assert from "node:assert/strict";
import test from "node:test";

import { NotaryDraftSession } from "./notaryDraftSession";

function createSession() {
  let sequence = 0;
  const revokedUrls: string[] = [];
  const session = new NotaryDraftSession({
    createId: () => `draft-${++sequence}`,
    revokeObjectUrl: (url) => revokedUrls.push(url),
  });
  return { revokedUrls, session };
}

test("reset releases draft resources and starts a new logical submission", () => {
  const { revokedUrls, session } = createSession();
  const initialDraft = session.getDraft();
  const image = new File(["image"], "evidence.png", { type: "image/png" });
  const video = new File(["video"], "evidence.mp4", { type: "video/mp4" });
  const pdf = new File(["pdf"], "evidence.pdf", { type: "application/pdf" });

  session.replaceDraft({
    ...initialDraft,
    step: 3,
    selectedType: "matter-1",
    selectedNotary: "staff-1",
    selectedNotaryObj: {
      id: "staff-1",
      name: "Reviewer",
      organization: "Notary Office",
      active: true,
      initial: "R",
    },
    parties: [{ id: "party-1", name: "Applicant", idCard: "11010519900101234X" }],
    applicationInfo: "Preserve evidence",
    attachments: [
      {
        id: "attachment-1",
        name: image.name,
        file: image,
        previewUrl: "blob:shared-preview",
        type: "image",
        size: "5 B",
      },
      {
        id: "attachment-2",
        name: video.name,
        file: video,
        previewUrl: "blob:shared-preview",
        type: "video",
        size: "5 B",
      },
      {
        id: "attachment-3",
        name: pdf.name,
        file: pdf,
        type: "file",
        size: "3 B",
      },
    ],
  });
  session.openPartyEditor({ mode: "edit", partyId: "party-1" });
  session.openNotarySelection();

  session.reset();

  const resetDraft = session.getDraft();
  assert.deepEqual(revokedUrls, ["blob:shared-preview"]);
  assert.equal(resetDraft.step, 1);
  assert.equal(resetDraft.selectedType, "");
  assert.equal(resetDraft.selectedNotary, "");
  assert.equal(resetDraft.selectedNotaryObj, null);
  assert.deepEqual(resetDraft.parties, []);
  assert.equal(resetDraft.applicationInfo, "");
  assert.deepEqual(resetDraft.attachments, []);
  assert.notEqual(
    resetDraft.submissionIdempotencyKey,
    initialDraft.submissionIdempotencyKey,
  );
  assert.equal(session.getPartyEditor(), null);
  assert.equal(session.isNotarySelectionOpen(), false);
});

test("editor and notary selection operations fail closed", () => {
  const { session } = createSession();
  const staff = {
    id: "staff-1",
    name: "Reviewer",
    organization: "Notary Office",
    active: true,
    initial: "R",
  };

  assert.equal(session.getPartyEditor(), null);
  assert.throws(() => session.selectNotary(staff), /not active/);
  assert.throws(
    () => session.updateParty({ id: "unknown", name: "Missing", idCard: "1" }),
    /Unknown notary party id/,
  );
  session.openPartyEditor({ mode: "edit", partyId: "unknown" });
  assert.equal(session.getPartyEditor(), null);
});

test("readonly previews cannot leak mutable state into later editors", () => {
  const { session } = createSession();
  session.addParty({
    id: "party-1",
    name: "Applicant",
    idCard: "11010519900101234X",
  });
  session.openPartyEditor({ mode: "readonly", partyId: "party-1" });

  const readonlyEditor = session.getPartyEditor();
  assert.equal(readonlyEditor?.mode, "readonly");
  assert.equal(readonlyEditor?.party?.name, "Applicant");
  if (readonlyEditor?.party) {
    readonlyEditor.party.name = "Mutated Preview";
  }
  assert.equal(session.getDraft().parties[0]?.name, "Applicant");

  session.closePartyEditor();
  session.openPartyEditor({ mode: "add" });
  assert.deepEqual(session.getPartyEditor(), { mode: "add", party: null });
});
