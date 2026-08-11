import assert from "node:assert/strict";
import test from "node:test";

import { LifeService, LifeServiceCapabilityUnavailableError } from "./LifeService";
import { ProductService, ProductCapabilityUnavailableError } from "./ProductService";
import { ProfileService } from "./ProfileService";
import { SettingsService } from "./SettingsService";
import { WorkService, WorkCapabilityUnavailableError } from "./WorkService";

test("user services return real profile/settings and fail closed for mock-only surfaces", async () => {
  const profile = await ProfileService.getUserProfile();
  assert.ok(profile);
  const updated = await ProfileService.updateUserProfile({ name: "Updated" });
  assert.equal(updated.name, "Updated");

  const settings = await SettingsService.getSettings();
  assert.ok(settings);
  const settingsUpdated = await SettingsService.updateSettings({ darkMode: true });
  assert.equal(settingsUpdated.darkMode, true);

  // Audited mock-only services must fail closed (PRD): no fabricated data.
  await assert.rejects(WorkService.getMyWorks(), WorkCapabilityUnavailableError);
  await assert.rejects(ProductService.getProducts(), ProductCapabilityUnavailableError);
  await assert.rejects(ProductService.getCategories(), ProductCapabilityUnavailableError);
  await assert.rejects(LifeService.getLifeServices(), LifeServiceCapabilityUnavailableError);
});
