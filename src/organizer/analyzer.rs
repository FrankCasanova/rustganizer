use crate::config::Config;
use crate::organizer::types::FileStats;
use std::path::Path;
use walkdir::WalkDir;

pub fn analyze_folder(path: &Path, config: &Config) -> FileStats {
    let mut stats = FileStats {
        music: 0,
        videos: 0,
        images: 0,
        docs: 0,
    };
    let file_extensions = config.get_file_extensions();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                let ext = extension.to_str().unwrap().to_lowercase();
                if file_extensions.music.contains(&ext.as_str()) {
                    stats.music += 1;
                } else if file_extensions.videos.contains(&ext.as_str()) {
                    stats.videos += 1;
                } else if file_extensions.images.contains(&ext.as_str()) {
                    stats.images += 1;
                } else if file_extensions.docs.contains(&ext.as_str()) {
                    stats.docs += 1;
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
