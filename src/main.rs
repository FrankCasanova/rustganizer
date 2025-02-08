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
                
                let download_dir = format!("C:/Users/{}/Downloads", username);
                let music_dir = format!("C:/Users/{}/Music", username);
                let videos_dir = format!("C:/Users/{}/Videos", username);
                let images_dir = format!("C:/Users/{}/Pictures", username);
                let pdf_files_dir = format!("C:/Users/{}/Documents", username);
                
                let (music_count, video_count, images_count, pdf_count, ) = organize_files(&download_dir, &music_dir, &videos_dir, &images_dir, &pdf_files_dir );
                
                let info_message = format!(
                    "{} music files were moved to Music directory\n{} video files were moved to Videos directory\n{} image files were moved to Pictures directory\n{} pdf files were moved to Documents directory",
                    music_count, video_count,images_count, pdf_count
                );
                
                s.add_layer(Dialog::info(info_message));
            }),
    );

    siv.run();
}

fn organize_files(download_dir: &str, music_dir: &str, videos_dir: &str, images_dir: &str, pdf_files_dir: &str) -> (usize, usize, usize, usize) {
    let mut music_count = 0;
    let mut video_count = 0;
    let mut images_count = 0;
    let mut pdf_count = 0;


    for entry in WalkDir::new(download_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let (target_dir, count) = match extension.to_str().unwrap().to_lowercase().as_str() {
                    "mp3" | "ogg" | "wav" | "flac" => (music_dir, &mut music_count),
                    "mp4" | "avi" | "mkv" | "mov" => (videos_dir, &mut video_count),
                    "png" | "jpg" | "jpeg" | "gif" => (images_dir, &mut images_count),
                    "pdf" => (pdf_files_dir, &mut pdf_count),
                    _ => continue,
                };

                let file_name = path.file_name().unwrap();
                let target_path = Path::new(target_dir).join(file_name);

                if let Err(e) = fs::rename(path, &target_path) {
                    eprintln!("Error moving file {:?}: {}", path, e);
                } else {
                    *count += 1;
                }
            }
        }
    }

    (music_count, video_count, images_count, pdf_count)
}

