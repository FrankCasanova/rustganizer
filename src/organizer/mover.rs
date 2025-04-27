// File moving and organizing logic will go here.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::organizer::analyzer::{analyze_folder, get_majority_type};
use crate::organizer::types::FileStats;

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
    let music_count = Arc::new(Mutex::new(0));
    let video_count = Arc::new(Mutex::new(0));
    let images_count = Arc::new(Mutex::new(0));
    let docs_count = Arc::new(Mutex::new(0));
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
    let dirs = vec![download_dir.to_string(), desktop_dir.to_string()];
    let mut handles = vec![];
    for dir in dirs {
        let music_dir = music_dir.to_string();
        let videos_dir = videos_dir.to_string();
        let images_dir = images_dir.to_string();
        let docs_files_dir = docs_files_dir.to_string();
        let music_count = Arc::clone(&music_count);
        let video_count = Arc::clone(&video_count);
        let images_count = Arc::clone(&images_count);
        let docs_count = Arc::clone(&docs_count);
        let is_desktop = dir.contains("/Desktop") || dir.contains("\\Desktop");
        let handle = thread::spawn(move || {
            let mut folders_to_process = Vec::new();
            let mut files_to_process = Vec::new();
            let mut processed_paths = HashMap::new();
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_dir() {
                        if !is_desktop {
                            folders_to_process.push(path);
                        }
                    } else if path.is_file() {
                        // REMOVE FILES WITH 0 SIZE
                        if let Ok(metadata) = fs::metadata(&path) {
                            if metadata.len() == 0 {
                                // Remove the file and skip further processing
                                if let Err(e) = fs::remove_file(&path) {
                                    eprintln!("Failed to remove empty file {:?}: {}", path, e);
                                }
                                continue;
                            }
                        }
                        files_to_process.push(path);
                    }
                }
            }
            for folder_path in &folders_to_process {
                let stats = analyze_folder(folder_path);
                if let Some(majority_type) = get_majority_type(&stats) {
                    let target_dir = match majority_type {
                        "music" => &music_dir,
                        "video" => &videos_dir,
                        "image" => &images_dir,
                        "docs" => &docs_files_dir,
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
                                "docs" => &docs_count,
                                _ => continue,
                            };
                            let mut count = count.lock().unwrap();
                            *count += 1;
                        }
                    }
                }
            }
            for file_path in &files_to_process {
                if let Some(extension) = file_path.extension() {
                    let ext = extension.to_str().unwrap().to_lowercase();
                    let target_dir = match ext.as_str() {
                        "mp3" | "ogg" | "wav" | "flac" => &music_dir,
                        "mp4" | "avi" | "mkv" | "mov" => &videos_dir,
                        "png" | "jpg" | "jpeg" | "gif" => &images_dir,
                        "pdf" | "txt" | "epub" => &docs_files_dir,
                        _ => continue,
                    };
                    if let Some(file_name) = file_path.file_name() {
                        let target_path = Path::new(target_dir).join(file_name);
                        if let Err(e) = fs::rename(&file_path, &target_path) {
                            eprintln!("Error moving file {:?}: {}", file_path, e);
                        } else {
                            processed_paths.insert(
                                file_path.to_str().unwrap().to_string(),
                                target_path.to_str().unwrap().to_string(),
                            );
                            let count = match ext.as_str() {
                                "mp3" | "ogg" | "wav" | "flac" => &music_count,
                                "mp4" | "avi" | "mkv" | "mov" => &video_count,
                                "png" | "jpg" | "jpeg" | "gif" => &images_count,
                                "pdf" | "txt" | "epub" => &docs_count,
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
        let docs_count = docs_count.lock().unwrap();
        FileStats {
            music: *music_count,
            videos: *video_count,
            images: *images_count,
            docs: *docs_count,
        }
    };
    Ok(result)
}
