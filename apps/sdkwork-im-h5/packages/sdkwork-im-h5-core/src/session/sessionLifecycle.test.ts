import assert from "node:assert/strict";
import test from "node:test";

import {
  notifyImH5SessionChanged,
  registerImH5SessionChangeListener,
} from "./index";

test("session lifecycle notifies composed modules and isolates teardown errors", () => {
  let notifications = 0;
  const unregisterFailing = registerImH5SessionChangeListener(() => {
    throw new Error("teardown failed");
  });
  const unregisterWorking = registerImH5SessionChangeListener(() => {
    notifications += 1;
  });

  notifyImH5SessionChanged();
  unregisterFailing();
  unregisterWorking();

  assert.equal(notifications, 1);
});
