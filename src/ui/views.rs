// UI construction and event handling will go here.

use crate::organizer::mover::organize_files;
use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, TextView, SelectView};
use std::process::Command;
// use std::fs;
use std::path::Path;

fn get_windows_users() -> Vec<String> {
    let output = Command::new("cmd")
        .args(["/C", "net user"])
        .output();
    let mut valid_users = Vec::new();
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut users = Vec::new();
        let mut in_users = false;
        for line in stdout.lines() {
            if line.contains("---") {
                in_users = !in_users;
                continue;
            }
            if in_users {
                users.extend(line.split_whitespace().map(|s| s.to_string()));
            }
        }
        // Only keep users that have a folder in C:/Users or C:/Usuarios
        let user_dirs = ["C:/Users", "C:/Usuarios"];
        for user in users {
            if user.eq_ignore_ascii_case("the") || user.eq_ignore_ascii_case("command") || user.eq_ignore_ascii_case("completed") || user.eq_ignore_ascii_case("successfully.") {
                continue;
            }
            let mut found = false;
            for base in &user_dirs {
                let path = Path::new(base).join(&user);
                if path.exists() {
                    found = true;
                    break;
                }
            }
            if found {
                valid_users.push(user);
            }
        }
    }
    valid_users
}

pub fn run_ui() {
    let users = get_windows_users();
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
                    Err(e) => errors.push(format!("{}: {}", user, e)),
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
    siv.add_layer(
        Dialog::new()
            .title("RustGanizer")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Select the Windows user to organize:"))
                    .child(select.with_name("user_select").fixed_width(50)),
            )
            .button("Close", |s| s.quit())
            .with_name("RustGanizer"),
    );
    siv.run();
}
