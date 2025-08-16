use crate::db::models::Image;
use crate::errors::DatalintResult;
use duckdb::{params, Connection};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub struct ImageQueries;

impl ImageQueries {
    const INSERT: &'static str = r#"
        INSERT INTO images (filename, relative_path, split, width, height, channels, format, file_size, file_hash)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
    "#;

    const SELECT_BY_HASH: &'static str = r#"
        SELECT id, filename, relative_path, split, width, height, channels, format, file_size, file_hash
        FROM images WHERE file_hash = ?
    "#;

    const COUNT_BY_SPLIT: &'static str = r#"
        SELECT split, COUNT(*) as count
        FROM images
        GROUP BY split
    "#;

    /// Insert a new image
    pub fn insert(conn: &Connection, image: &Image) -> DatalintResult<i64> {
        conn.query_row(
            Self::INSERT,
            params![
                image.filename,
                image.relative_path,
                image.split,
                image.width,
                image.height,
                image.channels,
                image.format,
                image.file_size,
                image.file_hash,
            ],
            |row| row.get(0),
        )
        .map_err(Into::into)
    }

    /// Compute SHA256 hash for a file
    pub fn compute_file_hash(path: &Path) -> DatalintResult<String> {
        let data = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Find image by hash
    pub fn find_by_hash(conn: &Connection, hash: &str) -> DatalintResult<Option<Image>> {
        let mut stmt = conn.prepare(Self::SELECT_BY_HASH)?;

        let result = stmt.query_row(params![hash], |row| {
            Ok(Image {
                id: Some(row.get(0)?),
                filename: row.get(1)?,
                relative_path: row.get(2)?,
                split: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                channels: row.get(6)?,
                format: row.get(7)?,
                file_size: row.get(8)?,
                file_hash: row.get(9)?,
            })
        });

        match result {
            Ok(image) => Ok(Some(image)),
            Err(duckdb::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Count images by split
    pub fn count_by_split(conn: &Connection) -> DatalintResult<Vec<(String, i32)>> {
        let mut stmt = conn.prepare(Self::COUNT_BY_SPLIT)?;

        let results = stmt.query_map(params![], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        })?;

        let mut vec = Vec::new();
        for result in results {
            vec.push(result?);
        }
        Ok(vec)
    }
}
