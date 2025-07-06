// UI construction and event handling will go here.

use crate::organizer::mover::organize_files;
#[cfg(target_os = "macos")]
use crate::platform::user::MacUserProvider;
#[cfg(all(unix, not(target_os = "macos")))]
use crate::platform::user::UnixUserProvider;
use crate::platform::user::UserProvider;
#[cfg(target_os = "windows")]
use crate::platform::user::WindowsUserProvider;
use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, SelectView, TextView};

pub fn run_ui() {
    #[cfg(target_os = "windows")]
    let user_provider = WindowsUserProvider;
    #[cfg(target_os = "macos")]
    let user_provider = MacUserProvider;
    #[cfg(all(unix, not(target_os = "macos")))]
    let user_provider = UnixUserProvider;
    let users: Vec<String> = user_provider.list_users();
    let mut siv = cursive::default();
    let mut select = SelectView::<String>::new().with_all_str(users.clone());
    select.add_item("<All Users>", "<ALL>".to_string());
    select.set_on_submit(move |s, username: &String| {
        let usernames: Vec<String> = if username == "<ALL>" {
            users.clone()
        } else {
            vec![username.clone()]
        };
        let processing_dialog = Dialog::new()
            .title("Organizing Files...")
            .content(TextView::new("Please wait, organizing your files..."));
        s.add_layer(processing_dialog);
        let cb_sink = s.cb_sink().clone();
        let usernames_clone = usernames.clone();
        std::thread::spawn(move || {
            let mut total_stats = crate::organizer::types::FileStats { music: 0, videos: 0, images: 0, docs: 0 };
            let mut errors = Vec::new();
            for user in usernames_clone {
                match organize_files(&user, "en") {
                    Ok(stats) => {
                        total_stats.music += stats.music;
                        total_stats.videos += stats.videos;
                        total_stats.images += stats.images;
                        total_stats.docs += stats.docs;
                    }
                    Err(e) => errors.push(format!("{user}: {e}")),
                }
            }
            let info_message = format!(
                "Organization Complete!\n\nMusic files/folders moved: {}\nVideo files/folders moved: {}\nImage files/folders moved: {}\nDocs files/folders moved: {}{}",
                total_stats.music, total_stats.videos, total_stats.images, total_stats.docs,
                if errors.is_empty() { "".to_string() } else { format!("\n\nErrors:\n{}", errors.join("\n")) }
            );
            cb_sink
                .send(Box::new(move |s| {
                    s.pop_layer();
                    s.add_layer(Dialog::info(info_message));
                }))
                .unwrap();
        });
    });
    // let screen_height = siv.screen_size().y;
    siv.add_global_callback(cursive::event::Key::Esc, |s| s.quit());
    siv.add_layer(
        Dialog::new()
            .title("RustGanizer")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Select the user to organize:"))
                    .child(select.with_name("user_select").fixed_width(50)),
            )
            .button("Esc", |s| s.quit())
            .with_name("RustGanizer"),
    );
    siv.run();
}
