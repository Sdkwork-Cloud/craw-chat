import assert from "node:assert/strict";
import test from "node:test";

import { LifeService } from "./LifeService";
import { ProductService } from "./ProductService";
import { ProfileService } from "./ProfileService";
import { SettingsService } from "./SettingsService";
import { WorkService } from "./WorkService";

test("user services return composed data", async () => {
  const profile = await ProfileService.getUserProfile();
  assert.ok(profile);
  const updated = await ProfileService.updateUserProfile({ name: "Updated" });
  assert.equal(updated.name, "Updated");

  const settings = await SettingsService.getSettings();
  assert.ok(settings);
  const settingsUpdated = await SettingsService.updateSettings({ darkMode: true });
  assert.equal(settingsUpdated.darkMode, true);

  const works = await WorkService.getMyWorks();
  assert.ok(Array.isArray(works));

  const products = await ProductService.getProducts();
  assert.ok(Array.isArray(products));

  const lifeServices = await LifeService.getLifeServices();
  assert.ok(Array.isArray(lifeServices));
});
