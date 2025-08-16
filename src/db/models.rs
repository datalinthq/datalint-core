use serde::{Deserialize, Serialize};

/// Cache metadata - single row configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    pub id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub datalint_version: String,
    pub dataset_path: String,
    pub dataset_type: String,
    pub dataset_task: String,
    pub keypoint_names: Option<String>,
    pub keypoint_skeleton: Option<String>,
}

/// Label information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: Option<i32>,
    pub name: String,
    pub color: Option<String>,
}

/// Image information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: Option<i32>,
    pub filename: String,
    pub relative_path: String,
    pub split: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub channels: Option<i32>,
    pub format: Option<String>,
    pub file_size: Option<i64>,
    pub file_hash: String,
}

/// Bounding box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bbox {
    pub id: Option<i32>,
    pub image_id: i32,
    pub label_id: i32,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub cx: Option<f64>,   // Computed on insert
    pub cy: Option<f64>,   // Computed on insert
    pub w: Option<f64>,    // Computed on insert
    pub h: Option<f64>,    // Computed on insert
    pub area: Option<f64>, // Computed on insert
    pub angle: Option<f64>,
    pub confidence: Option<f64>,
}

impl Bbox {
    /// Compute derived values (cx, cy, w, h, area)
    pub fn compute_derived(&mut self) {
        self.cx = Some((self.x1 + self.x2) / 2.0);
        self.cy = Some((self.y1 + self.y2) / 2.0);
        self.w = Some(self.x2 - self.x1);
        self.h = Some(self.y2 - self.y1);
        self.area = Some(self.w.unwrap() * self.h.unwrap());
    }
}

/// Segmentation vertices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segmentation {
    pub id: Option<i32>,
    pub bbox_id: i32,
    pub vertices: Vec<(f64, f64)>,
    pub vertex_count: i32,
}

/// Point structure for keypoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub visibility: Option<f64>,
}

/// Keypoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keypoint {
    pub id: Option<i32>,
    pub bbox_id: i32,
    pub points: Vec<Point>,
    pub point_count: i32,
    pub has_visibility: bool,
}

/// Classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classification {
    pub id: Option<i32>,
    pub image_id: i32,
    pub label_id: i32,
    pub confidence: Option<f64>,
}
