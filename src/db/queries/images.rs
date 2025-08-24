use crate::db::models::Image;
use crate::errors::DatalintResult;
use duckdb::{params, Connection};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub struct ImageQueries;

impl ImageQueries {
    const INSERT: &'static str = r#"
        INSERT INTO images (name, filename, extension, relative_path, split, width, height, channels, file_size, file_hash, is_corrupted)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
    "#;

    const SELECT_BY_HASH: &'static str = r#"
        SELECT id, name, filename, extension, relative_path, split, width, height, channels, file_size, file_hash, is_corrupted
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
                image.name,
                image.filename,
                image.extension,
                image.relative_path,
                image.split,
                image.width,
                image.height,
                image.channels,
                image.file_size,
                image.file_hash,
                image.is_corrupted
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
                name: row.get(1)?,
                filename: row.get(2)?,
                extension: row.get(3)?,
                relative_path: row.get(4)?,
                split: row.get(5)?,
                width: row.get(6)?,
                height: row.get(7)?,
                channels: row.get(8)?,
                file_size: row.get(9)?,
                file_hash: row.get(10)?,
                is_corrupted: row.get::<_, i32>(11)? != 0, // Convert i32 to bool
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
