# anylist-ts

Bun/Node bindings for anylist_rs.

## Task-tracking workflow

You must use `beads` (cli comman: `bd`) to track your work.
Run `bd prime` to understand how to use it deeply.

Short version:

- find available work: `bd ready`
- view issue details: `bd show <id>`
- claim work: `bd update <id> --status in_progress`
- complete work: `bd close <id>`
- persist task updates to git: `bd sync`

## Preferences

Prefer using Bun instead of node, ts-node, jest/vitest, webpack/esbuild, npm/yarn/pnpm, npx.

For more information, read the Bun API docs in `node_modules/bun-types/docs/**.mdx`.
