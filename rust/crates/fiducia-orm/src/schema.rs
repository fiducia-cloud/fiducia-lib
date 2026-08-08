//! Shared-schema naming. Schemas are strictly namespaced per org.

/// The Fiducia org's shared Postgres schema.
pub const ORG_SCHEMA: &str = "fiducia";

/// Return the schema-qualified name for `table`, e.g. `fiducia.users`.
///
/// # Panics
///
/// Panics if `table` is empty, contains whitespace, or contains quote
/// characters — such input is a programming error, never data.
pub fn qualified(table: &str) -> String {
    assert!(!table.is_empty(), "qualified: table name must not be empty");
    assert!(
        !table.chars().any(char::is_whitespace),
        "qualified: table name must not contain whitespace: {table:?}"
    );
    assert!(
        !table.contains('"') && !table.contains('\'') && !table.contains('`'),
        "qualified: table name must not contain quote characters: {table:?}"
    );
    format!("{ORG_SCHEMA}.{table}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qualifies_with_org_schema() {
        assert_eq!(qualified("users"), "fiducia.users");
    }

    #[test]
    #[should_panic(expected = "must not be empty")]
    fn rejects_empty() {
        qualified("");
    }

    #[test]
    #[should_panic(expected = "whitespace")]
    fn rejects_whitespace() {
        qualified("users; drop table");
    }

    #[test]
    #[should_panic(expected = "quote")]
    fn rejects_quotes() {
        qualified("users\"");
    }
}
