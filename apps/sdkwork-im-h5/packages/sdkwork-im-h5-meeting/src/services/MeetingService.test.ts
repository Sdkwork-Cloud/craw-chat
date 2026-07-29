import assert from "node:assert/strict";
import test from "node:test";

import { MeetingCapabilityUnavailableError, MeetingService } from "./MeetingService";

test("meeting operations fail closed until the owner SDK is composed", async () => {
  const request = {
    attendeeIds: ["user-id"],
    endTime: "2026-07-29T10:00:00Z",
    startTime: "2026-07-29T09:00:00Z",
    title: "test",
  };
  for (const operation of [
    MeetingService.getMeetings(),
    MeetingService.getMeetingDetail("meeting-id"),
    MeetingService.createMeeting(request),
    MeetingService.updateMeeting({ id: "meeting-id", title: "updated" }),
    MeetingService.cancelMeeting("meeting-id"),
  ]) {
    await assert.rejects(operation, MeetingCapabilityUnavailableError);
  }
});
