use crate::db::models::{Bbox, Keypoint, Segmentation};
use crate::errors::DatalintResult;
use duckdb::{params, Connection};
use serde_json;

pub struct BboxQueries;

impl BboxQueries {
    const INSERT: &'static str = r#"
        INSERT INTO bboxes (image_id, label_id, x1, y1, x2, y2, cx, cy, w, h, area, angle, confidence)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
    "#;

    const SELECT_BY_IMAGE: &'static str = r#"
        SELECT id, image_id, label_id, x1, y1, x2, y2, cx, cy, w, h, area, angle, confidence
        FROM bboxes WHERE image_id = ?
    "#;

    const COUNT_BY_LABEL: &'static str = r#"
        SELECT label_id, COUNT(*) as count
        FROM bboxes
        GROUP BY label_id
    "#;

    const INSERT_SEGMENTATION: &'static str = r#"
        INSERT INTO segmentations (bbox_id, vertices, vertex_count)
        VALUES (?, ?, ?)
        RETURNING id
    "#;

    const INSERT_KEYPOINT: &'static str = r#"
        INSERT INTO keypoints (bbox_id, points, point_count, has_visibility)
        VALUES (?, ?, ?, ?)
        RETURNING id
    "#;

    /// Insert a bounding box (computes derived values)
    pub fn insert(conn: &Connection, bbox: &mut Bbox) -> DatalintResult<i64> {
        bbox.compute_derived();

        conn.query_row(
            Self::INSERT,
            params![
                bbox.image_id,
                bbox.label_id,
                bbox.x1,
                bbox.y1,
                bbox.x2,
                bbox.y2,
                bbox.cx.unwrap(),
                bbox.cy.unwrap(),
                bbox.w.unwrap(),
                bbox.h.unwrap(),
                bbox.area.unwrap(),
                bbox.angle.unwrap_or(0.0),
                bbox.confidence,
            ],
            |row| row.get(0),
        )
        .map_err(Into::into)
    }

    /// Insert segmentation for a bbox
    pub fn insert_segmentation(conn: &Connection, seg: &Segmentation) -> DatalintResult<i64> {
        let vertices_json = serde_json::to_string(&seg.vertices)
            .map_err(|e| crate::errors::DatalintError::Generic(e.to_string()))?;

        conn.query_row(
            Self::INSERT_SEGMENTATION,
            params![seg.bbox_id, vertices_json, seg.vertex_count],
            |row| row.get(0),
        )
        .map_err(Into::into)
    }

    /// Insert keypoints for a bbox
    pub fn insert_keypoint(conn: &Connection, kp: &Keypoint) -> DatalintResult<i64> {
        let points_json = serde_json::to_string(&kp.points)
            .map_err(|e| crate::errors::DatalintError::Generic(e.to_string()))?;

        conn.query_row(
            Self::INSERT_KEYPOINT,
            params![
                kp.bbox_id,
                points_json,
                kp.point_count,
                kp.has_visibility as i32
            ],
            |row| row.get(0),
        )
        .map_err(Into::into)
    }

    /// Get bboxes for an image
    pub fn get_by_image(conn: &Connection, image_id: i32) -> DatalintResult<Vec<Bbox>> {
        let mut stmt = conn.prepare(Self::SELECT_BY_IMAGE)?;

        let results = stmt.query_map(params![image_id], |row| {
            Ok(Bbox {
                id: Some(row.get(0)?),
                image_id: row.get(1)?,
                label_id: row.get(2)?,
                x1: row.get(3)?,
                y1: row.get(4)?,
                x2: row.get(5)?,
                y2: row.get(6)?,
                cx: Some(row.get(7)?),
                cy: Some(row.get(8)?),
                w: Some(row.get(9)?),
                h: Some(row.get(10)?),
                area: Some(row.get(11)?),
                angle: row.get(12)?,
                confidence: row.get(13)?,
            })
        })?;

        let mut vec = Vec::new();
        for result in results {
            vec.push(result?);
        }
        Ok(vec)
    }

    /// Count bboxes by label
    pub fn count_by_label(conn: &Connection) -> DatalintResult<Vec<(i32, i32)>> {
        let mut stmt = conn.prepare(Self::COUNT_BY_LABEL)?;

        let results = stmt.query_map(params![], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?))
        })?;

        let mut vec = Vec::new();
        for result in results {
            vec.push(result?);
        }
        Ok(vec)
    }
}
