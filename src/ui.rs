use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use crate::organizer;

pub fn run_ui() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::new()
            .title("RustGanizer")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Enter your Windows username:"))
                    .child(EditView::new().with_name("username").fixed_width(50)),
            )
            .button("Organize", |s| {
                let username = s
                    .call_on_name("username", |view: &mut EditView| view.get_content())
                    .unwrap();

                // Call the organize_files function from the organizer module
                match organizer::organize_files(&username) {
                    Ok(stats) => {
                        let info_message = format!(
                            "{} music files/folders were moved to Music directory\n\
                             {} video files/folders were moved to Videos directory\n\
                             {} image files/folders were moved to Pictures directory\n\
                             {} pdf files/folders were moved to Documents directory",
                            stats.music, stats.videos, stats.images, stats.pdfs
                        );
                        s.add_layer(Dialog::info(info_message));
                    }
                    Err(e) => {
                        s.add_layer(Dialog::info(format!("Error organizing files: {}", e)));
                    }
                }
            }),
    );

    siv.run();
}