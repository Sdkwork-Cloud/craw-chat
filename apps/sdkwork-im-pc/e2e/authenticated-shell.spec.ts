import { expect, test } from '@playwright/test';

import { setupAuthenticatedPage } from './fixtures/setup-authenticated-page';

test.describe('authenticated production shell', () => {
  test.beforeEach(async ({ page }) => {
    await setupAuthenticatedPage(page, { imApi: false });
    await page.route('**/im/v3/api/**', async (route) => {
      if (route.request().method() === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ items: [], hasMore: false }),
        });
        return;
      }

      await route.continue();
    });
  });

  test('keeps authenticated users out of the login route', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await expect(page).not.toHaveURL(/\/auth\/login/u);
    await expect(page.locator('#root')).toBeAttached();
  });
});
