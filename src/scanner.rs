use crate::db::models::Image;
use crate::db::queries::ImageQueries;
use crate::errors::{DatalintError, DatalintResult};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use xxhash_rust::xxh3::xxh3_64;

/// Supported image extensions
const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "bmp", "gif", "webp", "tiff", "tif", "ico", "svg",
];

/// Check if a path has an image extension
fn is_image_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Process a single image file
fn process_image(path: &Path, dataset_root: &Path) -> DatalintResult<Image> {
    // Get relative path from dataset root
    let relative_path = path
        .parent()
        .and_then(|p| p.strip_prefix(dataset_root).ok())
        .unwrap_or_else(|| Path::new(""))
        .to_string_lossy()
        .to_string();

    let filename = path
        .file_name()
        .ok_or_else(|| DatalintError::Core("Invalid filename".to_string()))?
        .to_string_lossy()
        .to_string();

    // Extract basename without extension
    let name = path
        .file_stem()
        .ok_or_else(|| DatalintError::Core("Invalid filename stem".to_string()))?
        .to_string_lossy()
        .to_string();

    // Get file metadata
    let metadata = fs::metadata(path)?;
    let file_size = metadata.len() as i64;

    // Hash file with xxHash (super fast)
    let file_data = fs::read(path)?;
    let hash = format!("{:016x}", xxh3_64(&file_data));

    // Try to decode image for dimensions and format
    let (width, height, channels, format, is_corrupted) = match image::load_from_memory(&file_data)
    {
        Ok(img) => {
            let w = img.width() as i32;
            let h = img.height() as i32;
            let c = match img.color() {
                image::ColorType::L8 | image::ColorType::La8 => 1,
                image::ColorType::Rgb8 | image::ColorType::Rgb16 | image::ColorType::Rgb32F => 3,
                image::ColorType::Rgba8 | image::ColorType::Rgba16 | image::ColorType::Rgba32F => 4,
                _ => 3, // Default to RGB
            };
            let fmt = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| "unknown".to_string());

            (Some(w), Some(h), Some(c), Some(fmt), false)
        }
        Err(_) => {
            // Image is corrupted or unsupported
            let fmt = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_lowercase());

            (None, None, None, fmt, true)
        }
    };

    // Infer split from path (train/val/test)
    let split = {
        let path_lower = relative_path.to_lowercase();
        let mut matches = Vec::new();

        if path_lower.contains("train") {
            matches.push("train");
        }
        if path_lower.contains("val") {
            matches.push("val");
        }
        if path_lower.contains("test") {
            matches.push("test");
        }

        match matches.len() {
            1 => Some(matches[0].to_string()),
            _ => Some("unknown".to_string()),
        }
    };

    Ok(Image {
        id: None,
        name,
        filename,
        extension: format,
        relative_path,
        split,
        width,
        height,
        channels,
        file_size: Some(file_size),
        file_hash: hash,
        is_corrupted,
    })
}

/// Scan a directory for all images
pub fn scan_images(dataset_path: &Path) -> DatalintResult<Vec<Image>> {
    if !dataset_path.exists() {
        return Err(DatalintError::Core(format!(
            "Dataset path does not exist: {}",
            dataset_path.display()
        )));
    }

    // Collect all image paths first
    let image_paths: Vec<PathBuf> = WalkDir::new(dataset_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| is_image_file(path))
        .collect();

    if image_paths.is_empty() {
        return Ok(Vec::new());
    }

    // Process images in parallel using rayon
    let images: Vec<Image> = image_paths
        .par_iter()
        .filter_map(|path| process_image(path, dataset_path).ok())
        .collect();

    Ok(images)
}

/// Batch insert images into database
pub fn insert_images_batch(
    db: &mut crate::db::Database,
    images: &[Image],
    batch_size: usize,
) -> DatalintResult<()> {
    if images.is_empty() {
        return Ok(());
    }

    let mut success_count = 0;
    let mut errors = Vec::new();

    // Process in batches for transaction efficiency
    for chunk in images.chunks(batch_size) {
        // Start a transaction for this batch
        match db.conn.transaction() {
            Ok(tx) => {
                for img in chunk {
                    // Use the existing ImageQueries::insert
                    match ImageQueries::insert(&tx, img) {
                        Ok(_) => success_count += 1,
                        Err(e) => {
                            errors.push(format!("{}/{}: {}", img.relative_path, img.filename, e));
                        }
                    }
                }

                if let Err(e) = tx.commit() {
                    eprintln!("Failed to commit batch: {}", e);
                    return Err(e.into());
                }
            }
            Err(e) => {
                eprintln!("Failed to start transaction: {}", e);
                return Err(e.into());
            }
        }
    }

    println!("Inserted {} images", success_count);

    // TODO create log erro file
    // if !errors.is_empty() {
    //     eprintln!("Failed to insert {} images:", errors.len());
    //     for (i, err) in errors.iter().take(5).enumerate() {
    //         eprintln!("  {}. {}", i + 1, err);
    //     }
    //     if errors.len() > 5 {
    //         eprintln!("  ... and {} more errors", errors.len() - 5);
    //     }
    // }

    Ok(())
}
