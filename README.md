# anylist-napi

TypeScript/JavaScript bindings for [anylist_rs](https://github.com/phildenhoff/anylist_rs) using [NAPI](https://napi.rs).

## Installation

```bash
pnpm add @anylist-napi/anylist-napi
# or
bun add @anylist-napi/anylist-napi
```

## Runtime Support

This package targets multiple JavaScript runtimes:

- **Node.js** v20+ (primary target, fully tested)
- **Deno** v2+ (officially supported, CI tested)
- **Bun** (NAPI support is experimental, not tested)

## Usage

See [examples/](./examples/) for more.

```typescript
import { AnyListClient } from "@anylist-napi/anylist-napi";

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

### Integration tests

You can run the integration tests with a real Anylist account.
Each run will create a new list, named with the current time.

```sh
ANYLIST_EMAIL=<your email> ANYLIST_PASSWORD=<your password> bun run test
```

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
