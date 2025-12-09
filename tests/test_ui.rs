// UI tests are limited in Rust, but we can test the logic that would be called by the UI.
#[cfg(test)]
mod tests {
    use rustganizer::config::Config;
    use rustganizer::organizer::mover::organize_files;

    #[test]
    fn test_ui_organize_files_invalid_user() {
        let username = "nonexistent_user_xyz";
        let config = Config::default();
        let result = organize_files(username, "en", &config);
        assert!(result.is_err());
    }
}
