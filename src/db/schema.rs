/// Database schema definitions for DuckDB
pub const SCHEMA: &str = include_str!("../../migrations/001_initial.sql");

/// Drop all tables (useful for testing/resetting)
pub const DROP_TABLES: &str = r#"
    DROP TABLE IF EXISTS classifications;
    DROP TABLE IF EXISTS keypoints;
    DROP TABLE IF EXISTS segmentations;
    DROP TABLE IF EXISTS bboxes;
    DROP TABLE IF EXISTS images;
    DROP TABLE IF EXISTS labels;
    DROP TABLE IF EXISTS cache_metadata;
    DROP SEQUENCE IF EXISTS classifications_id_seq;
    DROP SEQUENCE IF EXISTS keypoints_id_seq;
    DROP SEQUENCE IF EXISTS segmentations_id_seq;
    DROP SEQUENCE IF EXISTS bboxes_id_seq;
    DROP SEQUENCE IF EXISTS images_id_seq;
    DROP SEQUENCE IF EXISTS labels_id_seq;
"#;
