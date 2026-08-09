import assert from "node:assert/strict";
import test from "node:test";

import {
  notifyImH5SessionChanged,
  registerImH5SessionChangeListener,
  registerImH5SessionLogoutHandler,
  requestImH5SessionLogout,
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

test("logout request delegates to the registered app-owned executor", async () => {
  let calls = 0;
  const unregister = registerImH5SessionLogoutHandler(async () => {
    calls += 1;
  });

  await requestImH5SessionLogout();
  unregister();

  assert.equal(calls, 1);
});

test("logout request is a safe no-op without a registered executor", async () => {
  await requestImH5SessionLogout();
});

test("unregistering the logout executor stops further delegation", async () => {
  let calls = 0;
  const unregister = registerImH5SessionLogoutHandler(async () => {
    calls += 1;
  });
  unregister();

  await requestImH5SessionLogout();

  assert.equal(calls, 0);
});

test("logout executor failures propagate to the requester", async () => {
  const unregister = registerImH5SessionLogoutHandler(async () => {
    throw new Error("logout failed");
  });
  try {
    await assert.rejects(
      requestImH5SessionLogout(),
      /logout failed/,
    );
  } finally {
    unregister();
  }
});
