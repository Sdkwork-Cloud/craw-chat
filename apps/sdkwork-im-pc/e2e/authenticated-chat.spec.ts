import { expect, test } from '@playwright/test';

import {
  PLAYWRIGHT_INITIAL_MESSAGE,
  PLAYWRIGHT_PEER_DISPLAY_NAME,
} from './fixtures/im-api';
import { setupAuthenticatedPage } from './fixtures/setup-authenticated-page';

test.describe('authenticated chat flow', () => {
  test.beforeEach(async ({ page }) => {
    await setupAuthenticatedPage(page);
  });

  test('renders inbox conversation and sends an outbound text message', async ({ page }) => {
    const outboundMessage = 'E2E outbound message from Playwright';

    await page.goto('/');
    await page.waitForLoadState('networkidle');

    await expect(page).not.toHaveURL(/\/auth\/login/u);
    await expect(
      page.getByRole('button', { name: new RegExp(PLAYWRIGHT_PEER_DISPLAY_NAME, 'u') }),
    ).toBeVisible({
      timeout: 20_000,
    });
    await expect(
      page.locator('#msg-message\\.playwright\\.1').getByText(PLAYWRIGHT_INITIAL_MESSAGE, { exact: true }),
    ).toBeVisible({
      timeout: 20_000,
    });

    const composer = page.locator('.ProseMirror').last();
    await expect(composer).toBeVisible({ timeout: 10_000 });
    await composer.click();
    await composer.fill(outboundMessage);
    await composer.press('Enter');

    await expect(page.getByText(outboundMessage, { exact: true }).last()).toBeVisible({
      timeout: 10_000,
    });
  });
});
