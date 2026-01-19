// Import the native bindings
// @ts-expect-error - CommonJS/ESM interop handled by Bun
import { AnyListClient } from "./index.js";

async function main() {
  const email = process.env.ANYLIST_EMAIL;
  const password = process.env.ANYLIST_PASSWORD;

  if (!email || !password) {
    console.log("Set ANYLIST_EMAIL and ANYLIST_PASSWORD environment variables");
    console.log("\nExample usage:");
    console.log("  ANYLIST_EMAIL=you@example.com ANYLIST_PASSWORD=secret bun example.ts");
    process.exit(1);
  }

  console.log("Logging in...");
  const client = await AnyListClient.login(email, password);

  console.log(`Logged in as user: ${client.userId()}`);
  console.log(`Premium user: ${client.isPremiumUser()}`);

  // Get and display lists
  const lists = await client.getLists();
  console.log(`\nFound ${lists.length} lists:`);
  for (const list of lists) {
    console.log(`  - ${list.name} (${list.items.length} items)`);
  }

  // Export tokens for later use
  const tokens = client.exportTokens();
  console.log(`\nTokens exported for user: ${tokens.userId}`);
}

main().catch(console.error);
