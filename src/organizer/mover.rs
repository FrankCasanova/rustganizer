// File moving and organizing logic will go here.

use crate::organizer::analyzer::{analyze_folder, get_majority_type};
use crate::organizer::types::FileStats;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

/// Returns a mapping from logical folder names to localized folder names based on language code.
fn get_localized_dirs(lang: &str) -> HashMap<&'static str, &'static str> {
    match lang {
        "es" => HashMap::from([
            ("Downloads", "Descargas"),
            ("Desktop", "Escritorio"),
            ("Music", "Música"),
            ("Videos", "Vídeos"),
            ("Pictures", "Imágenes"),
            ("Documents", "Documentos"),
        ]),
        _ => HashMap::from([
            ("Downloads", "Downloads"),
            ("Desktop", "Desktop"),
            ("Music", "Music"),
            ("Videos", "Videos"),
            ("Pictures", "Pictures"),
            ("Documents", "Documents"),
        ]),
    }
}

/// Organizes files for a user, supporting both English and Spanish Windows folder names.
pub fn organize_files(username: &str, lang: &str) -> Result<FileStats, String> {
    let username = username.trim();
    // Dynamically determine the base user directory ("Users" or "Usuarios")
    let user_base_dirs = vec!["C:/Users", "C:/Usuarios"];
    let mut user_dir_path = None;
    for base in &user_base_dirs {
        let candidate = format!("{}/{}", base, username);
        if Path::new(&candidate).exists() {
            user_dir_path = Some(candidate);
            break;
        }
    }
    let user_dir_path = match user_dir_path {
        Some(path) => path,
        None => {
            let err_msg = match lang {
                "es" => format!("Usuario '{}' no encontrado. Por favor, ingrese un nombre de usuario de Windows válido.", username),
                _ => format!("User '{}' not found. Please enter a valid Windows username.", username),
            };
            return Err(err_msg);
        }
    };
    let dirs_map = get_localized_dirs(lang);
    let music_count = Arc::new(Mutex::new(0));
    let video_count = Arc::new(Mutex::new(0));
    let images_count = Arc::new(Mutex::new(0));
    let docs_count = Arc::new(Mutex::new(0));
    let download_dir = format!("{}/{}", user_dir_path, dirs_map["Downloads"]);
    let desktop_dir = format!("{}/{}", user_dir_path, dirs_map["Desktop"]);
    let music_dir = format!("{}/{}", user_dir_path, dirs_map["Music"]);
    let videos_dir = format!("{}/{}", user_dir_path, dirs_map["Videos"]);
    let images_dir = format!("{}/{}", user_dir_path, dirs_map["Pictures"]);
    let docs_files_dir = format!("{}/{}", user_dir_path, dirs_map["Documents"]);
    for dir in [
        &music_dir,
        &videos_dir,
        &images_dir,
        &docs_files_dir,
        &desktop_dir,
    ] {
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
        let is_desktop = dir.contains(&dirs_map["Desktop"]);
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
                        if let Ok(metadata) = fs::metadata(&path) {
                            if metadata.len() == 0 {
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
                        Some((
                            majority_type,
                            folder_path.to_string_lossy().to_string(),
                            target_path.to_string_lossy().to_string(),
                        ))
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
                        Some((
                            kind,
                            file_path.to_string_lossy().to_string(),
                            target_path.to_string_lossy().to_string(),
                        ))
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

    #[test]
    fn test_organize_files_invalid_user_en() {
        let username = "nonexistent_user_xyz";
        let result = organize_files(username, "en");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, format!("User '{}' not found. Please enter a valid Windows username.", username));
    }

    #[test]
    fn test_organize_files_invalid_user_es() {
        let username = "nonexistent_user_xyz";
        let result = organize_files(username, "es");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, format!("Usuario '{}' no encontrado. Por favor, ingrese un nombre de usuario de Windows válido.", username));
    }

    #[test]
    fn test_get_localized_dirs_en() {
        let dirs = get_localized_dirs("en");
        assert_eq!(dirs["Downloads"], "Downloads");
        assert_eq!(dirs["Desktop"], "Desktop");
        assert_eq!(dirs["Music"], "Music");
        assert_eq!(dirs["Videos"], "Videos");
        assert_eq!(dirs["Pictures"], "Pictures");
        assert_eq!(dirs["Documents"], "Documents");
    }

    #[test]
    fn test_get_localized_dirs_es() {
        let dirs = get_localized_dirs("es");
        assert_eq!(dirs["Downloads"], "Descargas");
        assert_eq!(dirs["Desktop"], "Escritorio");
        assert_eq!(dirs["Music"], "Música");
        assert_eq!(dirs["Videos"], "Vídeos");
        assert_eq!(dirs["Pictures"], "Imágenes");
        assert_eq!(dirs["Documents"], "Documentos");
    }
}
