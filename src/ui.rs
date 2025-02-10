use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView}; // Import DummyView
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

                // Prepare a processing dialog
                let processing_dialog = Dialog::new()
                    .title("Organizing Files...")
                    .content(TextView::new("Please wait, organizing your files...")); // Simple processing message

                // Add the processing dialog to the screen
                s.add_layer(processing_dialog);

                // Disable the main dialog buttons to prevent multiple clicks
                // s.call_on_name("RustGanizer", |dialog: &mut Dialog| { // Assuming your main dialog has name "RustGanizer" or adjust accordingly. If not named, you might need a different approach to disable buttons.
                //     dialog.set_buttons_enabled(false);
                // });

                // Run the organization in a separate thread to keep the UI responsive
                let cb_sink = s.cb_sink().clone(); // Clone the callback sink to send commands to the Cursive runtime from the thread
                std::thread::spawn(move || {
                    match organizer::organize_files(&username) {
                        Ok(stats) => {
                            let info_message = format!(
                                "Organization Complete!\n\n\
                                 Music files/folders moved: {}\n\
                                 Video files/folders moved: {}\n\
                                 Image files/folders moved: {}\n\
                                 PDF files/folders moved: {}",
                                stats.music, stats.videos, stats.images, stats.pdfs
                            );

                            // Queue a callback to update the UI on the Cursive event thread
                            cb_sink.send(Box::new(move |s| {
                                // Remove the processing dialog
                                s.pop_layer();
                                // Re-enable the main dialog buttons (if you disabled them earlier, might not be needed in this example since we are replacing the dialog)
                                // s.call_on_name("RustGanizer", |dialog: &mut Dialog| {
                                //     dialog.set_buttons_enabled(true);
                                // });

                                // Show the results dialog
                                s.add_layer(Dialog::info(info_message));
                            })).unwrap();
                        }
                        Err(e) => {
                            // Queue a callback to update the UI on the Cursive event thread for errors
                            cb_sink.send(Box::new(move |s| {
                                // Remove the processing dialog
                                s.pop_layer();
                                // Re-enable the main dialog buttons (if you disabled them earlier)
                                // s.call_on_name("RustGanizer", |dialog: &mut Dialog| {
                                //     dialog.set_buttons_enabled(true);
                                // });
                                // Show the error dialog
                                s.add_layer(Dialog::info(format!("Error organizing files: {}", e)));
                            })).unwrap();
                        }
                    }
                });
            })
            .button("Close", |s| s.quit())
            .with_name("RustGanizer") // Give the main dialog a name for potential button disabling
    );

    siv.run();
}