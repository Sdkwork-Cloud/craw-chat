import { expect, test } from '@playwright/test';

const componentBaseUrl = process.env.PLAYWRIGHT_COMPONENT_BASE_URL
  ?? process.env.PLAYWRIGHT_BASE_URL
  ?? 'http://127.0.0.1:4176';

test('supports localized search, shortcut management, focus containment, and Drive navigation', async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto(`${componentBaseUrl}/e2e/fixtures/workspace-harness.html`);

  const search = page.getByRole('searchbox', { name: 'Search workspace' });
  await expect(search).toBeVisible();
  await page.keyboard.press('Control+K');
  await expect(search).toBeFocused();

  await search.fill('knowledge');
  await expect(page.getByRole('button', { name: 'Open Knowledge Base' })).toBeVisible();
  await expect(page.getByText('Roadmap.docx')).toBeHidden();
  await page.getByRole('button', { name: 'Clear search' }).click();

  await page.getByRole('button', { name: 'Manage Shortcuts' }).click();
  const dialog = page.getByRole('dialog', { name: 'Manage Shortcuts' });
  await expect(dialog).toBeVisible();
  await expect(dialog).toBeFocused();

  await page.keyboard.press('Shift+Tab');
  await expect(page.getByRole('button', { name: 'Save' })).toBeFocused();
  await page.keyboard.press('Tab');
  await expect(page.getByRole('button', { name: 'Close' })).toBeFocused();

  const knowledgeCheckbox = page.getByRole('checkbox', { name: 'Knowledge Base' });
  await knowledgeCheckbox.focus();
  await page.keyboard.press('Control+K');
  await expect(knowledgeCheckbox).toBeFocused();
  await knowledgeCheckbox.uncheck();
  await page.getByRole('button', { name: 'Save' }).click();
  await expect(dialog).toBeHidden();
  await expect(page.getByRole('button', { name: 'Open Knowledge Base' })).toBeHidden();
  await expect.poll(() => page.evaluate(() => window.__workspaceHarnessSavedPinnedIds)).toEqual([
    'notary',
    'drive',
  ]);

  await page.getByRole('button', { name: 'View All' }).click();
  await expect.poll(() => page.evaluate(() => window.__workspaceHarnessLastAppId)).toBe('drive');
  await expect(page.getByText('loading', { exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Open Drive for Roadmap.docx' }).click();
  await expect.poll(() => page.evaluate(() => window.__workspaceHarnessLastAppId)).toBe('drive');

  const initialLoadCalls = await page.evaluate(() => window.__workspaceHarnessLoadCalls);
  await page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('sdkwork-im-pc:language-changed', {
      detail: { lang: 'zh-CN' },
    }));
  });
  await expect(page.getByRole('button', { name: '管理快捷应用' })).toBeVisible();
  await page.evaluate(() => {
    window.dispatchEvent(new CustomEvent('sdkwork-im-pc:language-changed', {
      detail: { lang: 'fr-FR' },
    }));
  });
  await expect(page.getByRole('button', { name: '管理快捷应用' })).toBeVisible();
  await expect.poll(() => page.evaluate(() => window.__workspaceHarnessLoadCalls)).toBe(initialLoadCalls);

  await page.screenshot({ path: testInfo.outputPath('workspace-desktop.png'), fullPage: false });
});

test('shows retryable fallback, unexpected error, and empty states', async ({ page }) => {
  await page.goto(`${componentBaseUrl}/e2e/fixtures/workspace-harness.html?mode=fallback`);
  await expect(page.getByText('Some live data is unavailable. Local or default content is shown.')).toBeVisible();
  await page.getByRole('button', { name: 'Retry' }).click();
  await expect.poll(() => page.evaluate(() => window.__workspaceHarnessLoadCalls)).toBe(2);

  await page.goto(`${componentBaseUrl}/e2e/fixtures/workspace-harness.html?mode=retry`);
  await expect(page.getByRole('heading', { name: 'Failed to load workspace data' })).toBeVisible();
  await page.getByRole('button', { name: 'Retry' }).click();
  await expect(page.getByRole('button', { name: 'Open Knowledge Base' })).toBeVisible();

  await page.goto(`${componentBaseUrl}/e2e/fixtures/workspace-harness.html?mode=empty`);
  await expect(page.getByText('No frequent apps selected')).toBeVisible();
  await expect(page.getByText('No Recent Docs')).toBeVisible();

  await page.goto(`${componentBaseUrl}/e2e/fixtures/workspace-harness.html?mode=permission`);
  await expect(page.getByText('You do not have permission to load some workspace data.')).toBeVisible();
});

test('recovers from a module error when returning to the workspace', async ({ page }) => {
  await page.goto(`${componentBaseUrl}/e2e/fixtures/workspace-harness.html?mode=module-error`);
  await expect(page.getByRole('heading', { name: 'Module unavailable' })).toBeVisible();
  await expect(page.getByText('sensitive-module-internal-detail')).toBeHidden();
  await page.getByRole('button', { name: 'Return to workspace' }).click();
  await expect(page.getByText('Workspace recovered')).toBeVisible();
});

test('keeps the workbench within a narrow tablet viewport', async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 768, height: 900 });
  await page.goto(`${componentBaseUrl}/e2e/fixtures/workspace-harness.html`);
  await expect(page.getByRole('searchbox', { name: 'Search workspace' })).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(768);
  await page.screenshot({ path: testInfo.outputPath('workspace-tablet.png'), fullPage: false });
});
