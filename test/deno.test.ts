// @ts-nocheck - This file runs in Deno only, not type-checked with Node types
/**
 * Deno-specific test file that validates bindings load correctly.
 * Uses Deno's built-in test runner instead of vitest (which has esbuild issues in Deno).
 */

import { AnyListClient, type SavedTokens } from "../index.js";
import { shortId, dateStamp, testListName } from "./utils.ts";

/** Get platform identifier for test naming */
function getPlatform(): string {
  return Deno.env.get("TARGET") || `${Deno.build.os}-${Deno.build.arch}`;
}

Deno.test("AnyListClient class is exported", () => {
  if (typeof AnyListClient !== "function") {
    throw new Error("AnyListClient should be a function/class");
  }
});

Deno.test("AnyListClient has static login method", () => {
  if (typeof AnyListClient.login !== "function") {
    throw new Error("AnyListClient.login should be a function");
  }
});

Deno.test("AnyListClient has static fromTokens method", () => {
  if (typeof AnyListClient.fromTokens !== "function") {
    throw new Error("AnyListClient.fromTokens should be a function");
  }
});

Deno.test("fromTokens creates client from tokens", () => {
  const tokens: SavedTokens = {
    userId: "test-user",
    accessToken: "test-access",
    refreshToken: "test-refresh",
    isPremiumUser: false,
  };

  const client = AnyListClient.fromTokens(tokens);

  if (!(client instanceof AnyListClient)) {
    throw new Error("fromTokens should return AnyListClient instance");
  }

  const retrieved = client.getTokens();
  if (retrieved.userId !== "test-user") {
    throw new Error("Token userId mismatch");
  }
});

Deno.test("client has all expected methods", () => {
  const tokens: SavedTokens = {
    userId: "test",
    accessToken: "test",
    refreshToken: "test",
    isPremiumUser: false,
  };

  const client = AnyListClient.fromTokens(tokens);

  const methods = [
    "getLists",
    "createList",
    "deleteList",
    "addItem",
    "addItemWithDetails",
    "deleteItem",
    "crossOffItem",
    "uncheckItem",
    "getRecipes",
    "getRecipeById",
    "addRecipeToList",
    "getTokens",
  ];

  for (const method of methods) {
    if (typeof (client as Record<string, unknown>)[method] !== "function") {
      throw new Error(`client.${method} should be a function`);
    }
  }
});

const email = Deno.env.get("ANYLIST_EMAIL");
const password = Deno.env.get("ANYLIST_PASSWORD");

if (email && password) {
  Deno.test("integration: full API test", async () => {
    // Login
    const client = await AnyListClient.login(email, password);
    if (!(client instanceof AnyListClient)) {
      throw new Error("login should return AnyListClient instance");
    }

    // Verify tokens
    const tokens = client.getTokens();
    if (!tokens.userId || !tokens.accessToken) {
      throw new Error("getTokens should return valid tokens");
    }

    // Test fromTokens session restoration
    const restoredClient = AnyListClient.fromTokens(tokens);
    const restoredLists = await restoredClient.getLists();
    if (!Array.isArray(restoredLists)) {
      throw new Error("Restored client should work");
    }

    // Get lists
    const lists = await client.getLists();
    if (!Array.isArray(lists)) {
      throw new Error("getLists should return an array");
    }

    // Create a test list with identifiable name
    const listName = testListName("deno", getPlatform());
    console.log(`Creating test list: ${listName}`);
    const testList = await client.createList(listName);
    if (testList.name !== listName) {
      throw new Error("createList should return list with correct name");
    }

    // Add items
    await client.addItem(testList.id, "Deno Test Item 1");
    await client.addItemWithDetails(
      testList.id,
      "Deno Test Item 2",
      "2 lbs",
      "Test note",
      "Produce",
    );

    // Verify items were added
    const updatedLists = await client.getLists();
    const updatedList = updatedLists.find((l) => l.id === testList.id);
    if (!updatedList) {
      throw new Error("Test list should exist");
    }

    const item1 = updatedList.items.find((i) => i.name === "Deno Test Item 1");
    const item2 = updatedList.items.find((i) => i.name === "Deno Test Item 2");
    if (!item1 || !item2) {
      throw new Error("Added items should exist");
    }
    if (item2.quantity !== "2 lbs") {
      throw new Error("Item quantity should match");
    }

    // Cross off and uncheck
    await client.crossOffItem(testList.id, item1.id);
    let checkLists = await client.getLists();
    let checkList = checkLists.find((l) => l.id === testList.id);
    let checkedItem = checkList?.items.find((i) => i.id === item1.id);
    if (!checkedItem?.checked) {
      throw new Error("Item should be checked after crossOffItem");
    }

    await client.uncheckItem(testList.id, item1.id);
    checkLists = await client.getLists();
    checkList = checkLists.find((l) => l.id === testList.id);
    checkedItem = checkList?.items.find((i) => i.id === item1.id);
    if (checkedItem?.checked) {
      throw new Error("Item should be unchecked after uncheckItem");
    }

    // Delete items (cleanup)
    await client.deleteItem(testList.id, item1.id);
    await client.deleteItem(testList.id, item2.id);

    // Verify deletion
    const finalLists = await client.getLists();
    const finalList = finalLists.find((l) => l.id === testList.id);
    if (finalList?.items.some((i) => i.name.startsWith("Deno Test Item"))) {
      throw new Error("Deleted items should not exist");
    }

    // Test recipes (just verify it returns an array)
    const recipes = await client.getRecipes();
    if (!Array.isArray(recipes)) {
      throw new Error("getRecipes should return an array");
    }

    // Test deleteList explicitly
    const tempListName = `CI delete-test ${shortId()} ${dateStamp()}`;
    const tempList = await client.createList(tempListName);
    if (!tempList.id) {
      throw new Error("Temp list should have an id");
    }

    // Verify temp list exists
    let allLists = await client.getLists();
    if (!allLists.some((l) => l.id === tempList.id)) {
      throw new Error("Temp list should exist after creation");
    }

    // Delete temp list
    await client.deleteList(tempList.id);

    // Verify temp list is gone
    allLists = await client.getLists();
    if (allLists.some((l) => l.id === tempList.id)) {
      throw new Error("Temp list should not exist after deletion");
    }
    console.log(`deleteList test passed: ${tempListName}`);

    // Clean up: delete the main test list
    await client.deleteList(testList.id);
    console.log(`Cleaned up test list: ${listName}`);

    console.log("All Deno integration tests passed!");
  });
} else {
  console.log(
    "Skipping integration tests: ANYLIST_EMAIL and ANYLIST_PASSWORD not set",
  );
}
