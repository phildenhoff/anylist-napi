import { AnyListClient } from "@anylist-napi/anylist-napi";

// Login with email and password
const client = await AnyListClient.login(
  "your-email@example.com",
  "your-password",
);

// Save tokens for later use
const tokens = client.getTokens();
console.log("Logged in as:", tokens.userId);

// Resume session from saved tokens
const resumedClient = AnyListClient.fromTokens(tokens);
