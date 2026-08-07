# fiducia-lib

Shared Fiducia library code. This repository is the home of `fiducia-orm`, the
SeaORM data-access boundary crate used by Fiducia services to talk to the
shared Postgres schema.

## Purpose

The Fiducia platform enforces a strict data-access boundary:

- The Rust API tier owns **all writes** to the shared schema.
- Web/BFF tiers may **read** the shared schema, but only through a
  `ReadOnly` connection (`default_transaction_read_only=on`), and only via
  named query functions exported by `fiducia-orm` — never a raw ORM session.
- Migrations belong to the declarative-migrations/API tier, not to this
  library or to web tiers.

See [SERVICE_AND_DATA_ARCHITECTURE.md](https://github.com/fiducia-cloud/.github/blob/main/SERVICE_AND_DATA_ARCHITECTURE.md)
for the full architecture rules.

## Layout

- `rust/` — Cargo workspace
  - `rust/crates/fiducia-orm` — the SeaORM boundary crate

## License

MIT — see [LICENSE](LICENSE).
