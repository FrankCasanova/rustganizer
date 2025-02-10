use std::sync::{Arc, Mutex};
use std::thread;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::new()
            .title("RustGanizer")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Enter your Windows username:"))
                    .child(EditView::new().with_name("username").fixed_width(50))
            )
            .button("Organize", |s| {
                let username = s.call_on_name("username", |view: &mut EditView| view.get_content()).unwrap();
                
                // Directories to organize
                let download_dir = format!("C:/Users/{}/Downloads", username);
                let desktop_dir = format!("C:/Users/{}/Desktop", username);
                let music_dir = format!("C:/Users/{}/Music", username);
                let videos_dir = format!("C:/Users/{}/Videos", username);
                let images_dir = format!("C:/Users/{}/Pictures", username);
                let pdf_files_dir = format!("C:/Users/{}/Documents", username);

                // Ensure target directories exist
                if !Path::new(&music_dir).exists() {
                    fs::create_dir_all(&music_dir).unwrap_or_else(|e| {
                        eprintln!("Failed to create Music directory: {}", e);
                    });
                }
                if !Path::new(&videos_dir).exists() {
                    fs::create_dir_all(&videos_dir).unwrap_or_else(|e| {
                        eprintln!("Failed to create Videos directory: {}", e);
                    });
                }
                if !Path::new(&images_dir).exists() {
                    fs::create_dir_all(&images_dir).unwrap_or_else(|e| {
                        eprintln!("Failed to create Pictures directory: {}", e);
                    });
                }
                if !Path::new(&pdf_files_dir).exists() {
                    fs::create_dir_all(&pdf_files_dir).unwrap_or_else(|e| {
                        eprintln!("Failed to create Documents directory: {}", e);
                    });
                }
                if !Path::new(&desktop_dir).exists() {
                    fs::create_dir_all(&desktop_dir).unwrap_or_else(|e| {
                        eprintln!("Failed to create Desktop directory: {}", e);
                    });
                }

                let result = organize_files(
                    &download_dir,
                    &music_dir,
                    &videos_dir,
                    &images_dir,
                    &pdf_files_dir,
                    &desktop_dir
                );

                match result {
                    Ok((music_count, video_count, images_count, pdf_count)) => {
                        let info_message = format!(
                            "{} music files were moved to Music directory\n{} video files were moved to Videos directory\n{} image files were moved to Pictures directory\n{} pdf files were moved to Documents directory",
                            music_count, video_count, images_count, pdf_count
                        );
                        
                        s.add_layer(Dialog::info(info_message));
                    },
                    Err(e) => {
                        s.add_layer(Dialog::info(format!("Error organizing files: {}", e)));
                    }
                }
            }),
    );

    siv.run();
}


fn organize_files(
    download_dir: &str,
    music_dir: &str,
    videos_dir: &str,
    images_dir: &str,
    pdf_files_dir: &str,
    desktop_dir: &str,
) -> Result<(usize, usize, usize, usize), String> {
    let music_count = Arc::new(Mutex::new(0));
    let video_count = Arc::new(Mutex::new(0));
    let images_count = Arc::new(Mutex::new(0));
    let pdf_count = Arc::new(Mutex::new(0));

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
            for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        let (target_dir, count) = match extension.to_str().unwrap().to_lowercase().as_str() {
                            "mp3" | "ogg" | "wav" | "flac" => (&music_dir, &music_count),
                            "mp4" | "avi" | "mkv" | "mov" => (&videos_dir, &video_count),
                            "png" | "jpg" | "jpeg" | "gif" => (&images_dir, &images_count),
                            "pdf" => (&pdf_files_dir, &pdf_count),
                            _ => continue,
                        };

                        let file_name = path.file_name().unwrap();
                        let target_path = Path::new(target_dir).join(file_name);

                        if let Err(e) = fs::rename(path, &target_path) {
                            eprintln!("Error moving file {:?}: {}", path, e);
                        } else {
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
        let images_count = images_count.lock().unwrap();
        let video_count = video_count.lock().unwrap();
        let pdf_count = pdf_count.lock().unwrap();
    
        Ok((
            *music_count,
            *images_count,
            *video_count,
            *pdf_count,
        ))
    };
    
    result
}