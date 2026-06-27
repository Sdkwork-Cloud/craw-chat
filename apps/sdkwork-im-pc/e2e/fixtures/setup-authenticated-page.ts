import type { Page } from '@playwright/test';

import {
  SESSION_STORAGE_KEY,
  buildPlaywrightSessionFixture,
  buildPlaywrightSessionResponse,
} from './auth';
import {
  installPlaywrightImApiMocks,
  type PlaywrightImApiFixtureOptions,
} from './im-api';

export interface SetupAuthenticatedPageOptions {
  imApi?: PlaywrightImApiFixtureOptions | false;
}

export async function setupAuthenticatedPage(
  page: Page,
  options: SetupAuthenticatedPageOptions = {},
): Promise<void> {
  const session = buildPlaywrightSessionFixture();
  const sessionResponse = buildPlaywrightSessionResponse(session);

  await page.addInitScript(
    ({ storageKey, serializedSession }) => {
      window.sessionStorage.setItem(storageKey, serializedSession);
      window.localStorage.setItem(storageKey, serializedSession);
    },
    {
      storageKey: SESSION_STORAGE_KEY,
      serializedSession: JSON.stringify(session),
    },
  );

  await page.route('**/app/v3/api/**', async (route) => {
    if (route.request().method() !== 'GET') {
      await route.continue();
      return;
    }

    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        code: '0',
        message: 'ok',
        requestId: 'playwright.request.fallback',
        data: {},
      }),
    });
  });

  await page.route('**/app/v3/api/auth/sessions/current**', async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify(sessionResponse),
    });
  });

  if (options.imApi === false) {
    return;
  }

  await installPlaywrightImApiMocks(page, options.imApi);
}
