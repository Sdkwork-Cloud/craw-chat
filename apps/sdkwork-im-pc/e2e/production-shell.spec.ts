import { expect, test } from '@playwright/test';

test.describe('production shell', () => {
  test('serves the React mount point and document title', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('#root')).toBeAttached();
    await expect(page).toHaveTitle(/.+/);
  });

  test('loads static assets without server errors', async ({ page }) => {
    const failedResponses: string[] = [];
    page.on('response', (response) => {
      if (response.status() >= 500) {
        failedResponses.push(`${response.status()} ${response.url()}`);
      }
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');
    expect(failedResponses).toEqual([]);
  });
});
