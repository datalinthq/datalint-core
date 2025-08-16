use crate::db::models::Label;
use crate::errors::DatalintResult;
use duckdb::{params, Connection};

pub struct LabelQueries;

impl LabelQueries {
    const INSERT: &'static str = r#"
        INSERT INTO labels (name, color)
        VALUES (?, ?)
        RETURNING id
    "#;

    const SELECT_ALL: &'static str = r#"
        SELECT id, name, color FROM labels ORDER BY id
    "#;

    const SELECT_BY_NAME: &'static str = r#"
        SELECT id, name, color FROM labels WHERE name = ?
    "#;

    /// Insert a new label
    pub fn insert(conn: &Connection, label: &Label) -> DatalintResult<i64> {
        conn.query_row(Self::INSERT, params![label.name, label.color], |row| {
            row.get(0)
        })
        .map_err(Into::into)
    }

    /// Get all labels
    pub fn get_all(conn: &Connection) -> DatalintResult<Vec<Label>> {
        let mut stmt = conn.prepare(Self::SELECT_ALL)?;

        let results = stmt.query_map(params![], |row| {
            Ok(Label {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                color: row.get(2)?,
            })
        })?;

        let mut vec = Vec::new();
        for result in results {
            vec.push(result?);
        }
        Ok(vec)
    }

    /// Find label by name
    pub fn find_by_name(conn: &Connection, name: &str) -> DatalintResult<Option<Label>> {
        let mut stmt = conn.prepare(Self::SELECT_BY_NAME)?;

        let result = stmt.query_row(params![name], |row| {
            Ok(Label {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                color: row.get(2)?,
            })
        });

        match result {
            Ok(label) => Ok(Some(label)),
            Err(duckdb::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get or create label by name
    pub fn get_or_create(
        conn: &Connection,
        name: &str,
        color: Option<String>,
    ) -> DatalintResult<i32> {
        if let Some(existing) = Self::find_by_name(conn, name)? {
            Ok(existing.id.unwrap())
        } else {
            let label = Label {
                id: None,
                name: name.to_string(),
                color,
            };
            Self::insert(conn, &label).map(|id| id as i32)
        }
    }
}
