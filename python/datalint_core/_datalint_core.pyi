from typing import Literal

__version__: str

DatasetFormat = Literal[
    "YOLO_OBJECT_DETECTION",
    "YOLO_SEGMENTATION",
    "YOLO_OBB",
    "COCO_JSON",
    "PASCAL_VOC_XML",
]

def get_dataset_format(path: str) -> DatasetFormat: ...
def validate_dataset_format(path: str, format: DatasetFormat) -> None: ...
