//! # fiducia-orm
//!
//! The SeaORM data-access boundary crate for the Fiducia shared Postgres
//! schema.
//!
//! ## The write/read split
//!
//! The Fiducia platform enforces a strict boundary around the shared schema
//! (schema `fiducia`):
//!
//! - The Rust **API tier** performs all shared-schema **writes**. It connects
//!   with [`DbRole::ReadWrite`].
//! - **Web/BFF tiers** may **read** the shared schema but never write it.
//!   They connect with [`DbRole::ReadOnly`], which enforces
//!   `default_transaction_read_only=on` at the connection level, and they
//!   should call [`assert_read_only`] at startup to verify the setting took
//!   effect. (A SELECT-only DB role is the second layer of defense; that
//!   grant work is operational, not code.)
//! - Web tiers call only the named query functions in [`queries::read`];
//!   this crate never hands out a raw ORM session as its contract.
//! - Migrations belong to the declarative-migrations/API tier — not here.
//!
//! See `SERVICE_AND_DATA_ARCHITECTURE.md` in the `fiducia-cloud/.github`
//! repository for the full architecture rules:
//! <https://github.com/fiducia-cloud/.github/blob/main/SERVICE_AND_DATA_ARCHITECTURE.md>

pub mod connect;
pub mod queries;
pub mod schema;

pub use connect::{apply_role, assert_read_only, connect, DbRole};
pub use schema::{qualified, ORG_SCHEMA};
