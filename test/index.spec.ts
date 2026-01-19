import { test, expect, describe } from "vitest";

import { plus100, AnyListClient, type SavedTokens } from "../index.js";

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

  // Note: Actual API tests would require valid credentials
  // These are type-level tests to verify the bindings are working
  test.skip("login example (requires credentials)", async () => {
    // Example usage (skip in CI):
    // const client = await AnyListClient.login("email@example.com", "password");
    // const lists = await client.getLists();
    // console.log(lists);
  });
});
