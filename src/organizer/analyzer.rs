// Analysis logic for files and folders will go here.

use crate::organizer::types::FileStats;
use std::path::Path;
use walkdir::WalkDir;

pub fn analyze_folder(path: &Path) -> FileStats {
    let mut stats = FileStats {
        music: 0,
        videos: 0,
        images: 0,
        docs: 0,
    };
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                match extension.to_str().unwrap().to_lowercase().as_str() {
                    "mp3" | "ogg" | "wav" | "flac" => stats.music += 1,
                    "mp4" | "avi" | "mkv" | "mov" => stats.videos += 1,
                    "png" | "jpg" | "jpeg" | "gif" => stats.images += 1,
                    "pdf" | "txt" | "epub" => stats.docs += 1,
                    _ => (),
                }
            }
        }
    }
    stats
}

pub fn get_majority_type(stats: &FileStats) -> Option<&'static str> {
    let mut type_counts = [
        (stats.music, "music"),
        (stats.videos, "video"),
        (stats.images, "image"),
        (stats.docs, "docs"),
    ];
    type_counts.sort_by(|a, b| b.0.cmp(&a.0));
    if type_counts[0].0 > 0 {
        Some(type_counts[0].1)
    } else {
        None
    }
}
