use pyo3::prelude::*;
use std::fmt;

/// Dataset task types for computer vision
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatasetTask {
    #[pyo3(name = "OBJECT_DETECTION")]
    ObjectDetection,
    #[pyo3(name = "INSTANCE_SEGMENTATION")]
    InstanceSegmentation,
    #[pyo3(name = "SEMANTIC_SEGMENTATION")]
    SemanticSegmentation,
    #[pyo3(name = "CLASSIFICATION")]
    Classification,
    #[pyo3(name = "OBB_DETECTION")]
    ObbDetection,
    #[pyo3(name = "POSE_ESTIMATION")]
    PoseEstimation,
}

#[pymethods]
impl DatasetTask {
    #[new]
    fn new(value: &str) -> PyResult<Self> {
        Self::from_str(value).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Return the string value of the task
    fn __str__(&self) -> &str {
        self.as_str()
    }

    /// Return the string representation
    fn __repr__(&self) -> String {
        format!("DatasetTask.{}", self.python_name())
    }

    /// Get display name (uppercase)
    #[getter]
    fn display_name(&self) -> String {
        self.python_name().to_uppercase()
    }

    /// Get the value as string
    #[getter]
    fn value(&self) -> &str {
        self.as_str()
    }

    /// Get all task values as a list
    #[staticmethod]
    fn as_list() -> Vec<&'static str> {
        vec!["detect", "segment", "semantic", "classify", "obb", "pose"]
    }
}

impl DatasetTask {
    /// Parse a DatasetTask from a string
    pub fn from_str(value: &str) -> Result<Self, String> {
        match value {
            "detect" => Ok(DatasetTask::ObjectDetection),
            "segment" => Ok(DatasetTask::InstanceSegmentation),
            "semantic" => Ok(DatasetTask::SemanticSegmentation),
            "classify" => Ok(DatasetTask::Classification),
            "obb" => Ok(DatasetTask::ObbDetection),
            "pose" => Ok(DatasetTask::PoseEstimation),
            _ => Err(format!("Invalid DatasetTask value: {}", value)),
        }
    }

    /// Get string representation matching Python's value
    pub fn as_str(&self) -> &str {
        match self {
            DatasetTask::ObjectDetection => "detect",
            DatasetTask::InstanceSegmentation => "segment",
            DatasetTask::SemanticSegmentation => "semantic",
            DatasetTask::Classification => "classify",
            DatasetTask::ObbDetection => "obb",
            DatasetTask::PoseEstimation => "pose",
        }
    }

    /// Get Python enum member name
    fn python_name(&self) -> &str {
        match self {
            DatasetTask::ObjectDetection => "OBJECT_DETECTION",
            DatasetTask::InstanceSegmentation => "INSTANCE_SEGMENTATION",
            DatasetTask::SemanticSegmentation => "SEMANTIC_SEGMENTATION",
            DatasetTask::Classification => "CLASSIFICATION",
            DatasetTask::ObbDetection => "OBB_DETECTION",
            DatasetTask::PoseEstimation => "POSE_ESTIMATION",
        }
    }
}

impl fmt::Display for DatasetTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Dataset format types
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatasetType {
    #[pyo3(name = "COCO_CLASSIC")]
    CocoClassic,
    #[pyo3(name = "COCO")]
    Coco,
    #[pyo3(name = "VOC")]
    Voc,
    #[pyo3(name = "YOLO")]
    Yolo,
    #[pyo3(name = "CLS")]
    Cls,
    #[pyo3(name = "CUSTOM")]
    Custom,
    #[pyo3(name = "UNKNOWN")]
    Unknown,
}

#[pymethods]
impl DatasetType {
    #[new]
    fn new(value: &str) -> PyResult<Self> {
        Self::from_str(value).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Return the string value of the type
    fn __str__(&self) -> &str {
        self.as_str()
    }

    /// Return the string representation
    fn __repr__(&self) -> String {
        format!("DatasetType.{}", self.python_name())
    }

    /// Get display name (uppercase)
    #[getter]
    fn display_name(&self) -> String {
        self.python_name().to_uppercase()
    }

    /// Get the value as string
    #[getter]
    fn value(&self) -> &str {
        self.as_str()
    }

    /// Get all type values as a list
    #[staticmethod]
    fn as_list() -> Vec<&'static str> {
        vec![
            "coco_classic",
            "coco",
            "voc",
            "yolo",
            "cls",
            "custom",
            "unknown",
        ]
    }
}

impl DatasetType {
    /// Parse a DatasetType from a string
    pub fn from_str(value: &str) -> Result<Self, String> {
        match value {
            "coco_classic" => Ok(DatasetType::CocoClassic),
            "coco" => Ok(DatasetType::Coco),
            "voc" => Ok(DatasetType::Voc),
            "yolo" => Ok(DatasetType::Yolo),
            "cls" => Ok(DatasetType::Cls),
            "custom" => Ok(DatasetType::Custom),
            "unknown" => Ok(DatasetType::Unknown),
            _ => Err(format!("Invalid DatasetType value: {}", value)),
        }
    }

    /// Get string representation matching Python's value
    pub fn as_str(&self) -> &str {
        match self {
            DatasetType::CocoClassic => "coco_classic",
            DatasetType::Coco => "coco",
            DatasetType::Voc => "voc",
            DatasetType::Yolo => "yolo",
            DatasetType::Cls => "cls",
            DatasetType::Custom => "custom",
            DatasetType::Unknown => "unknown",
        }
    }

    /// Get Python enum member name
    fn python_name(&self) -> &str {
        match self {
            DatasetType::CocoClassic => "COCO_CLASSIC",
            DatasetType::Coco => "COCO",
            DatasetType::Voc => "VOC",
            DatasetType::Yolo => "YOLO",
            DatasetType::Cls => "CLS",
            DatasetType::Custom => "CUSTOM",
            DatasetType::Unknown => "UNKNOWN",
        }
    }
}

impl fmt::Display for DatasetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
