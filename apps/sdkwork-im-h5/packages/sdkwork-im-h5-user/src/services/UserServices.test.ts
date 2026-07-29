import assert from "node:assert/strict";
import test from "node:test";

import { CharacterService } from "./CharacterService";
import { LifeService } from "./LifeService";
import { MomentService } from "./MomentService";
import { ProductService } from "./ProductService";
import { ProfileService } from "./ProfileService";
import { SettingsService } from "./SettingsService";
import { UserCapabilityUnavailableError } from "./UserCapabilityUnavailableError";
import { WorkService } from "./WorkService";

test("non-auth user operations fail closed without owner SDK composition", async () => {
  const work = {
    comments: 0,
    coverUrl: "",
    createdAt: "",
    id: "work-id",
    likes: 0,
    title: "Work",
    type: "article" as const,
    views: 0,
  };

  for (const operation of [
    () => ProfileService.getUserProfile(),
    () => ProfileService.updateUserProfile({ name: "Updated" }),
    () => SettingsService.getSettings(),
    () => SettingsService.updateSettings({ darkMode: true }),
    () => CharacterService.getCharacters(),
    () => CharacterService.addCharacter({ avatar: "", desc: "", name: "Character" }),
    () => CharacterService.editCharacter("character-id", { name: "Updated" }),
    () => WorkService.getMyWorks(),
    () => WorkService.deleteWork("work-id"),
    () => WorkService.updateWork("work-id", { title: "Updated" }),
    () => WorkService.addWork(work),
    () => MomentService.getMoments(),
    () => MomentService.addMoment("Content"),
    () => MomentService.toggleLike("moment-id", "user-id"),
    () => MomentService.addComment("moment-id", "Author", "Comment"),
    () => MomentService.deleteMoment("moment-id"),
    () => ProductService.getProducts(),
    () => ProductService.getCategories(),
    () => LifeService.getLifeServices(),
  ]) {
    await assert.rejects(operation, UserCapabilityUnavailableError);
  }
});
