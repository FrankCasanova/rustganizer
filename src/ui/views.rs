use crate::organizer::mover::organize_files;
#[cfg(target_os = "macos")]
use crate::platform::user::MacUserProvider;
#[cfg(all(unix, not(target_os = "macos")))]
use crate::platform::user::UnixUserProvider;
use crate::platform::user::UserProvider;
#[cfg(target_os = "windows")]
use crate::platform::user::WindowsUserProvider;
use cursive::traits::*;
use cursive::views::{Dialog, TextView};

pub fn run_ui() {
    #[cfg(target_os = "windows")]
    let user_provider = WindowsUserProvider;
    #[cfg(target_os = "macos")]
    let user_provider = MacUserProvider;
    #[cfg(all(unix, not(target_os = "macos")))]
    let user_provider = UnixUserProvider;

    let username = user_provider
        .current_user()
        .unwrap_or_else(|| "unknown".to_string());
    let username_display = username.clone();

    let mut siv = cursive::default();

    siv.add_global_callback(cursive::event::Key::Esc, |s| s.quit());
    siv.add_layer(
        Dialog::new()
            .title("RustGanizer")
            .content(
                TextView::new(format!(
                    "Welcome, {username_display}!\n\nThis will organize your Downloads and Desktop files\ninto Music, Videos, Pictures, and Documents folders.\n"
                ))
                .with_name("welcome"),
            )
            .button("ORGANIZE!", move |s| {
                let username = username.clone();
                let processing_dialog = Dialog::new()
                    .title("Organizing Files...")
                    .content(TextView::new("Please wait, organizing your files..."));
                s.add_layer(processing_dialog);
                let cb_sink = s.cb_sink().clone();
                let config = crate::config::Config::default();
                std::thread::spawn(move || {
                    match organize_files(&username, "en", &config) {
                        Ok(stats) => {
                            let info_message = format!(
                                "Organization Complete!\n\nMusic files/folders moved: {}\nVideo files/folders moved: {}\nImage files/folders moved: {}\nDocs files/folders moved: {}",
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
                                    s.add_layer(Dialog::info(format!("Error: {e}")));
                                }))
                                .unwrap();
                        }
                    }
                });
            })
            .button("Esc", |s| s.quit()),
    );
    siv.run();
}
