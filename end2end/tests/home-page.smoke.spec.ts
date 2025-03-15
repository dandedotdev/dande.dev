import { test, expect } from '@playwright/test';

test('homepage has title and heading text', async ({ page }) => {
  await page.goto('http://localhost:8080/');

  await expect(page).toHaveTitle('Dande.dev');
  await expect(page.locator('h1')).toHaveText('Dandelion Huang');
});
