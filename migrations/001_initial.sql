-- Datalint Core Database Schema
-- Single row configuration table
CREATE TABLE cache_metadata (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    datalint_version TEXT NOT NULL,
    dataset_path TEXT NOT NULL,
    dataset_type TEXT NOT NULL,
    dataset_task TEXT NOT NULL,
    keypoint_names TEXT,
    keypoint_skeleton TEXT
);

-- Create sequences for auto-incrementing IDs
CREATE SEQUENCE IF NOT EXISTS labels_id_seq START 1;
CREATE SEQUENCE IF NOT EXISTS images_id_seq START 1;
CREATE SEQUENCE IF NOT EXISTS bboxes_id_seq START 1;
CREATE SEQUENCE IF NOT EXISTS segmentations_id_seq START 1;
CREATE SEQUENCE IF NOT EXISTS keypoints_id_seq START 1;
CREATE SEQUENCE IF NOT EXISTS classifications_id_seq START 1;

CREATE TABLE labels (
    id INTEGER PRIMARY KEY DEFAULT nextval('labels_id_seq'),
    name TEXT NOT NULL UNIQUE,
    color TEXT
);

CREATE TABLE images (
    id INTEGER PRIMARY KEY DEFAULT nextval('images_id_seq'),
    name TEXT NOT NULL,
    filename TEXT NOT NULL,
    extension TEXT,
    relative_path TEXT NOT NULL,
    split TEXT NOT NULL DEFAULT 'unknown' CHECK(split IN ('train', 'val', 'test', 'unknown')),
    width INTEGER,
    height INTEGER,
    channels INTEGER,
    file_size INTEGER,
    file_hash TEXT NOT NULL,
    is_corrupted INTEGER NOT NULL DEFAULT 0 CHECK(is_corrupted IN (0, 1)),
    UNIQUE(relative_path, filename)
);

CREATE TABLE bboxes (
    id INTEGER PRIMARY KEY DEFAULT nextval('bboxes_id_seq'),
    image_id INTEGER NOT NULL REFERENCES images(id),
    label_id INTEGER NOT NULL REFERENCES labels(id),
    -- Store corners
    x1 REAL NOT NULL,
    y1 REAL NOT NULL,
    x2 REAL NOT NULL,
    y2 REAL NOT NULL,
    -- Computed values (calculated during insertion)
    cx REAL NOT NULL,  -- center x
    cy REAL NOT NULL,  -- center y
    w REAL NOT NULL,   -- width
    h REAL NOT NULL,   -- height
    area REAL NOT NULL,
    angle REAL DEFAULT 0,
    confidence REAL,
    CHECK(x1 < x2 AND y1 < y2)
);

CREATE TABLE segmentations (
    id INTEGER PRIMARY KEY DEFAULT nextval('segmentations_id_seq'),
    bbox_id INTEGER NOT NULL REFERENCES bboxes(id),
    vertices TEXT NOT NULL,
    vertex_count INTEGER NOT NULL
);

CREATE TABLE keypoints (
    id INTEGER PRIMARY KEY DEFAULT nextval('keypoints_id_seq'),
    bbox_id INTEGER NOT NULL REFERENCES bboxes(id),
    points TEXT NOT NULL,
    point_count INTEGER NOT NULL,
    has_visibility INTEGER NOT NULL CHECK(has_visibility IN (0, 1))
);

CREATE TABLE classifications (
    id INTEGER PRIMARY KEY DEFAULT nextval('classifications_id_seq'),
    image_id INTEGER NOT NULL REFERENCES images(id),
    label_id INTEGER NOT NULL REFERENCES labels(id),
    confidence REAL
);

-- Indexes for performance
CREATE INDEX idx_images_hash ON images(file_hash);
CREATE INDEX idx_images_name ON images(name);
CREATE INDEX idx_bboxes_image ON bboxes(image_id);
CREATE INDEX idx_bboxes_label ON bboxes(label_id);
CREATE INDEX idx_bboxes_spatial ON bboxes(x1, y1, x2, y2);
