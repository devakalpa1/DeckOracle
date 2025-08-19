import { test, expect } from '@playwright/test';

test.describe('DeckOracle User Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('complete user flow: create deck → add cards → study', async ({ page }) => {
    // Step 1: Navigate to home page
    await expect(page).toHaveTitle(/DeckOracle/);
    await expect(page.locator('h1')).toContainText('Welcome to DeckOracle');

    // Step 2: Navigate to decks page
    await page.click('text=Browse Decks');
    await expect(page).toHaveURL(/\/decks/);

    // Step 3: Create a new deck (assuming button exists)
    await page.click('text=+ New Deck');
    
    // Fill in deck creation form
    await page.fill('input[name="name"]', 'Test Deck for E2E');
    await page.fill('textarea[name="description"]', 'This is a test deck created during E2E testing');
    await page.click('button[type="submit"]');

    // Step 4: Verify deck was created and navigate to it
    await expect(page.locator('text=Test Deck for E2E')).toBeVisible();
    await page.click('text=Test Deck for E2E');

    // Step 5: Add cards to the deck
    await page.click('text=Add Card');
    
    // Add first card
    await page.fill('input[name="front"]', 'What is Playwright?');
    await page.fill('textarea[name="back"]', 'A framework for Web Testing and Automation');
    await page.fill('input[name="tags"]', 'testing, automation');
    await page.click('button:has-text("Save Card")');

    // Add second card
    await page.click('text=Add Card');
    await page.fill('input[name="front"]', 'What is E2E testing?');
    await page.fill('textarea[name="back"]', 'End-to-end testing that validates the entire application flow');
    await page.fill('input[name="tags"]', 'testing, qa');
    await page.click('button:has-text("Save Card")');

    // Step 6: Start study session
    await page.click('text=Study Now');
    await expect(page).toHaveURL(/\/study/);

    // Step 7: Study the cards
    // First card
    await expect(page.locator('.card-front')).toContainText('What is Playwright?');
    await page.click('.flip-card-button');
    await expect(page.locator('.card-back')).toContainText('A framework for Web Testing and Automation');
    
    // Mark as correct
    await page.click('button:has-text("Easy")');

    // Second card
    await expect(page.locator('.card-front')).toContainText('What is E2E testing?');
    await page.click('.flip-card-button');
    await expect(page.locator('.card-back')).toContainText('End-to-end testing');
    
    // Mark as medium difficulty
    await page.click('button:has-text("Medium")');

    // Step 8: Complete study session
    await expect(page.locator('text=Study session complete!')).toBeVisible();
    await expect(page.locator('text=Cards studied: 2')).toBeVisible();
  });

  test('CSV import and export flow', async ({ page }) => {
    // Navigate to decks page
    await page.click('text=Browse Decks');
    
    // Create a new deck
    await page.click('text=+ New Deck');
    await page.fill('input[name="name"]', 'CSV Test Deck');
    await page.fill('textarea[name="description"]', 'Testing CSV import/export');
    await page.click('button[type="submit"]');

    // Navigate to the deck
    await page.click('text=CSV Test Deck');

    // Import CSV
    await page.click('text=Import CSV');
    
    // Create CSV content
    const csvContent = `front,back,tags
"What is JavaScript?","A programming language","programming,web"
"What is HTML?","HyperText Markup Language","web,markup"
"What is CSS?","Cascading Style Sheets","web,styling"`;

    // Upload CSV file
    await page.setInputFiles('input[type="file"]', {
      name: 'test-cards.csv',
      mimeType: 'text/csv',
      buffer: Buffer.from(csvContent),
    });

    await page.click('button:has-text("Import")');

    // Verify cards were imported
    await expect(page.locator('text=3 cards imported successfully')).toBeVisible();
    await expect(page.locator('text=What is JavaScript?')).toBeVisible();
    await expect(page.locator('text=What is HTML?')).toBeVisible();
    await expect(page.locator('text=What is CSS?')).toBeVisible();

    // Export CSV
    const downloadPromise = page.waitForEvent('download');
    await page.click('text=Export CSV');
    const download = await downloadPromise;

    // Verify download
    expect(download.suggestedFilename()).toContain('.csv');
    
    // Read the downloaded file
    const path = await download.path();
    expect(path).toBeTruthy();
  });

  test('folder organization flow', async ({ page }) => {
    // Navigate to decks page
    await page.click('text=Browse Decks');

    // Create a folder
    await page.click('text=New Folder');
    await page.fill('input[name="folderName"]', 'Programming Languages');
    await page.fill('input[name="color"]', '#549aab');
    await page.click('button:has-text("Create Folder")');

    // Verify folder was created
    await expect(page.locator('text=Programming Languages')).toBeVisible();

    // Create a deck inside the folder
    await page.click('text=Programming Languages');
    await page.click('text=+ New Deck');
    await page.fill('input[name="name"]', 'JavaScript Basics');
    await page.click('button[type="submit"]');

    // Verify deck is in the folder
    await expect(page.locator('text=JavaScript Basics')).toBeVisible();
    
    // Navigate back and verify folder shows deck count
    await page.click('text=Back to Folders');
    await expect(page.locator('text=Programming Languages')).toBeVisible();
    await expect(page.locator('text=1 deck')).toBeVisible();
  });

  test('responsive design on mobile', async ({ page, isMobile }) => {
    if (!isMobile) {
      test.skip();
    }

    // Check mobile menu
    await expect(page.locator('.mobile-menu-button')).toBeVisible();
    await page.click('.mobile-menu-button');
    
    // Check mobile navigation
    await expect(page.locator('.mobile-nav')).toBeVisible();
    await page.click('text=Decks');
    
    // Verify navigation works on mobile
    await expect(page).toHaveURL(/\/decks/);
  });

  test('card flip animation', async ({ page }) => {
    await page.goto('/');
    
    // Scroll to demo section
    await page.locator('text=Interactive Demo').scrollIntoViewIfNeeded();
    
    // Test card flip
    const card = page.locator('.flip-card').first();
    await expect(card).toBeVisible();
    
    // Click to flip
    await card.click();
    
    // Wait for animation
    await page.waitForTimeout(600); // Wait for flip animation (0.6s)
    
    // Verify card flipped (back should be visible)
    await expect(card.locator('.card-back')).toBeVisible();
    
    // Click again to flip back
    await card.click();
    await page.waitForTimeout(600);
    
    // Verify front is visible again
    await expect(card.locator('.card-front')).toBeVisible();
  });

  test('drag and drop cards to reorder', async ({ page }) => {
    await page.goto('/');
    
    // Scroll to demo section
    await page.locator('text=Drag & Drop Cards').scrollIntoViewIfNeeded();
    
    // Get draggable cards
    const firstCard = page.locator('.draggable-card').first();
    const secondCard = page.locator('.draggable-card').nth(1);
    
    // Get initial order
    const initialFirstText = await firstCard.textContent();
    const initialSecondText = await secondCard.textContent();
    
    // Perform drag and drop
    await firstCard.dragTo(secondCard);
    
    // Verify order changed
    const newFirstText = await page.locator('.draggable-card').first().textContent();
    const newSecondText = await page.locator('.draggable-card').nth(1).textContent();
    
    expect(newFirstText).toBe(initialSecondText);
    expect(newSecondText).toBe(initialFirstText);
  });

  test('study session with spaced repetition', async ({ page }) => {
    // Create a deck with cards first
    await page.goto('/decks');
    await page.click('text=+ New Deck');
    await page.fill('input[name="name"]', 'Spaced Repetition Test');
    await page.click('button[type="submit"]');
    
    // Add cards
    for (let i = 1; i <= 3; i++) {
      await page.click('text=Add Card');
      await page.fill('input[name="front"]', `Question ${i}`);
      await page.fill('textarea[name="back"]', `Answer ${i}`);
      await page.click('button:has-text("Save Card")');
    }
    
    // Start study session
    await page.click('text=Study Now');
    
    // Study cards with different difficulties
    // Card 1 - Easy
    await page.click('.flip-card-button');
    await page.click('button:has-text("Easy")');
    
    // Card 2 - Medium
    await page.click('.flip-card-button');
    await page.click('button:has-text("Medium")');
    
    // Card 3 - Hard
    await page.click('.flip-card-button');
    await page.click('button:has-text("Hard")');
    
    // Check study statistics
    await expect(page.locator('text=Session Complete')).toBeVisible();
    await expect(page.locator('text=Easy: 1')).toBeVisible();
    await expect(page.locator('text=Medium: 1')).toBeVisible();
    await expect(page.locator('text=Hard: 1')).toBeVisible();
    
    // Verify next review dates are set
    await page.click('text=View Deck');
    await expect(page.locator('text=Next review')).toBeVisible();
  });
});

test.describe('Error Handling', () => {
  test('handles network errors gracefully', async ({ page, context }) => {
    // Block API calls
    await context.route('**/api/**', route => route.abort());
    
    await page.goto('/decks');
    
    // Try to create a deck
    await page.click('text=+ New Deck');
    await page.fill('input[name="name"]', 'Test Deck');
    await page.click('button[type="submit"]');
    
    // Should show error message
    await expect(page.locator('text=Failed to create deck')).toBeVisible();
  });

  test('validates form inputs', async ({ page }) => {
    await page.goto('/decks');
    await page.click('text=+ New Deck');
    
    // Try to submit empty form
    await page.click('button[type="submit"]');
    
    // Should show validation errors
    await expect(page.locator('text=Name is required')).toBeVisible();
    
    // Test max length validation
    const longText = 'a'.repeat(256);
    await page.fill('input[name="name"]', longText);
    await page.click('button[type="submit"]');
    
    await expect(page.locator('text=Name must be less than 255 characters')).toBeVisible();
  });
});
