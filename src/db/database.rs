use crate::errors::DatalintResult;
use chrono::Utc;
use duckdb::Connection;
use std::path::Path;

use super::models::*;
use super::queries::*;
use super::schema;

/// Database manager for DuckDB operations
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new in-memory database
    pub fn new_memory() -> DatalintResult<Self> {
        let conn = Connection::open_in_memory()?;
        let mut db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Open or create a database file
    pub fn open(path: &Path) -> DatalintResult<Self> {
        let conn = Connection::open(path)?;
        let mut db = Self { conn };

        // Initialize schema if tables don't exist
        if !db.tables_exist()? {
            db.init_schema()?;
        }

        Ok(db)
    }

    /// Initialize database schema
    pub fn init_schema(&mut self) -> DatalintResult<()> {
        self.conn.execute_batch(schema::SCHEMA)?;
        Ok(())
    }

    /// Drop all tables (useful for testing)
    pub fn drop_tables(&mut self) -> DatalintResult<()> {
        self.conn.execute_batch(schema::DROP_TABLES)?;
        Ok(())
    }

    /// Check if tables exist
    fn tables_exist(&self) -> DatalintResult<bool> {
        let query =
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = 'cache_metadata'";
        let count: i32 = self.conn.query_row(query, [], |row| row.get(0))?;
        Ok(count > 0)
    }

    /// Initialize cache metadata
    pub fn init_cache_metadata(
        &mut self,
        dataset_path: &str,
        dataset_type: &str,
        dataset_task: &str,
        version: &str,
    ) -> DatalintResult<()> {
        let now = Utc::now().to_rfc3339();

        let query = r#"
            INSERT INTO cache_metadata
            (id, created_at, updated_at, datalint_version, dataset_path, dataset_type, dataset_task)
            VALUES (1, ?, ?, ?, ?, ?, ?)
        "#;

        self.conn.execute(
            query,
            duckdb::params![
                &now,
                &now,
                version,
                dataset_path,
                dataset_type,
                dataset_task
            ],
        )?;

        Ok(())
    }

    /// Get cache metadata
    pub fn get_cache_metadata(&self) -> DatalintResult<Option<CacheMetadata>> {
        let query = r#"
            SELECT id, created_at, updated_at, datalint_version, dataset_path,
                   dataset_type, dataset_task, keypoint_names, keypoint_skeleton
            FROM cache_metadata WHERE id = 1
        "#;

        let result = self.conn.query_row(query, [], |row| {
            Ok(CacheMetadata {
                id: row.get(0)?,
                created_at: row.get(1)?,
                updated_at: row.get(2)?,
                datalint_version: row.get(3)?,
                dataset_path: row.get(4)?,
                dataset_type: row.get(5)?,
                dataset_task: row.get(6)?,
                keypoint_names: row.get(7)?,
                keypoint_skeleton: row.get(8)?,
            })
        });

        match result {
            Ok(metadata) => Ok(Some(metadata)),
            Err(duckdb::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Update cache metadata timestamp
    pub fn touch_cache_metadata(&mut self) -> DatalintResult<()> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "UPDATE cache_metadata SET updated_at = ? WHERE id = 1",
            duckdb::params![&now],
        )?;

        Ok(())
    }

    /// Get a reference to the connection for direct queries
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Begin a transaction
    pub fn transaction(&mut self) -> DatalintResult<duckdb::Transaction<'_>> {
        Ok(self.conn.transaction()?)
    }

    /// Execute a batch insert for better performance
    pub fn batch_insert_images(&mut self, images: &[Image]) -> DatalintResult<Vec<i64>> {
        let tx = self.transaction()?;
        let mut ids = Vec::with_capacity(images.len());

        for image in images {
            let id = ImageQueries::insert(&tx, image)?;
            ids.push(id);
        }

        tx.commit()?;

        Ok(ids)
    }

    /// Batch insert bboxes
    pub fn batch_insert_bboxes(&mut self, bboxes: &mut [Bbox]) -> DatalintResult<Vec<i64>> {
        let tx = self.transaction()?;
        let mut ids = Vec::with_capacity(bboxes.len());

        for bbox in bboxes {
            let id = BboxQueries::insert(&tx, bbox)?;
            ids.push(id);
        }

        tx.commit()?;

        Ok(ids)
    }

    /// Delete an image and cascade to related records
    pub fn delete_image(&mut self, image_id: i32) -> DatalintResult<()> {
        let tx = self.transaction()?;

        // Delete in cascade order (children first)
        tx.execute(
            "DELETE FROM keypoints WHERE bbox_id IN (SELECT id FROM bboxes WHERE image_id = ?)",
            duckdb::params![image_id],
        )?;
        tx.execute(
            "DELETE FROM segmentations WHERE bbox_id IN (SELECT id FROM bboxes WHERE image_id = ?)",
            duckdb::params![image_id],
        )?;
        tx.execute(
            "DELETE FROM bboxes WHERE image_id = ?",
            duckdb::params![image_id],
        )?;
        tx.execute(
            "DELETE FROM classifications WHERE image_id = ?",
            duckdb::params![image_id],
        )?;
        tx.execute("DELETE FROM images WHERE id = ?", duckdb::params![image_id])?;

        tx.commit()?;
        Ok(())
    }
}
