// Get all lists
const lists = await client.getLists();
for (const list of lists) {
  console.log(`${list.name}: ${list.items.length} items`);
}

// Create a new list
const groceryList = await client.createList("Weekly Groceries");

// Add items
await client.addItem(groceryList.id, "Milk");
await client.addItemWithDetails(
  groceryList.id,
  "Apples",
  "2 lbs", // quantity
  "Organic", // note
  "Produce", // category
);

// Check off items
const itemToCheck = groceryList.items[0];
await client.crossOffItem(groceryList.id, itemToCheck.id);

// Uncheck items
await client.uncheckItem(groceryList.id, itemToCheck.id);

// Delete items
await client.deleteItem(groceryList.id, itemToCheck.id);
