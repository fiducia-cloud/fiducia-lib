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

## Repository-local Git worktrees

- Create or use a Git worktree only when the human operator explicitly authorizes it for the current task. Concurrency or a dirty checkout is not permission by itself.
- Put every authorized worktree at `<repository-root>/tmp/worktrees/<name>`; from the repository root, use `./tmp/worktrees/<name>`. Never place worktrees beside repositories or organization directories.
- Keep `tmp`, `temp`, `tmp/worktrees`, and `temp/worktrees` ignored in the repository-root `.gitignore`. Do not commit files from those directories.
- Relocate or remove a worktree only when the operator explicitly requests it. Before removal, preserve and publish intended changes, verify its commit is represented on the target branch, and confirm there are no tracked, untracked, ignored-sensitive, or in-use files that must survive. Remove it with `git worktree remove <path>` without `--force`; never delete a worktree directory with `rm`.

<!-- BEGIN ores-agents-pointer: managed by ORESoftware/my-ai; edit there, not here -->

## Canonical agent instructions

Before doing anything else in this repository, also read:

    .ores/agents/AGENTS.md

That path is a symlink to `~/codes/oresoftware/my-ai/AGENTS.md`, whose canonical copy is
<https://github.com/ORESoftware/my-ai/blob/main/AGENTS.md>.

It exists at a fixed path *inside* the repository because some agents cannot walk up past
the repository root, so machine-wide instructions one or more directories above are
invisible to them. This pointer plus that path make the same file reachable from a working
directory anywhere in the tree.

The symlink is deliberately **not committed**: it names an absolute path that is only valid
on a machine with `~/codes/oresoftware/my-ai` checked out, so committing it would produce a
broken link for everyone else and for CI. `.ores/` is git-ignored for that reason. If
`.ores/agents/AGENTS.md` is missing on your machine, create it with:

    mkdir -p .ores/agents
    ln -sfn "$HOME/codes/oresoftware/my-ai/AGENTS.md" .ores/agents/AGENTS.md

or run `~/codes/oresoftware/my-ai/scripts/link-repo-agents.sh` once to do it for every git
repository under `~/codes`, and `--check` to verify them.

A missing `.ores/agents/AGENTS.md` is a setup gap on the reader's machine, never a reason to
skip the canonical instructions: fetch them from the URL above instead.

<!-- END ores-agents-pointer -->
