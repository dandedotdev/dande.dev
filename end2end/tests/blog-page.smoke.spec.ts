import { test, expect } from '@playwright/test';

test('homepage has title and heading text', async ({ page }) => {
  await page.goto('http://localhost:8080/blog');

  await expect(page).toHaveTitle('Blog | Dande.dev');
  await expect(page.locator('h1')).toHaveText('All Posts');
});
