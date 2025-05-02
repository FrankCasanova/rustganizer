// UI construction and event handling will go here.

use crate::organizer::mover::organize_files;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};

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
                let processing_dialog = Dialog::new()
                    .title("Organizing Files...")
                    .content(TextView::new("Please wait, organizing your files..."));
                s.add_layer(processing_dialog);
                let cb_sink = s.cb_sink().clone();
                std::thread::spawn(move || match organize_files(&username, "en") {
                    Ok(stats) => {
                        let info_message = format!(
                            "Organization Complete!\n\n\
                                 Music files/folders moved: {}\n\
                                 Video files/folders moved: {}\n\
                                 Image files/folders moved: {}\n\
                                 Docs files/folders moved: {}",
                            stats.music, stats.videos, stats.images, stats.docs
                        );
                        cb_sink
                            .send(Box::new(move |s| {
                                s.pop_layer();
                                s.add_layer(Dialog::info(info_message));
                            }))
                            .unwrap();
                    }
                    Err(e) => {
                        cb_sink
                            .send(Box::new(move |s| {
                                s.pop_layer();
                                s.add_layer(Dialog::info(format!("Error organizing files: {}", e)));
                            }))
                            .unwrap();
                    }
                });
            })
            .button("Close", |s| s.quit())
            .with_name("RustGanizer"),
    );
    siv.run();
}
