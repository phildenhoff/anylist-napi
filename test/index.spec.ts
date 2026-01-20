import { test, expect, describe, beforeAll, afterAll } from "vitest";

import { AnyListClient, type SavedTokens, type List } from "../index.js";
import { shortId, dateStamp, testListName } from "./utils.js";

const TEST_EMAIL = process.env.ANYLIST_EMAIL;
const TEST_PASSWORD = process.env.ANYLIST_PASSWORD;
const hasCredentials = Boolean(TEST_EMAIL && TEST_PASSWORD);

/** Get platform identifier for test naming */
function getPlatform(): string {
  return process.env.TARGET || `${process.platform}-${process.arch}`;
}

describe("AnyListClient API", () => {
  test("exports AnyListClient class", () => {
    expect(AnyListClient).toBeDefined();
    expect(typeof AnyListClient.login).toBe("function");
    expect(typeof AnyListClient.fromTokens).toBe("function");
  });

  test("SavedTokens interface is correctly typed", () => {
    const tokens: SavedTokens = {
      userId: "test-user-id",
      accessToken: "test-access-token",
      refreshToken: "test-refresh-token",
      isPremiumUser: false,
    };

    expect(tokens.userId).toBe("test-user-id");
    expect(tokens.isPremiumUser).toBe(false);
  });

  test("fromTokens creates client without network call", () => {
    const tokens: SavedTokens = {
      userId: "fake-user",
      accessToken: "fake-access",
      refreshToken: "fake-refresh",
      isPremiumUser: false,
    };

    const client = AnyListClient.fromTokens(tokens);
    expect(client).toBeInstanceOf(AnyListClient);

    const retrievedTokens = client.getTokens();
    expect(retrievedTokens.userId).toBe("fake-user");
    expect(retrievedTokens.accessToken).toBe("fake-access");
  });

  test("client methods exist", () => {
    const tokens: SavedTokens = {
      userId: "fake-user",
      accessToken: "fake-access",
      refreshToken: "fake-refresh",
      isPremiumUser: false,
    };

    const client = AnyListClient.fromTokens(tokens);

    expect(typeof client.getLists).toBe("function");
    expect(typeof client.createList).toBe("function");
    expect(typeof client.deleteList).toBe("function");
    expect(typeof client.addItem).toBe("function");
    expect(typeof client.addItemWithDetails).toBe("function");
    expect(typeof client.deleteItem).toBe("function");
    expect(typeof client.crossOffItem).toBe("function");
    expect(typeof client.uncheckItem).toBe("function");
    expect(typeof client.getRecipes).toBe("function");
    expect(typeof client.getRecipeById).toBe("function");
    expect(typeof client.addRecipeToList).toBe("function");
  });
});

describe.runIf(hasCredentials)("AnyListClient Integration", () => {
  let client: AnyListClient;
  let testList: List;
  const listName = testListName("node", getPlatform());
  const addedItemIds: string[] = [];

  beforeAll(async () => {
    client = await AnyListClient.login(TEST_EMAIL!, TEST_PASSWORD!);
  });

  afterAll(async () => {
    // Clean up: delete the test list (and all its items)
    if (client && testList) {
      try {
        await client.deleteList(testList.id);
        console.log(`Cleaned up test list: ${listName}`);
      } catch {
        // Fallback: try to delete items if list deletion fails
        for (const itemId of addedItemIds) {
          try {
            await client.deleteItem(testList.id, itemId);
          } catch {
            // Ignore cleanup errors
          }
        }
      }
    }
  });

  test("login returns authenticated client", () => {
    expect(client).toBeInstanceOf(AnyListClient);
  });

  test("getTokens returns valid tokens after login", () => {
    const tokens = client.getTokens();

    expect(tokens.userId).toBeTruthy();
    expect(tokens.accessToken).toBeTruthy();
    expect(tokens.refreshToken).toBeTruthy();
    expect(typeof tokens.isPremiumUser).toBe("boolean");
  });

  test("fromTokens can restore session", async () => {
    const tokens = client.getTokens();
    const restoredClient = AnyListClient.fromTokens(tokens);

    // Verify the restored client works
    const lists = await restoredClient.getLists();
    expect(Array.isArray(lists)).toBe(true);
  });

  test("getLists returns array of lists", async () => {
    const lists = await client.getLists();

    expect(Array.isArray(lists)).toBe(true);
    // Every AnyList account has at least one default list
    if (lists.length > 0) {
      expect(lists[0]).toHaveProperty("id");
      expect(lists[0]).toHaveProperty("name");
      expect(lists[0]).toHaveProperty("items");
    }
  });

  test("createList creates a new list", async () => {
    console.log(`Creating test list: ${listName}`);
    testList = await client.createList(listName);

    expect(testList).toHaveProperty("id");
    expect(testList.name).toBe(listName);
    expect(Array.isArray(testList.items)).toBe(true);
  });

  test("addItem adds item to list", async () => {
    await client.addItem(testList.id, "Test Item 1");

    // Fetch the list to verify
    const lists = await client.getLists();
    const updatedList = lists.find((l) => l.id === testList.id);

    expect(updatedList).toBeDefined();
    const addedItem = updatedList!.items.find((i) => i.name === "Test Item 1");
    expect(addedItem).toBeDefined();
    expect(addedItem!.checked).toBe(false);

    addedItemIds.push(addedItem!.id);
  });

  test("addItemWithDetails adds item with quantity and note", async () => {
    await client.addItemWithDetails(
      testList.id,
      "Test Item 2",
      "2 lbs",
      "Get the organic one",
      "Produce",
    );

    const lists = await client.getLists();
    const updatedList = lists.find((l) => l.id === testList.id);
    const addedItem = updatedList!.items.find((i) => i.name === "Test Item 2");

    expect(addedItem).toBeDefined();
    expect(addedItem!.quantity).toBe("2 lbs");

    expect(addedItem!.note).toBe("Get the organic one");

    addedItemIds.push(addedItem!.id);
  });

  test("crossOffItem checks the item", async () => {
    const itemId = addedItemIds[0];
    await client.crossOffItem(testList.id, itemId);

    const lists = await client.getLists();
    const updatedList = lists.find((l) => l.id === testList.id);
    const item = updatedList!.items.find((i) => i.id === itemId);

    expect(item!.checked).toBe(true);
  });

  test("uncheckItem unchecks the item", async () => {
    const itemId = addedItemIds[0];
    await client.uncheckItem(testList.id, itemId);

    const lists = await client.getLists();
    const updatedList = lists.find((l) => l.id === testList.id);
    const item = updatedList!.items.find((i) => i.id === itemId);

    expect(item!.checked).toBe(false);
  });

  test("deleteItem removes item from list", async () => {
    const itemId = addedItemIds.pop()!;
    await client.deleteItem(testList.id, itemId);

    const lists = await client.getLists();
    const updatedList = lists.find((l) => l.id === testList.id);
    const deletedItem = updatedList!.items.find((i) => i.id === itemId);

    expect(deletedItem).toBeUndefined();
  });

  test("getRecipes returns array", async () => {
    const recipes = await client.getRecipes();

    expect(Array.isArray(recipes)).toBe(true);
    // Recipes may be empty for a new account
    if (recipes.length > 0) {
      expect(recipes[0]).toHaveProperty("id");
      expect(recipes[0]).toHaveProperty("name");
      expect(recipes[0]).toHaveProperty("ingredients");
    }
  });

  test("deleteList removes a list", async () => {
    // Create a temporary list to delete
    const tempListName = `CI delete-test ${shortId()} ${dateStamp()}`;
    const tempList = await client.createList(tempListName);
    expect(tempList.id).toBeTruthy();

    // Verify it exists
    let lists = await client.getLists();
    expect(lists.some((l) => l.id === tempList.id)).toBe(true);

    // Delete it
    await client.deleteList(tempList.id);

    // Verify it's gone
    lists = await client.getLists();
    expect(lists.some((l) => l.id === tempList.id)).toBe(false);
  });
});
