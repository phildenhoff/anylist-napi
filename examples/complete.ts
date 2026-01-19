import { AnyListClient } from "../index.js";

async function main() {
  // Authenticate
  const client = await AnyListClient.login(
    process.env.ANYLIST_EMAIL!,
    process.env.ANYLIST_PASSWORD!,
  );

  // Create a new grocery list
  const list = await client.createList("Weekend Shopping");

  // Add some items
  await client.addItem(list.id, "Bread");
  await client.addItem(list.id, "Eggs");
  await client.addItemWithDetails(list.id, "Chicken", "2 lbs", null, "Meat");

  // Get and display all lists
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
