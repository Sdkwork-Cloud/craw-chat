import assert from "node:assert/strict";
import test from "node:test";

import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";
import { LifeService } from "./LifeService";
import { ProductService } from "./ProductService";
import { ProfileService } from "./ProfileService";
import { SettingsService } from "./SettingsService";
import { WorkService } from "./WorkService";

test("user services fail closed without composed owner SDK surfaces", async () => {
  // Audited mock-only services must fail closed (PRD): no fabricated data.
  await assert.rejects(WorkService.getMyWorks(), UserCapabilityUnavailableError);
  await assert.rejects(ProductService.getProducts(), UserCapabilityUnavailableError);
  await assert.rejects(ProductService.getCategories(), UserCapabilityUnavailableError);
  await assert.rejects(LifeService.getLifeServices(), UserCapabilityUnavailableError);
  await assert.rejects(SettingsService.getSettings(), UserCapabilityUnavailableError);
  await assert.rejects(SettingsService.updateSettings({ darkMode: true }), UserCapabilityUnavailableError);
  await assert.rejects(ProfileService.updateUserProfile({ beans: 100 }), UserCapabilityUnavailableError);
});
