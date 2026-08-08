//! Role-aware connection helpers for the Fiducia shared schema.

use std::time::Duration;

use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement,
};

use crate::schema::ORG_SCHEMA;

/// The role a service plays toward the shared schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbRole {
    /// API tier: may read and write the shared schema.
    ReadWrite,
    /// Web/BFF tier: may only read the shared schema. Connections carry the
    /// Postgres startup option `default_transaction_read_only=on`.
    ReadOnly,
}

/// Percent-encoded Postgres startup option enforcing read-only transactions:
/// `options=-c default_transaction_read_only=on`.
const READ_ONLY_OPTION: &str = "options=-c%20default_transaction_read_only%3Don";

/// Apply `role` to a Postgres connection URL.
///
/// For [`DbRole::ReadOnly`] the startup option
/// `options=-c default_transaction_read_only=on` (percent-encoded) is
/// appended, respecting any existing query string. [`DbRole::ReadWrite`]
/// returns the URL unchanged.
///
/// Pure function — unit-testable without a database.
pub fn apply_role(database_url: &str, role: DbRole) -> String {
    match role {
        DbRole::ReadWrite => database_url.to_owned(),
        DbRole::ReadOnly => {
            let separator = if database_url.contains('?') { '&' } else { '?' };
            format!("{database_url}{separator}{READ_ONLY_OPTION}")
        }
    }
}

/// Connect to Postgres with the given role.
///
/// Sets the schema search path to [`ORG_SCHEMA`], applies sane pool defaults,
/// and connects using the role-applied URL from [`apply_role`].
pub async fn connect(database_url: &str, role: DbRole) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(apply_role(database_url, role));
    options
        .max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(300))
        .sqlx_logging(false)
        .set_schema_search_path(ORG_SCHEMA);
    Database::connect(options).await
}

/// Verify the connection really is read-only.
///
/// Checks `current_setting('default_transaction_read_only') = 'on'` and
/// returns an error otherwise. Web servers must call this at startup on their
/// shared-schema connection so a misconfigured URL fails fast instead of
/// silently allowing writes.
pub async fn assert_read_only(conn: &DatabaseConnection) -> Result<(), DbErr> {
    let row = conn
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            "SELECT current_setting('default_transaction_read_only') AS setting",
        ))
        .await?
        .ok_or_else(|| {
            DbErr::Custom("assert_read_only: current_setting returned no row".to_owned())
        })?;
    let setting: String = row.try_get("", "setting")?;
    if setting == "on" {
        Ok(())
    } else {
        Err(DbErr::Custom(format!(
            "assert_read_only: expected default_transaction_read_only=on, got {setting:?}; \
             this connection must be created with DbRole::ReadOnly"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_only_appends_option_to_plain_url() {
        let url = apply_role("postgres://app@db.internal:5432/fiducia", DbRole::ReadOnly);
        assert_eq!(
            url,
            "postgres://app@db.internal:5432/fiducia?options=-c%20default_transaction_read_only%3Don"
        );
    }

    #[test]
    fn read_only_respects_existing_query_string() {
        let url = apply_role(
            "postgres://app@db.internal:5432/fiducia?sslmode=require",
            DbRole::ReadOnly,
        );
        assert_eq!(
            url,
            "postgres://app@db.internal:5432/fiducia?sslmode=require&options=-c%20default_transaction_read_only%3Don"
        );
    }

    #[test]
    fn read_write_is_a_passthrough() {
        let original = "postgres://app@db.internal:5432/fiducia?sslmode=require";
        assert_eq!(apply_role(original, DbRole::ReadWrite), original);
    }
}
