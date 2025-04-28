<<<<<<< HEAD
use crate::ui::views::run_ui;
mod organizer;
mod ui; // Declare the ui module
fn main() {
    run_ui(); // Call the function to run the UI from the ui module
=======
mod organizer;
mod ui; // Declare the ui module
fn main() {
    ui::run_ui(); // Call the function to run the UI from the ui module
>>>>>>> 1cbb1d9dbf7304f1dcb1435bb36af4a8bb76ea75
}
