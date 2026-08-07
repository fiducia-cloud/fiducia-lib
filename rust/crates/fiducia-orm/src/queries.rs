//! Named query functions for the Fiducia shared schema.
//!
//! ## Contract
//!
//! This library exports **named query functions**, never a raw ORM session.
//! Callers pass a [`sea_orm::DatabaseConnection`] obtained from
//! [`crate::connect`] and invoke a function that encodes exactly one query.
//!
//! - Web/BFF tiers may **only** call functions in the [`read`] submodule, on
//!   a [`crate::DbRole::ReadOnly`] connection.
//! - Functions in [`write`] are reserved for the Fiducia API tier and its
//!   [`crate::DbRole::ReadWrite`] connection.

pub use read::healthcheck;

/// Read-only queries. The only submodule web tiers may call.
pub mod read {
    use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, DbErr, Statement};

    /// Verify the database is reachable (`SELECT 1`).
    pub async fn healthcheck(conn: &DatabaseConnection) -> Result<(), DbErr> {
        conn.query_one(Statement::from_string(DbBackend::Postgres, "SELECT 1"))
            .await?;
        Ok(())
    }
}

/// Write queries. Reserved for the Fiducia API tier — the sole owner of
/// shared-schema writes. Intentionally empty until the API tier lands its
/// first shared-schema mutation.
pub mod write {
    // pub async fn record_event(conn: &DatabaseConnection, ...) -> Result<(), DbErr>
}
