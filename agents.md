# Agent guidelines — fiducia-lib

Shared Fiducia library code; home of `fiducia-orm`, the SeaORM data-access
boundary crate for the shared Postgres schema.

## Instruction discovery

- Resolve the real path of `$PWD`, walk its ancestor directories through the filesystem root, and load every readable lowercase `agents.md` in root-to-leaf order.
- Do not search sibling directories. Deduplicate canonical paths, detect symlink cycles, and report unreadable instruction files.
- `AGENTS.md`, `.claude/CLAUDE.md`, `.gemini/GEMINI.md`, and `.openai/AGENTS.md` are compatibility pointers only; lowercase `agents.md` files are canonical.

## Linear mapping

- GitHub organization: `github.com/fiducia-cloud`.
- Linear project: `github.com/fiducia-cloud` in the Denman workspace.
- Locate or create the matching Linear issue before substantial work. Record PR links, test evidence, blockers, and remaining work there.

## Library invariants

- `fiducia-orm` exports named query functions, never a raw ORM session. Web tiers call only the `read` submodule; shared-schema writes belong to the API tier.
- `sea-orm` is the only direct data-layer dependency; never add a direct `sqlx` dependency. No migration code in this repository.
- Root `.zpkg.toml` only — do not create nested manifests.

## Command and Git safety

- No rebase, force-push, hard reset, clean, or history rewriting. Prefer normal commits, `git merge`, and reversible operations.
- Start from current `main`, use a focused feature branch and PR; never commit to `main` directly.
- Resolve conflicts semantically — never blindly ours/theirs — and grep for `<<<<<<<`, `=======`, `>>>>>>>` before committing.

## Validation

Run `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` in `rust/` before pushing.
