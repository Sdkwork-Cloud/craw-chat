import { expect, test } from '@playwright/test';

const STRESS_CONVERSATION_COUNT = 10_000;
const CHAT_ROW_HEIGHT = 64;
const componentBaseUrl = process.env.PLAYWRIGHT_COMPONENT_BASE_URL
  ?? process.env.PLAYWRIGHT_BASE_URL
  ?? 'http://127.0.0.1:4176';

test('keeps a 10,000-conversation inbox virtualized and keyboard reachable', async ({ context, page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  const cdp = await context.newCDPSession(page);
  await page.goto(`${componentBaseUrl}/e2e/fixtures/conversation-list-harness.html?count=0`);
  await cdp.send('HeapProfiler.collectGarbage');
  const baselineHeap = await cdp.send('Runtime.getHeapUsage');
  await page.goto(
    `${componentBaseUrl}/e2e/fixtures/conversation-list-harness.html?count=${STRESS_CONVERSATION_COUNT}`,
  );

  const scrollRegion = page.getByTestId('chat-conversation-list');
  await expect(scrollRegion).toBeVisible({ timeout: 30_000 });

  const virtualList = scrollRegion.locator('ul');
  const mountedRows = virtualList.locator(':scope > li');
  await expect(mountedRows.first()).toBeVisible({ timeout: 30_000 });

  expect(await mountedRows.count()).toBeLessThan(40);
  await expect(virtualList).toHaveJSProperty(
    'scrollHeight',
    STRESS_CONVERSATION_COUNT * CHAT_ROW_HEIGHT,
  );

  const firstTwoBoxes = await mountedRows.evaluateAll((rows) => (
    rows.slice(0, 2).map((row) => row.getBoundingClientRect())
  ));
  expect(firstTwoBoxes).toHaveLength(2);
  expect(firstTwoBoxes[0]?.height).toBe(CHAT_ROW_HEIGHT);
  expect((firstTwoBoxes[1]?.top ?? 0) - (firstTwoBoxes[0]?.top ?? 0)).toBe(CHAT_ROW_HEIGHT);

  const firstConversation = page.getByRole('button', {
    name: /Conversation 00000/u,
  }).first();
  await firstConversation.focus();
  await expect(firstConversation).toBeFocused();
  await page.keyboard.press('Tab');
  await expect(mountedRows.nth(1).getByRole('button')).toBeFocused();

  await page.keyboard.press('End');
  await expect.poll(
    () => page.evaluate(() => window.__chatListHarnessLoadMoreCalls),
  ).toBeGreaterThan(0);

  const lastConversation = page.getByRole('button', {
    name: /Conversation 09999/u,
  });
  await expect(lastConversation).toBeVisible({ timeout: 10_000 });
  await expect(lastConversation).toBeFocused();
  expect(await mountedRows.count()).toBeLessThan(40);

  await page.keyboard.press('ArrowUp');
  await expect(page.getByRole('button', { name: /Conversation 09998/u })).toBeFocused();
  await page.keyboard.press('End');
  await expect(lastConversation).toBeFocused();

  await lastConversation.focus();
  await expect(lastConversation).toBeFocused();
  await lastConversation.click();
  await expect(lastConversation).toHaveAttribute('aria-current', 'true');

  const automaticLoadCalls = await page.evaluate(() => window.__chatListHarnessLoadMoreCalls);
  await page.getByRole('button', { name: 'Load more conversations' }).click();
  await expect.poll(
    () => page.evaluate(() => window.__chatListHarnessLoadMoreCalls),
  ).toBeGreaterThan(automaticLoadCalls);

  await lastConversation.click({ button: 'right' });
  await page.getByRole('button', { name: 'Delete conversation' }).click();
  await expect(page.getByRole('alertdialog', { name: 'Delete conversation?' })).toBeVisible();
  await page.getByRole('button', { name: 'Cancel' }).click();
  await expect(page.getByRole('alertdialog', { name: 'Delete conversation?' })).toBeHidden();

  await cdp.send('HeapProfiler.collectGarbage');
  const loadedHeap = await cdp.send('Runtime.getHeapUsage');
  for (let iteration = 0; iteration < 20; iteration += 1) {
    await scrollRegion.evaluate((element, scrollToBottom) => {
      element.scrollTop = scrollToBottom ? element.scrollHeight : 0;
      element.dispatchEvent(new Event('scroll'));
    }, iteration % 2 === 0);
  }
  await cdp.send('HeapProfiler.collectGarbage');
  const finalHeap = await cdp.send('Runtime.getHeapUsage');
  const datasetHeapGrowth = finalHeap.usedSize - baselineHeap.usedSize;
  const repeatedScrollHeapGrowth = finalHeap.usedSize - loadedHeap.usedSize;
  expect(datasetHeapGrowth).toBeLessThan(64 * 1024 * 1024);
  expect(repeatedScrollHeapGrowth).toBeLessThan(16 * 1024 * 1024);
  await testInfo.attach('conversation-list-memory.json', {
    body: JSON.stringify({
      baselineHeapBytes: baselineHeap.usedSize,
      datasetHeapGrowth,
      finalHeapBytes: finalHeap.usedSize,
      loadedHeapBytes: loadedHeap.usedSize,
      mountedRows: await mountedRows.count(),
      repeatedScrollHeapGrowth,
    }, null, 2),
    contentType: 'application/json',
  });
  await page.screenshot({
    path: testInfo.outputPath('conversation-list-10000.png'),
    fullPage: false,
  });
});
