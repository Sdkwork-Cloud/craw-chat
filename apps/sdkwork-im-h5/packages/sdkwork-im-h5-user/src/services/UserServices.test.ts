import assert from "node:assert/strict";
import test from "node:test";

import { CharacterService } from "./CharacterService";
import { LifeService } from "./LifeService";
import { MomentService } from "./MomentService";
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

  const characters = await CharacterService.getCharacters();
  assert.ok(Array.isArray(characters));
  const added = await CharacterService.addCharacter({ avatar: "", desc: "", name: "Character" });
  assert.equal(added.name, "Character");

  const works = await WorkService.getMyWorks();
  assert.ok(Array.isArray(works));

  const moments = await MomentService.getMoments();
  assert.ok(Array.isArray(moments));

  const products = await ProductService.getProducts();
  assert.ok(Array.isArray(products));

  const lifeServices = await LifeService.getLifeServices();
  assert.ok(Array.isArray(lifeServices));
});
