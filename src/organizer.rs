use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use walkdir::WalkDir;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FileStats { // Make FileStats pub if you need to access it from ui.rs
    pub music: usize, // Make fields pub if needed externally
    pub videos: usize,
    pub images: usize,
    pub pdfs: usize,
}

fn analyze_folder(path: &Path) -> FileStats {
    let mut stats = FileStats {
        music: 0,
        videos: 0,
        images: 0,
        pdfs: 0,
    };

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                match extension.to_str().unwrap().to_lowercase().as_str() {
                    "mp3" | "ogg" | "wav" | "flac" => stats.music += 1,
                    "mp4" | "avi" | "mkv" | "mov" => stats.videos += 1,
                    "png" | "jpg" | "jpeg" | "gif" => stats.images += 1,
                    "pdf" => stats.pdfs += 1,
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
        (stats.pdfs, "pdf"),
    ];

    type_counts.sort_by(|a, b| b.0.cmp(&a.0));

    if type_counts[0].0 > 0 {
        Some(type_counts[0].1)
    } else {
        None
    }
}

pub fn organize_files(username: &str) -> Result<FileStats, String> { // Modified to take username and return FileStats
    let music_count = Arc::new(Mutex::new(0));
    let video_count = Arc::new(Mutex::new(0));
    let images_count = Arc::new(Mutex::new(0));
    let pdf_count = Arc::new(Mutex::new(0));

    // Directories to organize
    let download_dir = format!("C:/Users/{}/Downloads", username);
    let desktop_dir = format!("C:/Users/{}/Desktop", username);
    let music_dir = format!("C:/Users/{}/Music", username);
    let videos_dir = format!("C:/Users/{}/Videos", username);
    let images_dir = format!("C:/Users/{}/Pictures", username);
    let pdf_files_dir = format!("C:/Users/{}/Documents", username);

    // Ensure target directories exist (moved inside organizer module)
    for dir in [&music_dir, &videos_dir, &images_dir, &pdf_files_dir, &desktop_dir] {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).unwrap_or_else(|e| {
                eprintln!("Failed to create directory {}: {}", dir, e);
            });
        }
    }


    let dirs = vec![download_dir.to_string(), desktop_dir.to_string()];
    let mut handles = vec![];

    for dir in dirs {
        let music_dir = music_dir.to_string();
        let videos_dir = videos_dir.to_string();
        let images_dir = images_dir.to_string();
        let pdf_files_dir = pdf_files_dir.to_string();
        let music_count = Arc::clone(&music_count);
        let video_count = Arc::clone(&video_count);
        let images_count = Arc::clone(&images_count);
        let pdf_count = Arc::clone(&pdf_count);

        let handle = thread::spawn(move || {
            // First pass: Collect all folders and their statistics
            let mut folders_to_process = Vec::new();
            let mut processed_paths = HashMap::new();

            // Collect all direct subfolders first
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_dir() {
                        folders_to_process.push(path);
                    } else if path.is_file() {
                        // Process files here as well (if needed, you can add file handling logic)
                    }
                }
            }

            // Process folders
            for folder_path in &folders_to_process {
                let stats = analyze_folder(folder_path);
                if let Some(majority_type) = get_majority_type(&stats) {
                    let target_dir = match majority_type {
                        "music" => &music_dir,
                        "video" => &videos_dir,
                        "image" => &images_dir,
                        "pdf" => &pdf_files_dir,
                        _ => continue,
                    };

                    if let Some(folder_name) = folder_path.file_name() {
                        let target_path = Path::new(target_dir).join(folder_name);

                        if let Err(e) = fs::rename(&folder_path, &target_path) {
                            eprintln!("Error moving folder {:?}: {}", folder_path, e);
                        } else {
                            processed_paths.insert(
                                folder_path.to_str().unwrap().to_string(),
                                target_path.to_str().unwrap().to_string(),
                            );

                            let count = match majority_type {
                                "music" => &music_count,
                                "video" => &video_count,
                                "image" => &images_count,
                                "pdf" => &pdf_count,
                                _ => continue,
                            };
                            let mut count = count.lock().unwrap();
                            *count += 1;
                        }
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| "Thread panicked".to_string())?;
    }

    let result = {
        let music_count = music_count.lock().unwrap();
        let video_count = video_count.lock().unwrap();
        let images_count = images_count.lock().unwrap();
        let pdf_count = pdf_count.lock().unwrap();

        FileStats { // Return FileStats struct
            music: *music_count,
            videos: *video_count,
            images: *images_count,
            pdfs: *pdf_count,
        }
    };

    Ok(result)
}