import { AnyListClient } from "../index.js";

async function main() {
  const email = process.env.ANYLIST_EMAIL;
  const password = process.env.ANYLIST_PASSWORD;

  if (!email || !password) {
    throw new Error("Email and password must be provided");
  }

  // Authenticate
  const client = await AnyListClient.login(email, password);

  console.log("Creating a new list...");
  const list = await client.createList("Weekend Shopping");

  console.log("Adding some items...");
  await client.addItem(list.id, "Bread");
  await client.addItem(list.id, "Eggs");
  await client.addItemWithDetails(list.id, "Chicken", "2 lbs", null, "Meat");

  console.log("Listing all items from all lists...");
  const allLists = await client.getLists();
  for (const l of allLists) {
    console.log(`\n${l.name}:`);
    for (const item of l.items) {
      const status = item.checked ? "✓" : " ";
      console.log(`  [${status}] ${item.name} ${item.quantity || ""}`);
    }
  }

  // Save tokens for next time
  const tokens = client.getTokens();
  // Store tokens securely...
}

main().catch(console.error);
