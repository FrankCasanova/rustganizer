use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use rayon::prelude::*;

#[derive(Debug)]
pub struct FileStats {
    pub music: usize,
    pub videos: usize,
    pub images: usize,
    pub docs: usize,
}

fn analyze_folder(path: &Path) -> FileStats {
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

fn get_majority_type(stats: &FileStats) -> Option<&'static str> {
    let mut type_counts = vec![
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

pub fn organize_files(username: &str) -> Result<FileStats, String> {
    // Trim whitespace from username to avoid path errors
    let username = username.trim();
    let user_dir_path = format!("C:/Users/{}", username);
    if !Path::new(&user_dir_path).exists() {
        return Err(format!(
            "User '{}' not found. Please enter a valid Windows username.",
            username
        ));
    }

    // Directories to organize
    let download_dir = format!("C:/Users/{}/Downloads", username);
    let desktop_dir = format!("C:/Users/{}/Desktop", username);
    let music_dir = format!("C:/Users/{}/Music", username);
    let videos_dir = format!("C:/Users/{}/Videos", username);
    let images_dir = format!("C:/Users/{}/Pictures", username);
    let docs_files_dir = format!("C:/Users/{}/Documents", username);
    for dir in [&music_dir, &videos_dir, &images_dir, &docs_files_dir, &desktop_dir] {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).unwrap_or_else(|e| {
                eprintln!("Failed to create directory {}: {}", dir, e);
            });
        }
    }

    // Define dirs here
    let dirs = vec![download_dir.to_string(), desktop_dir.to_string()];

    // Collect all files and folders to process
    let mut all_folders = Vec::new();
    let mut all_files = Vec::new();

    for dir in &dirs {
        let is_desktop = dir.contains("/Desktop") || dir.contains("\\Desktop");
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() && !is_desktop {
                    all_folders.push(path);
                } else if path.is_file() {
                    all_files.push(path);
                }
            }
        }
    }

    // Process folders in parallel
    let folder_results: Vec<(&'static str, String, String)> = all_folders
        .par_iter()
        .filter_map(|folder_path| {
            let stats = analyze_folder(folder_path);
            if let Some(majority_type) = get_majority_type(&stats) {
                let target_dir = match majority_type {
                    "music" => &music_dir,
                    "video" => &videos_dir,
                    "image" => &images_dir,
                    "docs" => &docs_files_dir,
                    _ => return None,
                };
                if let Some(folder_name) = folder_path.file_name() {
                    let target_path = Path::new(target_dir).join(folder_name);
                    if let Err(e) = fs::rename(&folder_path, &target_path) {
                        eprintln!("Error moving folder {:?}: {}", folder_path, e);
                        None
                    } else {
                        Some((majority_type, folder_path.to_string_lossy().to_string(), target_path.to_string_lossy().to_string()))
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Process files in parallel
    let file_results: Vec<(&'static str, String, String)> = all_files
        .par_iter()
        .filter_map(|file_path| {
            if let Some(extension) = file_path.extension() {
                let ext = extension.to_str().unwrap().to_lowercase();
                let target_dir = match ext.as_str() {
                    "mp3" | "ogg" | "wav" | "flac" => &music_dir,
                    "mp4" | "avi" | "mkv" | "mov" => &videos_dir,
                    "png" | "jpg" | "jpeg" | "gif" => &images_dir,
                    "pdf" | "txt" | "epub" => &docs_files_dir,
                    _ => return None,
                };
                if let Some(file_name) = file_path.file_name() {
                    let target_path = Path::new(target_dir).join(file_name);
                    if let Err(e) = fs::rename(&file_path, &target_path) {
                        eprintln!("Error moving file {:?}: {}", file_path, e);
                        None
                    } else {
                        let kind = match ext.as_str() {
                            "mp3" | "ogg" | "wav" | "flac" => "music",
                            "mp4" | "avi" | "mkv" | "mov" => "video",
                            "png" | "jpg" | "jpeg" | "gif" => "image",
                            "pdf" | "txt" | "epub" => "docs",
                            _ => return None,
                        };
                        Some((kind, file_path.to_string_lossy().to_string(), target_path.to_string_lossy().to_string()))
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Count results
    let mut stats = FileStats {
        music: 0,
        videos: 0,
        images: 0,
        docs: 0,
    };

    for (kind, _, _) in folder_results.into_iter().chain(file_results.into_iter()) {
        match kind {
            "music" => stats.music += 1,
            "video" => stats.videos += 1,
            "image" => stats.images += 1,
            "docs" => stats.docs += 1,
            _ => {}
        }
    }

    Ok(stats)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_analyze_folder_counts_files_correctly() {
        let dir = tempdir().unwrap();
        let music_file = dir.path().join("song.mp3");
        let video_file = dir.path().join("movie.mp4");
        let image_file = dir.path().join("pic.jpg");
        let doc_file = dir.path().join("doc.pdf");
        File::create(&music_file).unwrap();
        File::create(&video_file).unwrap();
        File::create(&image_file).unwrap();
        File::create(&doc_file).unwrap();

        let stats = analyze_folder(dir.path());
        assert_eq!(stats.music, 1);
        assert_eq!(stats.videos, 1);
        assert_eq!(stats.images, 1);
        assert_eq!(stats.docs, 1);
    }

    #[test]
    fn test_get_majority_type() {
        let stats = FileStats { music: 5, videos: 2, images: 1, docs: 0 };
        assert_eq!(get_majority_type(&stats), Some("music"));
        let stats = FileStats { music: 0, videos: 0, images: 0, docs: 0 };
        assert_eq!(get_majority_type(&stats), None);
    }
}
