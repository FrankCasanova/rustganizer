// File moving and organizing logic will go here.

use crate::config::Config;
use crate::organizer::analyzer::{analyze_folder, get_majority_type};
use crate::organizer::types::FileStats;
#[cfg(target_os = "macos")]
use crate::platform::user::MacUserProvider;
#[cfg(all(unix, not(target_os = "macos")))]
use crate::platform::user::UnixUserProvider;
use crate::platform::user::UserProvider;
#[cfg(target_os = "windows")]
use crate::platform::user::WindowsUserProvider;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

fn move_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        return fs::rename(src, dst);
    }
    // If dst exists, move all files and folders from src into dst
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            move_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::rename(&src_path, &dst_path)?;
        }
    }
    fs::remove_dir(src)?;
    Ok(())
}

/// Organizes files for a user, supporting both English and Spanish Windows folder names.
pub fn organize_files(username: &str, lang: &str, config: &Config) -> Result<FileStats, String> {
    let username = username.trim();
    if username.is_empty() {
        return Err(config.get_error_message(lang, "empty_username", username));
    }
    #[cfg(target_os = "windows")]
    let user_provider = WindowsUserProvider;
    #[cfg(target_os = "macos")]
    let user_provider = MacUserProvider;
    #[cfg(all(unix, not(target_os = "macos")))]
    let user_provider = UnixUserProvider;
    let user_dir_path = match user_provider.user_home(username) {
        Some(path) => path,
        None => {
            return Err(config.get_error_message(lang, "user_not_found", username));
        }
    };
    let user_dir_path = user_dir_path.to_string_lossy();
    let music_count = Arc::new(Mutex::new(0));
    let video_count = Arc::new(Mutex::new(0));
    let images_count = Arc::new(Mutex::new(0));
    let docs_count = Arc::new(Mutex::new(0));
    let download_dir = format!(
        "{}/{}",
        user_dir_path,
        config.get_localized_dir(lang, "Downloads")
    );
    let desktop_dir = format!(
        "{}/{}",
        user_dir_path,
        config.get_localized_dir(lang, "Desktop")
    );
    let music_dir = format!(
        "{}/{}",
        user_dir_path,
        config.get_localized_dir(lang, "Music")
    );
    let videos_dir = format!(
        "{}/{}",
        user_dir_path,
        config.get_localized_dir(lang, "Videos")
    );
    let images_dir = format!(
        "{}/{}",
        user_dir_path,
        config.get_localized_dir(lang, "Pictures")
    );
    let docs_files_dir = format!(
        "{}/{}",
        user_dir_path,
        config.get_localized_dir(lang, "Documents")
    );
    for dir in [
        &music_dir,
        &videos_dir,
        &images_dir,
        &docs_files_dir,
        &desktop_dir,
    ] {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).unwrap_or_else(|e| {
                eprintln!("Failed to create directory {dir}: {e}");
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
        let is_desktop = dir.contains(&config.get_localized_dir(lang, "Desktop"));
        let config = config.clone();
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
                                    eprintln!("Failed to remove empty file {path:?}: {e}");
                                }
                                continue;
                            }
                        }
                        files_to_process.push(path);
                    }
                }
            }
            for folder_path in &folders_to_process {
                let stats = analyze_folder(folder_path, &config);
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
                        if let Err(e) = move_dir_recursive(folder_path, &target_path) {
                            eprintln!("Error moving folder {folder_path:?}: {e}");
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
                    let file_extensions = config.get_file_extensions();
                    let target_dir = if file_extensions.music.contains(&ext.as_str()) {
                        &music_dir
                    } else if file_extensions.videos.contains(&ext.as_str()) {
                        &videos_dir
                    } else if file_extensions.images.contains(&ext.as_str()) {
                        &images_dir
                    } else if file_extensions.docs.contains(&ext.as_str()) {
                        &docs_files_dir
                    } else {
                        continue;
                    };
                    if let Some(file_name) = file_path.file_name() {
                        let target_path = Path::new(target_dir).join(file_name);
                        if let Err(e) = fs::rename(file_path, &target_path) {
                            eprintln!("Error moving file {file_path:?}: {e}");
                        } else {
                            processed_paths.insert(
                                file_path.to_str().unwrap().to_string(),
                                target_path.to_str().unwrap().to_string(),
                            );
                            let count = if file_extensions.music.contains(&ext.as_str()) {
                                &music_count
                            } else if file_extensions.videos.contains(&ext.as_str()) {
                                &video_count
                            } else if file_extensions.images.contains(&ext.as_str()) {
                                &images_count
                            } else if file_extensions.docs.contains(&ext.as_str()) {
                                &docs_count
                            } else {
                                continue;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organize_files_invalid_user_en() {
        let username = "nonexistent_user_xyz";
        let config = Config::default();
        let result = organize_files(username, "en", &config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err,
            format!("User {username} not found. Please enter a valid username.")
        );
    }

    #[test]
    fn test_organize_files_invalid_user_es() {
        let username = "nonexistent_user_xyz";
        let config = Config::default();
        let result = organize_files(username, "es", &config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err,
            format!(
                "Usuario {username} no encontrado. Por favor, ingrese un nombre de usuario válido."
            )
        );
    }

    #[test]
    fn test_config_localized_dirs_en() {
        let config = Config::default();
        assert_eq!(config.get_localized_dir("en", "Downloads"), "Downloads");
        assert_eq!(config.get_localized_dir("en", "Desktop"), "Desktop");
        assert_eq!(config.get_localized_dir("en", "Music"), "Music");
        assert_eq!(config.get_localized_dir("en", "Videos"), "Videos");
        assert_eq!(config.get_localized_dir("en", "Pictures"), "Pictures");
        assert_eq!(config.get_localized_dir("en", "Documents"), "Documents");
    }

    #[test]
    fn test_config_localized_dirs_es() {
        let config = Config::default();
        assert_eq!(config.get_localized_dir("es", "Downloads"), "Descargas");
        assert_eq!(config.get_localized_dir("es", "Desktop"), "Escritorio");
        assert_eq!(config.get_localized_dir("es", "Music"), "Música");
        assert_eq!(config.get_localized_dir("es", "Videos"), "Vídeos");
        assert_eq!(config.get_localized_dir("es", "Pictures"), "Imágenes");
        assert_eq!(config.get_localized_dir("es", "Documents"), "Documentos");
    }
}
