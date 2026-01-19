# anylist-napi

TypeScript/JavaScript bindings for [anylist_rs](https://github.com/phildenhoff/anylist_rs) using [NAPI](https://napi.rs).

## Installation

```bash
pnpm add anylist-napi
# or
bun add anylist-napi
```

## Runtime Support

This package targets multiple JavaScript runtimes:

- **Node.js** v20+ (primary target, fully tested)
- **Deno** v2+ (officially supported, CI tested)
- **Bun** (NAPI support is experimental, not tested)

## Usage

### Authentication

```typescript
import { AnyListClient } from "anylist-napi";

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
```

### Working with Lists

```typescript
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
```

### Working with Recipes

```typescript
// Get all recipes
const recipes = await client.getRecipes();
console.log(`Found ${recipes.length} recipes`);

// Get a specific recipe
const recipe = await client.getRecipeById("recipe-id-here");
console.log(`Recipe: ${recipe.name}`);
console.log(`Ingredients: ${recipe.ingredients.length}`);

// Add recipe ingredients to a list
await client.addRecipeToList(
  recipe.id,
  groceryList.id,
  1.5, // scale factor (optional)
);
```

### Complete Example

```typescript
import { AnyListClient } from "anylist-napi";

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
```

## Development

### Prerequisites

- **Rust** (latest stable) - for building native bindings
- **Node.js 20+** OR **Deno 2+** - runtime for testing
- **Bun** (optional, recommended) - for development tooling

### Common Commands

| Task              | Command                                            | Description                                                  |
| ----------------- | -------------------------------------------------- | ------------------------------------------------------------ |
| **Setup**         | `bun install`                                      | Install dependencies (or use `npm install` / `yarn install`) |
| **Build**         | `bun run build`                                    | Release build                                                |
|                   | `bun run build:debug`                              | Debug build                                                  |
| **Test**          | `bun run test`                                     | Run tests (Node.js runtime)                                  |
|                   | `bun run test:watch`                               | Watch mode                                                   |
|                   | `deno run --allow-ffi --allow-read npm:vitest run` | Test on Deno                                                 |
| **Benchmark**     | `bun run bench`                                    | Run performance benchmarks                                   |
| **Lint & Format** | `bun run lint`                                     | Lint TypeScript with oxlint                                  |
|                   | `bun run format`                                   | Format TypeScript and Rust code                              |

## Platform Support

Pre-built native binaries are available for:

- macOS (Intel & Apple Silicon)
- Windows x64
- Linux GNU (x86_64)
- Linux musl (x86_64)

## Contributing

Contributions are welcome! Please ensure:

1. Tests pass on Node.js (`bun run test`)
2. Code is formatted (`bun run format`)
3. Linting passes (`bun run lint`)
4. Benchmarks still work (`bun run bench`)

## License

MIT
