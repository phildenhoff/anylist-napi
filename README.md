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

```typescript
<content to come>
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
