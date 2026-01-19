# anylist-napi

Bun/Node/Deno bindings for anylist_rs.

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

Bun is the package manager of choice. Prefer typescript-first tooling.
